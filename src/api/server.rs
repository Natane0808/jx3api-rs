//! 服务器状态接口。
//!
//! 对应 OpenAPI 中的 `/server/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::server::ServerCheckResult;

/// 服务器状态接口。
///
/// 通过 [`Client::server`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let status = client.server().status_check("长安城", None).await?;
/// println!("{status:?}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ServerApi {
    inner: Arc<Inner>,
}

impl ServerApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 大区状态（免费）。
    ///
    /// * `type=1`（默认）读取数据库中的服务器状态合集，返回**数组**；
    /// * 其它值实时拉取 xoyo 服务器列表解析在线状态：指定区服返回**单条**，
    ///   否则返回全部收录区服。
    ///
    /// 由于返回结构随 `type` 变化，这里统一返回 [`ServerCheckResult`]，
    /// 用 [`into_vec`](ServerCheckResult::into_vec) 可以抹平差异。
    ///
    /// # 参数
    ///
    /// * `server` —— 区服名称，例如 `长安城`；传空字符串表示不按区服筛选。
    /// * `r#type` —— 可选，状态类型，默认 `1`（数据库合集），其它值走 xoyo 在线状态。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let result = client.server().status_check("", None).await?;
    /// for item in result.into_vec() {
    ///     println!("{} {} 状态：{}", item.zone, item.server, item.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn status_check(
        &self,
        server: impl AsRef<str>,
        r#type: Option<i64>,
    ) -> Result<ServerCheckResult> {
        let mut query: Vec<(&str, String)> = Vec::new();
        let server = server.as_ref();
        if !server.is_empty() {
            query.push(("server", server.to_owned()));
        }
        if let Some(value) = r#type {
            query.push(("type", value.to_string()));
        }
        self.inner.get("/server/status/check", &query).await
    }
}

impl Client {
    /// 服务器状态接口。
    pub fn server(&self) -> ServerApi {
        ServerApi::new(self.inner())
    }
}
