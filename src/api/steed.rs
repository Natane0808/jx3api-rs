//! 的卢拍卖接口。
//!
//! 对应 OpenAPI 中的 `/steed/records` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::steed::SteedRecord;

/// 的卢拍卖接口。
///
/// 通过 [`Client::steed`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let records = client.steed().records(Some("长安城")).await?;
/// println!("{} 条记录", records.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SteedApi {
    inner: Arc<Inner>,
}

impl SteedApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 的卢拍卖记录（VIP2）。
    ///
    /// 指定服务器时返回该服最新 10 条；不指定时每服返回最新 1 条
    /// （按收录服列全，无记录的服返回空）。
    ///
    /// # 参数
    ///
    /// * `server` —— 服务器名称，可选；`None` 表示查询全部收录服务器。
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
    /// for record in client.steed().records(None).await? {
    ///     println!("{} {} {}", record.server, record.map_name, record.auction_amount);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, server: Option<&str>) -> Result<Vec<SteedRecord>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(server) = server {
            query.push(("server", server.to_owned()));
        }
        self.inner.get("/steed/records", &query).await
    }
}

impl Client {
    /// 的卢拍卖接口。
    pub fn steed(&self) -> SteedApi {
        SteedApi::new(self.inner())
    }
}
