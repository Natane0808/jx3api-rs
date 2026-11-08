//! 马场相关接口。
//!
//! 对应 OpenAPI 中的 `/ranch/*` 路径。

use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::ranch::RanchRecords;

/// 马场事件查询参数。
///
/// 对应接口：`GET /ranch/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::ranch::RanchRecordsQuery;
/// let query = RanchRecordsQuery::new().server("长安城").expired(0);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RanchRecordsQuery {
    /// 服务器名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 是否包含已过期记录，`0` 否 / `1` 是，默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<i64>,
}

impl RanchRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置是否包含已过期记录（`0` 否 / `1` 是）。
    pub fn expired(mut self, expired: i64) -> Self {
        self.expired = Some(expired);
        self
    }
}

/// 马场相关接口。
///
/// 通过 [`Client::ranch`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::ranch::RanchRecordsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = RanchRecordsQuery::new().server("长安城");
/// let records = client.ranch().records(&query).await?;
/// println!("{} {}", records.server, records.note);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RanchApi {
    inner: Arc<Inner>,
}

impl RanchApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 马场事件（VIP1）。
    ///
    /// 返回 jx3box 马场播报预告、本周期赤兔 / 的卢出世情况，并附带提示。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RanchRecordsQuery`]。
    ///   `server` 必填，`expired` 默认 `0`（`0..=1`）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::ranch::RanchRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RanchRecordsQuery::new().server("长安城").expired(1);
    /// let records = client.ranch().records(&query).await?;
    /// for (map, lines) in &records.data {
    ///     println!("{map}：{}", lines.join(" / "));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &RanchRecordsQuery) -> Result<RanchRecords> {
        self.inner.get("/ranch/records", query).await
    }

    /// 马场预告（VIP2）。
    ///
    /// 解析 jx3box 马场 NPC 播报，返回「地图名 → 下一匹马预计出世时刻」的映射。
    ///
    /// # 参数
    ///
    /// * `server` —— 服务器名称（必填）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let chat = client.ranch().chat("长安城").await?;
    /// for (map, lines) in &chat {
    ///     println!("{map}：{}", lines.join(" / "));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn chat(&self, server: impl AsRef<str>) -> Result<HashMap<String, Vec<String>>> {
        let query = vec![("server", server.as_ref().to_owned())];
        self.inner.get("/ranch/chat", &query).await
    }
}

impl Client {
    /// 马场相关接口。
    pub fn ranch(&self) -> RanchApi {
        RanchApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_query_serializes_expired() {
        let query = RanchRecordsQuery::new().server("长安城").expired(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("expired=1"));
    }

    #[test]
    fn records_query_skips_missing_expired() {
        let query = RanchRecordsQuery::new().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("expired="));
    }
}
