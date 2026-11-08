//! 沙盘据点接口。
//!
//! 对应 OpenAPI 中的 `/sand/records` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::sand::ServerSand;

/// 沙盘据点接口。
///
/// 通过 [`Client::sand`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let sand = client.sand().records("长安城").await?;
/// println!("{} 个据点", sand.data.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SandApi {
    inner: Arc<Inner>,
}

impl SandApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 沙盘据点（VIP1）。
    ///
    /// 返回各据点归属帮会 / 阵营以及本周被献祭（防守）次数。
    ///
    /// # 参数
    ///
    /// * `server` —— 服务器名称，必填，例如 `长安城`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1 及以上）或服务端返回错误码
    /// （例如服务器未收录）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// let sand = client.sand().records("长安城").await?;
    /// for record in &sand.data {
    ///     println!("{} 归属 {}", record.castle_name, record.tong_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, server: impl AsRef<str>) -> Result<ServerSand> {
        let query = vec![("server", server.as_ref().to_owned())];
        self.inner.get("/sand/records", &query).await
    }
}

impl Client {
    /// 沙盘据点接口。
    pub fn sand(&self) -> SandApi {
        SandApi::new(self.inner())
    }
}
