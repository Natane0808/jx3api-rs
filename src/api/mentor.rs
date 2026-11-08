//! 师徒列表相关接口。
//!
//! 对应 OpenAPI 中的 `/mentor/search` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::mentor::MentorRecord;

/// 师徒列表查询参数。
///
/// 对应接口：`GET /mentor/search`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::mentor::MentorSearchQuery;
/// let query = MentorSearchQuery::new().server("长安城").label(1).limit(50);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct MentorSearchQuery {
    /// 师徒类型（必填），`1..=2`，默认 `1`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<i64>,
    /// 服务器大区（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 关键字，用于过滤招募留言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 显示数量，默认 50，取值范围 `1..=500`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl MentorSearchQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置师徒类型（`1..=2`）。
    pub fn label(mut self, label: i64) -> Self {
        self.label = Some(label);
        self
    }

    /// 设置服务器大区。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置关键字。
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置显示数量（`1..=500`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 师徒列表接口。
///
/// 通过 [`Client::mentor`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::mentor::MentorSearchQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = MentorSearchQuery::new().server("长安城").label(1);
/// for record in client.mentor().search(&query).await? {
///     println!("{}：{}", record.role_name, record.comment_text);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MentorApi {
    inner: Arc<Inner>,
}

impl MentorApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 师徒列表（VIP1）。
    ///
    /// 转发到远程服务获取师徒招募名单，按关键字过滤留言，超量时随机抽样。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`MentorSearchQuery`]。
    ///   `label` 必填（`1..=2`，默认 1），`server` 必填，`keyword` 可选，
    ///   `limit` 默认 50（`1..=500`）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::mentor::MentorSearchQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = MentorSearchQuery::new().server("长安城").label(2).keyword("日常");
    /// let records = client.mentor().search(&query).await?;
    /// println!("共 {} 条招募", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: &MentorSearchQuery) -> Result<Vec<MentorRecord>> {
        self.inner.get("/mentor/search", query).await
    }
}

impl Client {
    /// 师徒列表接口。
    pub fn mentor(&self) -> MentorApi {
        MentorApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serializes_all_fields() {
        let query = MentorSearchQuery::new()
            .label(1)
            .server("长安城")
            .keyword("日常")
            .limit(50);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("label=1"));
        assert!(encoded.contains("server="));
        assert!(encoded.contains("keyword="));
        assert!(encoded.contains("limit=50"));
    }

    #[test]
    fn query_skips_optional_keyword() {
        let query = MentorSearchQuery::new().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("keyword="));
    }
}
