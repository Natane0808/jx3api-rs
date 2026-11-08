//! 团队招募相关接口。
//!
//! 对应 OpenAPI 中的 `/recruit/search` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::recruit::RecruitRecord;

/// 团队招募查询参数。
///
/// 对应接口：`GET /recruit/search`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::recruit::RecruitSearchQuery;
/// let query = RecruitSearchQuery::new().server("长安城").label(1).limit(50);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RecruitSearchQuery {
    /// 服务器大区（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 服务类型（必填），`1..=3`，默认 `1`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<i64>,
    /// 关键字，用于过滤活动 / 团长 / 内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 显示数量（必填），默认 50，取值范围 `20..=500`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl RecruitSearchQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器大区。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置服务类型（`1..=3`）。
    pub fn label(mut self, label: i64) -> Self {
        self.label = Some(label);
        self
    }

    /// 设置关键字。
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置显示数量（`20..=500`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 团队招募接口。
///
/// 通过 [`Client::recruit`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::recruit::RecruitSearchQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = RecruitSearchQuery::new().server("长安城").label(1).limit(50);
/// for record in client.recruit().search(&query).await? {
///     println!("{}：{}", record.activity_name, record.comment_text);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RecruitApi {
    inner: Arc<Inner>,
}

impl RecruitApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 团队招募（VIP1）。
    ///
    /// 转发到远程服务获取招募名单，按关键字（活动 / 团长 / 内容）过滤，超量时截断。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RecruitSearchQuery`]。
    ///   `server` 必填，`label` 必填（`1..=3`，默认 1），`keyword` 可选，
    ///   `limit` 必填（`20..=500`，默认 50）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码
    /// （例如暂无招募数据时返回 `404`）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::recruit::RecruitSearchQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RecruitSearchQuery::new()
    ///     .server("长安城")
    ///     .label(2)
    ///     .keyword("教学")
    ///     .limit(20);
    /// let records = client.recruit().search(&query).await?;
    /// println!("共 {} 条招募", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: &RecruitSearchQuery) -> Result<Vec<RecruitRecord>> {
        self.inner.get("/recruit/search", query).await
    }
}

impl Client {
    /// 团队招募接口。
    pub fn recruit(&self) -> RecruitApi {
        RecruitApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serializes_all_fields() {
        let query = RecruitSearchQuery::new()
            .server("长安城")
            .label(1)
            .keyword("教学")
            .limit(50);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("label=1"));
        assert!(encoded.contains("keyword="));
        assert!(encoded.contains("limit=50"));
    }

    #[test]
    fn query_skips_optional_keyword() {
        let query = RecruitSearchQuery::new()
            .server("长安城")
            .label(3)
            .limit(20);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("keyword="));
        assert!(encoded.contains("limit=20"));
    }
}
