//! 科举答题数据模型。

use serde::{Deserialize, Serialize};

/// 科举答案条目。
///
/// 对应接口：`GET /exam/search?subject=古琴&limit=10`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExamAnswer {
    /// 题库编号。
    pub id: i64,
    /// 试题。
    pub question: String,
    /// 答案。
    pub answer: String,
    /// 正确性标记（pyi 为 `int | None`，可能缺失）。
    pub correctness: Option<i64>,
    /// 结果序号。
    pub index: i64,
    /// 题目拼音首字母。
    pub pinyin: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    #[test]
    fn parses_fixture() {
        let envelope: ApiResponse<Vec<ExamAnswer>> =
            serde_json::from_str(include_str!("../../tests/fixtures/exam_search.json"))
                .expect("夹具应当能解析");
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert!(!items[0].question.is_empty());
        assert_eq!(items[0].correctness, Some(1));
    }

    #[test]
    fn correctness_may_be_null_or_missing() {
        let explicit_null: ExamAnswer =
            serde_json::from_str(r#"{"question":"题","correctness":null}"#).unwrap();
        assert!(explicit_null.correctness.is_none());
        let missing: ExamAnswer = serde_json::from_str(r#"{"question":"题"}"#).unwrap();
        assert!(missing.correctness.is_none());
    }
}
