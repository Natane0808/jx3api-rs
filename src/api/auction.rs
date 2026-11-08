//! 阵营拍卖接口。
//!
//! 对应 OpenAPI 中的 `/auction/records` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::auction::AuctionRecord;

/// 阵营拍卖记录查询参数。
///
/// 对应接口：`GET /auction/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::auction::AuctionRecordsQuery;
/// let query = AuctionRecordsQuery::default().server("长安城").limit(10);
/// println!("{query:?}");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct AuctionRecordsQuery {
    /// 服务器名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 物品名称，用于模糊筛选，可选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 返回条数，默认 `50`，取值范围 `1..=100`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl AuctionRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置物品名称（模糊筛选）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置返回条数（`1..=100`）。
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 阵营拍卖接口。
///
/// 通过 [`Client::auction`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::auction::AuctionRecordsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let query = AuctionRecordsQuery::default().server("长安城").limit(10);
/// let records = client.auction().records(&query).await?;
/// println!("{} 条拍卖记录", records.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct AuctionApi {
    inner: Arc<Inner>,
}

impl AuctionApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 阵营拍卖记录（VIP2）。
    ///
    /// 按大区查询角色竞拍记录，可按物品名模糊筛选，结果按时间倒序。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`AuctionRecordsQuery`]。其中 `server` 必填；
    ///   `limit` 默认 `50`，取值范围 `1..=100`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码
    /// （例如缺少 `server`、`limit` 越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::auction::AuctionRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// let query = AuctionRecordsQuery::default().server("长安城");
    /// for record in client.auction().records(&query).await? {
    ///     println!("{} 拍下 {}", record.role_name, record.item_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &AuctionRecordsQuery) -> Result<Vec<AuctionRecord>> {
        self.inner.get("/auction/records", query).await
    }
}

impl Client {
    /// 阵营拍卖接口。
    pub fn auction(&self) -> AuctionApi {
        AuctionApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_skips_unset_fields() {
        let query = AuctionRecordsQuery::default().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "server=%E9%95%BF%E5%AE%89%E5%9F%8E");
    }

    #[test]
    fn query_serializes_all_fields() {
        let query = AuctionRecordsQuery::default()
            .server("长安城")
            .name("枭鸣堕夜")
            .limit(10);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(encoded.contains("limit=10"));
    }
}
