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
        let parsed = self.inner.request_parsed(path, query).await?;
        let data = match parsed.data {
            Value::Null => None,
            raw => Some(serde_json::from_value(raw)?),
        };
        Ok(ApiResponse {
            code: parsed.code,
            msg: parsed.msg,
            data,
            time: parsed.time,
        })
    }

    /// 调用任意接口并直接返回 `data`。
    ///
    /// # Errors
    ///
    /// 网络异常、响应无法解析，或服务端返回非 `200` 的业务码。
    pub async fn get<T: DeserializeOwned>(&self, path: &str, query: &impl Serialize) -> Result<T> {
        let parsed = self.inner.request_parsed(path, query).await?;
        serde_json::from_value(parsed.data).map_err(Error::from)
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
        let parsed = self.request_parsed(path, query).await?;
        serde_json::from_value(parsed.data).map_err(Error::from)
    }

    /// 发起 GET 请求并返回已解析的业务包络。
    pub(crate) async fn request_parsed(
        &self,
        path: &str,
        query: &impl Serialize,
    ) -> Result<Parsed> {
        let url = self.build_url(path, query)?;
        let response = self.http.get(&url).send().await?;
        read_parsed(path, response).await
    }

    /// 发起 `POST` 请求（JSON 请求体）并把业务数据解析成 `T`。
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
        let parsed = read_parsed(path, response).await?;
        serde_json::from_value(parsed.data).map_err(Error::from)
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

/// 已解析的业务包络。
///
/// 服务端在 2026-11-20 把响应包络从 `code` / `msg` / `data` / `time` 换成了
/// `status` / `detail`，这里把两种形态归一化后交给上层。
#[derive(Debug, Clone)]
pub(crate) struct Parsed {
    /// 业务状态码：新包络取 `status`，旧包络取 `code`；`200` 表示成功。
    pub(crate) code: i32,
    /// 业务描述：旧包络取 `msg`；新包络失败时 `detail` 是描述字符串，此时取它。
    pub(crate) msg: String,
    /// 业务数据：新包络取 `detail`（成功时是对象 / 数组），旧包络取 `data`。
    pub(crate) data: Value,
    /// 服务端时间戳（秒）。新包络不再返回该字段，此时为 `0`。
    pub(crate) time: i64,
}

/// 读取响应并解析包络。
async fn read_parsed(path: &str, response: reqwest::Response) -> Result<Parsed> {
    let status = response.status().as_u16();
    let body = response.text().await?;
    parse_body(path, status, body)
}

