//! 科举答题接口。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::exam::ExamAnswer;

/// 科举答案查询参数。
///
/// 对应接口：`GET /exam/search`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::exam::ExamSearchQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = ExamSearchQuery::default().subject("古琴").limit(5);
/// let answers = client.exam().search(&query).await?;
/// println!("{} 条候选答案", answers.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ExamSearchQuery {
    /// 题目内容或拼音首字母，支持模糊匹配，例如 `古琴` 或 `GQ`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 返回条数，默认 10，取值范围 `1..=20`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ExamSearchQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置题目内容或拼音首字母。
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// 设置返回条数（`1..=20`）。
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 科举答题接口。
///
/// 通过 [`Client::exam`] 获取。
#[derive(Debug, Clone)]
pub struct ExamApi {
    inner: Arc<Inner>,
}

impl ExamApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 科举答案（免费）。
    ///
    /// 按题目或拼音模糊匹配，返回候选答案。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`ExamSearchQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如 `subject` 为空）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::exam::ExamSearchQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = ExamSearchQuery::default().subject("古琴");
    /// for answer in client.exam().search(&query).await? {
    ///     println!("{} → {}", answer.question, answer.answer);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: &ExamSearchQuery) -> Result<Vec<ExamAnswer>> {
        self.inner.get("/exam/search", query).await
    }
}

impl Client {
    /// 科举答题接口。
    pub fn exam(&self) -> ExamApi {
        ExamApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_only_set_fields() {
        let query = ExamSearchQuery::default().subject("古琴");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "subject=%E5%8F%A4%E7%90%B4");
    }

    #[test]
    fn serializes_all_fields() {
        let query = ExamSearchQuery::default().subject("GQ").limit(5);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("subject=GQ"));
        assert!(encoded.contains("limit=5"));
    }
}
