//! HTTP 客户端与构造器。

use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::Client as HttpClient;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::{Error, Result};
use crate::response::ApiResponse;

/// 默认的 API 根地址。
pub const DEFAULT_BASE_URL: &str = "https://www.jx3api.com";

/// 默认请求超时。
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// 默认的 `User-Agent`。
pub const DEFAULT_USER_AGENT: &str = concat!("jx3-api/", env!("CARGO_PKG_VERSION"));

/// JX3API 的同步门面。
///
/// 按业务域拆成若干子句柄，例如：
///
/// ```no_run
/// use jx3_api::Client;
///
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let calendar = client.active().calendar(Default::default()).await?;
/// println!("{calendar:?}");
/// # Ok(())
/// # }
/// ```
///
/// `Client` 可克隆、可跨线程共享，内部共享同一个连接池。
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

/// 客户端内部状态，由各业务域句柄共享。
#[derive(Debug)]
pub(crate) struct Inner {
    http: HttpClient,
    base_url: String,
    token: RwLock<Option<String>>,
    ticket: RwLock<Option<String>>,
}

impl Client {
    /// 用默认配置创建客户端（无 token，只能访问免费接口）。
    ///
    /// # Errors
    ///
    /// 底层 HTTP 客户端初始化失败时返回 [`Error::Http`]。
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// 用指定 token 创建客户端。
    ///
    /// # Errors
    ///
    /// 底层 HTTP 客户端初始化失败时返回 [`Error::Http`]。
    pub fn with_token(token: impl Into<String>) -> Result<Self> {
        Self::builder().token(token).build()
    }

    /// 创建构造器。
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// 内部句柄。
    pub(crate) fn inner(&self) -> Arc<Inner> {
        Arc::clone(&self.inner)
    }

    /// 当前使用的 API 根地址。
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// 当前使用的站点令牌。
    pub fn token(&self) -> Option<String> {
        self.inner.token().clone()
    }

    /// 运行时替换站点令牌。
    pub fn set_token(&self, token: Option<String>) {
        self.inner.set_token(token);
    }

    /// 当前使用的推栏 Ticket。
    pub fn ticket(&self) -> Option<String> {
        self.inner.ticket().clone()
    }

    /// 运行时替换推栏 Ticket。
    pub fn set_ticket(&self, ticket: Option<String>) {
        self.inner.set_ticket(ticket);
    }

    /// 调用任意接口并返回完整响应包络。
    ///
    /// 适用于本 SDK 尚未封装的接口：`path` 形如 `/active/calendar`。
    ///
    /// # Errors
    ///
    /// 网络异常、响应无法解析，或服务端返回非 `200` 的业务码。
    pub async fn request<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &impl Serialize,
    ) -> Result<ApiResponse<T>> {
        let value = self.inner.request_value(path, query).await?;
        let code = value
            .get("code")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        let msg = value
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let time = value
            .get("time")
            .and_then(Value::as_i64)
            .unwrap_or_default();
        let data = match value.get("data") {
            None | Some(Value::Null) => None,
            Some(raw) => Some(serde_json::from_value(raw.clone())?),
        };
        Ok(ApiResponse {
            code,
            msg,
            data,
            time,
        })
    }

    /// 调用任意接口并直接返回 `data`。
    ///
    /// # Errors
    ///
    /// 网络异常、响应无法解析，或服务端返回非 `200` 的业务码。
    pub async fn get<T: DeserializeOwned>(&self, path: &str, query: &impl Serialize) -> Result<T> {
        let value = self.inner.request_value(path, query).await?;
        let raw = value.get("data").cloned().unwrap_or(Value::Null);
        serde_json::from_value(raw).map_err(Error::from)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new().expect("默认 HTTP 客户端初始化失败")
    }
}

impl Inner {
    /// 拼接完整 URL，并把 token / ticket 注入查询串。
    fn build_url(&self, path: &str, query: &impl Serialize) -> Result<String> {
        let mut query_string =
            serde_urlencoded::to_string(query).map_err(|err| Error::Encode(err.to_string()))?;

        if let Some(token) = self.token() {
            if !query_string.contains("token=") {
                push_pair(&mut query_string, "token", &token);
            }
        }
        if let Some(ticket) = self.ticket() {
            if !query_string.contains("ticket=") {
                push_pair(&mut query_string, "ticket", &ticket);
            }
        }

        let mut url = self.build_url_plain(path);
        if !query_string.is_empty() {
            url.push('?');
            url.push_str(&query_string);
        }
        Ok(url)
    }

