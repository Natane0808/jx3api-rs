//! 诛恶事件接口。
//!
//! 对应 OpenAPI 中的 `/wicked/records` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::wicked::WickedRecord;

/// 诛恶事件查询参数。
///
/// 对应接口：`GET /wicked/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::wicked::WickedRecordsQuery;
/// let query = WickedRecordsQuery::default().server("长安城").limit(10);
/// println!("{query:?}");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct WickedRecordsQuery {
    /// 服务器名称，可选；不设置时查询全服。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 显示数量，默认 `20`，取值范围 `1..=100`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl WickedRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置显示数量（`1..=100`）。
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 诛恶事件接口。
///
/// 通过 [`Client::wicked`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::wicked::WickedRecordsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let query = WickedRecordsQuery::default().limit(10);
/// let records = client.wicked().records(&query).await?;
/// println!("{} 条诛恶记录", records.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct WickedApi {
    inner: Arc<Inner>,
}

impl WickedApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 诛恶（恶人）刷新记录（VIP2）。
    ///
    /// 返回恶人刷新记录，时间倒序；指定服务器时查单服，不指定时查全服。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`WickedRecordsQuery`]。`server` 可选；
    ///   `limit` 默认 `20`，取值范围 `1..=100`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码
    /// （例如 `limit` 越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::wicked::WickedRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// let query = WickedRecordsQuery::default().server("长安城").limit(10);
    /// for record in client.wicked().records(&query).await? {
    ///     println!("{} {}", record.map_name, record.time);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &WickedRecordsQuery) -> Result<Vec<WickedRecord>> {
        self.inner.get("/wicked/records", query).await
    }
}

impl Client {
    /// 诛恶事件接口。
    pub fn wicked(&self) -> WickedApi {
        WickedApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_skips_unset_fields() {
        let query = WickedRecordsQuery::default();
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.is_empty());
    }

    #[test]
    fn query_serializes_all_fields() {
        let query = WickedRecordsQuery::default().server("长安城").limit(10);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("limit=10"));
    }
}
