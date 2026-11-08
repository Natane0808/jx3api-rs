//! 掉落统计相关接口。
//!
//! 对应 OpenAPI 中的 `/reward/statistics` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::reward::RewardRecord;

/// 掉落统计查询参数。
///
/// 对应接口：`GET /reward/statistics`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::reward::RewardStatisticsQuery;
/// let query = RewardStatisticsQuery::new().name("玄晶").limit(20);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RewardStatisticsQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 物品名称（必填），支持模糊匹配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 显示数量（必填），默认 20，取值范围 `1..=100`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl RewardStatisticsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置物品名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置显示数量（`1..=100`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 掉落统计接口。
///
/// 通过 [`Client::reward`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::reward::RewardStatisticsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = RewardStatisticsQuery::new().name("玄晶").limit(20);
/// for record in client.reward().statistics(&query).await? {
///     println!("{} 掉落了 {}", record.role_name, record.item_name);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RewardApi {
    inner: Arc<Inner>,
}

impl RewardApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 掉落统计（VIP1）。
    ///
    /// 按物品名模糊匹配掉落记录，按时间倒序；指定 `server` 时查单服，否则查全服。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RewardStatisticsQuery`]。
    ///   `name`（物品名称）必填，`server` 可选，`limit` 必填
    ///   （`1..=100`，默认 20）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::reward::RewardStatisticsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RewardStatisticsQuery::new()
    ///     .server("长安城")
    ///     .name("玄晶")
    ///     .limit(50);
    ///     let records = client.reward().statistics(&query).await?;
    ///     println!("共 {} 条掉落记录", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statistics(&self, query: &RewardStatisticsQuery) -> Result<Vec<RewardRecord>> {
        self.inner.get("/reward/statistics", query).await
    }
}

impl Client {
    /// 掉落统计接口。
    pub fn reward(&self) -> RewardApi {
        RewardApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serializes_all_fields() {
        let query = RewardStatisticsQuery::new()
            .server("长安城")
            .name("玄晶")
            .limit(20);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(encoded.contains("limit=20"));
    }

    #[test]
    fn query_skips_missing_server() {
        let query = RewardStatisticsQuery::new().name("玄晶").limit(20);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("server="));
    }
}