    /// 发起 GET 请求并把 `data` 字段解析成 `T`。
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &impl Serialize,
    ) -> Result<T> {
        let value = self.request_value(path, query).await?;
        let raw = value.get("data").cloned().unwrap_or(Value::Null);
        serde_json::from_value(raw).map_err(Error::from)
    }

    /// 发起 GET 请求并返回解析后的 JSON 根节点。
    pub(crate) async fn request_value(&self, path: &str, query: &impl Serialize) -> Result<Value> {
        let url = self.build_url(path, query)?;
        let response = self.http.get(&url).send().await?;
        self.parse_envelope(path, response).await
    }

    /// 发起 `POST` 请求（JSON 请求体）并把 `data` 字段解析成 `T`。
    ///
    /// 目前仅用于 `/token/stats`：该接口把令牌放在**请求体**里，且不走
    /// 「token 注入查询串」的通道，因此这里不复用 [`Inner::build_url`]。
    pub(crate) async fn post_json<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let response = self
            .http
            .post(self.build_url_plain(path))
            .json(body)
            .send()
            .await?;
        let value = self.parse_envelope(path, response).await?;
        let raw = value.get("data").cloned().unwrap_or(Value::Null);
        serde_json::from_value(raw).map_err(Error::from)
    }

    /// 校验 HTTP 状态与业务码，返回解析后的 JSON 根节点。
    async fn parse_envelope(&self, path: &str, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        let body = response.text().await?;

        let value: Value = match serde_json::from_str(&body) {
            Ok(value) => value,
            Err(err) => {
                if status.is_success() {
                    return Err(Error::Decode(err));
                }
                return Err(Error::HttpStatus {
                    status: status.as_u16(),
                    body,
                });
            }
        };

        let code = value
            .get("code")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        if code != 200 {
            let msg = value
                .get("msg")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned();
            return Err(Error::Api {
                code,
                msg,
                path: path.to_owned(),
            });
        }

        Ok(value)
    }

    /// 仅拼接根地址与路径（不附加任何查询参数）。
    fn build_url_plain(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// 当前令牌（供 [`crate::api::token`] 读取）。
    pub(crate) fn token(&self) -> Option<String> {
        self.token.read().ok().and_then(|guard| guard.clone())
    }

    fn set_token(&self, token: Option<String>) {
        if let Ok(mut guard) = self.token.write() {
            *guard = token;
        }
    }

    fn ticket(&self) -> Option<String> {
        self.ticket.read().ok().and_then(|guard| guard.clone())
    }

    fn set_ticket(&self, ticket: Option<String>) {
        if let Ok(mut guard) = self.ticket.write() {
            *guard = ticket;
        }
    }
}

fn push_pair(query: &mut String, key: &str, value: &str) {
    let encoded = serde_urlencoded::to_string([(key, value)]).unwrap_or_default();
    if encoded.is_empty() {
        return;
    }
    if !query.is_empty() {
        query.push('&');
    }
    query.push_str(&encoded);
}

/// [`Client`] 的构造器。
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: String,
    token: Option<String>,
    ticket: Option<String>,
    timeout: Duration,
    user_agent: String,
    http: Option<HttpClient>,
}

impl ClientBuilder {
    /// 创建构造器（全部使用默认值）。
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            token: None,
            ticket: None,
            timeout: DEFAULT_TIMEOUT,
            user_agent: DEFAULT_USER_AGENT.to_owned(),
            http: None,
        }
    }

    /// 设置 API 根地址，默认为 `https://www.jx3api.com`。
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 设置站点令牌，用于访问免费以外的接口。
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// 设置推栏 Ticket，用于访问心法、名剑等需要推栏鉴权的接口。
    pub fn ticket(mut self, ticket: impl Into<String>) -> Self {
        self.ticket = Some(ticket.into());
        self
    }

    /// 设置请求超时，默认 30 秒。
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置 `User-Agent`。
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// 使用自定义的 `reqwest` 客户端（例如配置代理、自签证书等）。
    pub fn http_client(mut self, http: HttpClient) -> Self {
        self.http = Some(http);
        self
    }

    /// 构建 [`Client`]。
    ///
    /// # Errors
    ///
    /// 底层 HTTP 客户端初始化失败时返回 [`Error::Http`]。
    pub fn build(self) -> Result<Client> {
        let http = match self.http {
            Some(http) => http,
            None => {
                let mut builder = HttpClient::builder().timeout(self.timeout);
                if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.user_agent) {
                    builder = builder.default_headers({
                        let mut headers = reqwest::header::HeaderMap::new();
                        headers.insert(reqwest::header::USER_AGENT, value);
                        headers
                    });
                }
                builder.build()?
            }
        };

        let inner = Inner {
            http,
            base_url: self.base_url,
            token: RwLock::new(self.token),
            ticket: RwLock::new(self.ticket),
        };

        Ok(Client {
            inner: Arc::new(inner),
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let client = Client::builder().build().unwrap();
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
        assert!(client.token().is_none());
        assert!(client.ticket().is_none());
    }

    #[test]
    fn token_can_be_replaced_at_runtime() {
        let client = Client::builder().token("abc").build().unwrap();
        assert_eq!(client.token().as_deref(), Some("abc"));
        client.set_token(Some("def".into()));
        assert_eq!(client.token().as_deref(), Some("def"));
        client.set_token(None);
        assert!(client.token().is_none());
    }

    #[test]
    fn url_contains_token_and_ticket() {
        let client = Client::builder()
            .token("jx3api::abc")
            .ticket("tk")
            .build()
            .unwrap();
        let inner = client.inner();
        let url = inner
            .build_url("/active/calendar", &vec![("num", "1")])
            .unwrap();
        assert!(url.starts_with("https://www.jx3api.com/active/calendar?"));
        assert!(url.contains("num=1"));
        assert!(url.contains("token=jx3api"));
        assert!(url.contains("ticket=tk"));
    }

    #[test]
    fn base_url_trailing_slash_is_stripped() {
        let client = Client::builder()
            .base_url("https://example.com/")
            .build()
            .unwrap();
        let url = client
            .inner()
            .build_url("/active/calendar", &Vec::<(String, String)>::new())
            .unwrap();
        assert_eq!(url, "https://example.com/active/calendar");
    }
}