/// 解析响应体，兼容新旧两种包络。
///
/// * 旧包络（2026-11 之前）：`{ "code": 200, "msg": "success", "data": …, "time": … }`
/// * 新包络（2026-11-20 起）：`{ "status": 200, "detail": … }`，其中失败时
///   `detail` 是错误描述字符串，成功时是业务数据。
///
/// 判定方式是「有 `status` 且没有 `code`」，因此两种形态可以共存，
/// 仓库里 84 份旧包络夹具也仍然可以解析。
pub(crate) fn parse_body(path: &str, http_status: u16, body: String) -> Result<Parsed> {
    let value: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(err) => {
            if (200..300).contains(&http_status) {
                return Err(Error::Decode(err));
            }
            return Err(Error::HttpStatus {
                status: http_status,
                body,
            });
        }
    };

    let is_new = value.get("status").is_some() && value.get("code").is_none();
    let code = value
        .get(if is_new { "status" } else { "code" })
        .and_then(Value::as_i64)
        .unwrap_or_default() as i32;

    let (msg, data, time) = if is_new {
        let detail = value.get("detail").cloned().unwrap_or(Value::Null);
        // 失败时 detail 是描述字符串；成功时 detail 是业务数据。
        let msg = detail.as_str().unwrap_or_default().to_owned();
        let data = if code == 200 { detail } else { Value::Null };
        (msg, data, 0)
    } else {
        let msg = value
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let data = value.get("data").cloned().unwrap_or(Value::Null);
        let time = value
            .get("time")
            .and_then(Value::as_i64)
            .unwrap_or_default();
        (msg, data, time)
    };

    if code != 200 {
        return Err(Error::Api {
            code,
            msg,
            path: path.to_owned(),
        });
    }

    Ok(Parsed {
        code,
        msg,
        data,
        time,
    })
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

                // TLS 后端由 crate 的 feature 决定。两个 feature 同时启用时
                // （例如 `--all-features`）保持 reqwest 的默认选择（native-tls），
                // 这样与 tokio-tungstenite 的优先级一致，HTTP 与 WebSocket 不会分叉。
                #[cfg(all(feature = "rustls-tls", not(feature = "native-tls")))]
                {
                    builder = builder.use_rustls_tls();
                }
                #[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
                {
                    builder = builder.use_native_tls();
                }

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
    // ---- 包络解析：兼容 2026-11-20 前后的两种响应格式 ----

    #[test]
    fn parses_legacy_envelope() {
        let body = r#"{"code":200,"msg":"success","data":{"date":"2026-11-08"},"time":1794106931}"#;
        let parsed =
            parse_body("/active/calendar", 200, body.to_owned()).expect("旧包络应解析成功");
        assert_eq!(parsed.code, 200);
        assert_eq!(parsed.msg, "success");
        assert_eq!(parsed.time, 1_794_106_931);
        assert_eq!(parsed.data["date"], "2026-11-08");
    }

    #[test]
    fn parses_new_status_detail_envelope() {
        // 2026-11-20 起站点改为 { status, detail }，且不再返回 time。
        let body = r#"{"status":200,"detail":{"date":"2026-11-20","war":"大战！英雄南山马场"}}"#;
        let parsed =
            parse_body("/active/calendar", 200, body.to_owned()).expect("新包络应解析成功");
        assert_eq!(parsed.code, 200);
        assert!(parsed.msg.is_empty(), "成功响应没有错误描述");
        assert_eq!(parsed.time, 0, "新包络不返回 time");
        assert_eq!(parsed.data["war"], "大战！英雄南山马场");
    }

    #[test]
    fn new_envelope_error_uses_detail_as_message() {
        let body = r#"{"status":404,"detail":"令牌不存在"}"#;
        match parse_body("/token/stats", 200, body.to_owned()) {
            Err(Error::Api { code, msg, path }) => {
                assert_eq!(code, 404);
                assert_eq!(msg, "令牌不存在");
                assert_eq!(path, "/token/stats");
            }
            other => panic!("应映射为 Error::Api，实际 {other:?}"),
        }
    }

    #[test]
    fn legacy_envelope_error_uses_msg() {
        let body = r#"{"code":400,"msg":"参数 [name] 不能为空","data":{}}"#;
        match parse_body("/role/detail", 200, body.to_owned()) {
            Err(Error::Api { code, msg, .. }) => {
                assert_eq!(code, 400);
                assert_eq!(msg, "参数 [name] 不能为空");
            }
            other => panic!("应映射为 Error::Api，实际 {other:?}"),
        }
    }

    #[test]
    fn non_json_body_maps_by_http_status() {
        // 非 2xx 且不是 JSON → HttpStatus
        match parse_body("/x", 502, "<html>bad gateway</html>".to_owned()) {
            Err(Error::HttpStatus { status, body }) => {
                assert_eq!(status, 502);
                assert!(body.contains("bad gateway"));
            }
            other => panic!("应映射为 Error::HttpStatus，实际 {other:?}"),
        }
        // 2xx 但不是 JSON → Decode
        match parse_body("/x", 200, "not json".to_owned()) {
            Err(Error::Decode(_)) => {}
            other => panic!("应映射为 Error::Decode，实际 {other:?}"),
        }
    }

    #[test]
    fn new_envelope_null_detail_yields_null_data() {
        let body = r#"{"status":200,"detail":null}"#;
        let parsed = parse_body("/x", 200, body.to_owned()).expect("应解析成功");
        assert_eq!(parsed.data, Value::Null);
    }
}
