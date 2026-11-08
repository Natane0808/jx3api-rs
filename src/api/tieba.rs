//! 百度贴吧相关接口。
//!
//! 对应 OpenAPI 中的 `/tieba/*` 路径，两个接口均为 VIP1。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::tieba::{TiebaItemRecord, TiebaRandom};

/// 外观记录查询参数。
///
/// 对应接口：`GET /tieba/item/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::tieba::TiebaItemRecordsQuery::default()
///     .name("同人白发")
///     .limit(5);
/// let records = client.tieba().item_records(&query).await?;
/// println!("{} 条相关楼层", records.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TiebaItemRecordsQuery {
    /// 服务器名称，可选，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 物品名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 返回数量，可选，默认 `10`，取值范围 `1..=50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TiebaItemRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置物品名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置返回数量（`1..=50`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 随机贴吧帖子查询参数。
///
/// 对应接口：`GET /tieba/random`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TiebaRandomQuery {
    /// 帖子类型，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 服务器名称，可选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 返回数量，可选，默认 `10`，取值范围 `1..=50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TiebaRandomQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置帖子类型（必填）。
    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置返回数量（`1..=50`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 百度贴吧相关接口。
///
/// 通过 [`Client::tieba`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::tieba::TiebaItemRecordsQuery::default().name("同人白发");
/// for record in client.tieba().item_records(&query).await? {
///     println!("{} 楼：{}", record.floor, record.context);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TiebaApi {
    inner: Arc<Inner>,
}

impl TiebaApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 外观记录（VIP1）。
    ///
    /// 在贴吧楼层中按物品名称搜索相关帖子（正则 + 分词兜底）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TiebaItemRecordsQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::tieba::TiebaItemRecordsQuery::default()
    ///     .server("长安城")
    ///     .name("同人白发")
    ///     .limit(10);
    /// for record in client.tieba().item_records(&query).await? {
    ///     println!("{}:{} {}", record.url, record.floor, record.context);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn item_records(
        &self,
        query: &TiebaItemRecordsQuery,
    ) -> Result<Vec<TiebaItemRecord>> {
        self.inner.get("/tieba/item/records", query).await
    }

    /// 随机贴吧帖子（VIP1）。
    ///
    /// 按帖子类型随机取近 31 天的帖子，可选指定服务器。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TiebaRandomQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::tieba::TiebaRandomQuery::default()
    ///     .tags("玩家")
    ///     .limit(5);
    /// for post in client.tieba().random(&query).await? {
    ///     println!("{} {}", post.date, post.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn random(&self, query: &TiebaRandomQuery) -> Result<Vec<TiebaRandom>> {
        self.inner.get("/tieba/random", query).await
    }
}

impl Client {
    /// 百度贴吧相关接口。
    pub fn tieba(&self) -> TiebaApi {
        TiebaApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_records_query_serializes_limit() {
        let query = TiebaItemRecordsQuery::default()
            .server("长安城")
            .name("同人白发")
            .limit(10);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("limit=10"));
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
    }

    #[test]
    fn random_query_keeps_tags() {
        let query = TiebaRandomQuery::default().tags("玩家");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("tags=%E7%8E%A9%E5%AE%B6"));
        assert!(!encoded.contains("server"));
        assert!(!encoded.contains("limit"));
    }
}
