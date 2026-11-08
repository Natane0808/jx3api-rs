//! 角色聊天接口。
//!
//! 对应 OpenAPI 中的 `/chat/records` 路径。官方 Python SDK 未封装该接口，
//! 返回结构按真实响应夹具建模（`{ total, list }`）。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::chat::ChatRecords;

/// 角色聊天记录查询参数。
///
/// 对应接口：`GET /chat/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::chat::ChatRecordsQuery;
/// let query = ChatRecordsQuery::default()
///     .server("长安城")
///     .name("漓心")
///     .page(1);
/// println!("{query:?}");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChatRecordsQuery {
    /// 服务器名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 每页数量，默认 `20`，取值范围 `1..=50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 页码，从 `1` 开始，默认 `1`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ChatRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置每页数量（`1..=50`）。
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 设置页码（从 `1` 开始）。
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

/// 角色聊天接口。
///
/// 通过 [`Client::chat`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::chat::ChatRecordsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let query = ChatRecordsQuery::default().server("长安城").name("漓心");
/// let records = client.chat().records(&query).await?;
/// println!("共 {} 条，本页 {} 条", records.total, records.list.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ChatApi {
    inner: Arc<Inner>,
}

impl ChatApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 角色聊天记录（VIP2）。
    ///
    /// 分页查询指定服务器角色的聊天记录，按 `time` 字段降序。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`ChatRecordsQuery`]。其中 `server`、`name`
    ///   必填；`limit` 默认 `20`、取值 `1..=50`；`page` 默认 `1`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码
    /// （例如缺少 `server` / `name`、分页参数越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::chat::ChatRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// let query = ChatRecordsQuery::default()
    ///     .server("长安城")
    ///     .name("漓心")
    ///     .limit(10);
    /// for record in client.chat().records(&query).await?.list {
    ///     println!("[{}] {}", record.channel, record.message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &ChatRecordsQuery) -> Result<ChatRecords> {
        self.inner.get("/chat/records", query).await
    }
}

impl Client {
    /// 角色聊天接口。
    pub fn chat(&self) -> ChatApi {
        ChatApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_skips_unset_fields() {
        let query = ChatRecordsQuery::default().server("长安城").name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(!encoded.contains("limit="));
        assert!(!encoded.contains("page="));
    }

    #[test]
    fn query_serializes_paging_fields() {
        let query = ChatRecordsQuery::default()
            .server("长安城")
            .name("漓心")
            .limit(10)
            .page(2);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("limit=10"));
        assert!(encoded.contains("page=2"));
    }
}
