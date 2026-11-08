//! 关隘首领接口。
//!
//! 对应 OpenAPI 中的 `/castle/status` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::castle::CastleStatus;

/// 关隘首领接口。
///
/// 通过 [`Client::castle`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let statuses = client.castle().status().await?;
/// println!("{} 个服务器", statuses.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CastleApi {
    inner: Arc<Inner>,
}

impl CastleApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 关隘首领状态（VIP2）。
    ///
    /// 返回各服关隘首领的当前状态（可拾取 / 保护期 / 可抢占 / 每日 7-12 点统一保护），
    /// 并按服务器分组。
    ///
    /// # 参数
    ///
    /// 无参数。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// for status in client.castle().status().await? {
    ///     for record in &status.data {
    ///         println!("{} {} {}", status.server, record.castle, record.status_text);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn status(&self) -> Result<Vec<CastleStatus>> {
        self.inner
            .get("/castle/status", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 关隘首领接口。
    pub fn castle(&self) -> CastleApi {
        CastleApi::new(self.inner())
    }
}
