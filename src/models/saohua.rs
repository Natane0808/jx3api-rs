//! 骚话数据模型。
//!
//! 对应接口：`/saohua/answer`、`/saohua/content`、`/saohua/context`、
//! `/saohua/drink`、`/saohua/eat`、`/saohua/random`、`/saohua/zhanan`。

use serde::{Deserialize, Serialize};

/// 答案之书。
///
/// 对应接口：`GET /saohua/answer`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaohuaAnswer {
    /// 随机答案，例如 `那不值得纠结`。
    pub answer: String,
    /// 鼓励语，例如 `🌟阳光之下,人各有份🌟`。
    pub hearten: String,
}

/// 舔狗日志。
///
/// 对应接口：`GET /saohua/content`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaohuaContent {
    /// 日志内容。
    pub text: String,
}

/// 分类语录。
///
/// 对应接口：`GET /saohua/context?name=疯狂星期四`（VIP2）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaohuaContext {
    /// 语录内容。
    pub content: String,
}

/// 随机骚话。
///
/// 对应接口：`GET /saohua/random`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaohuaRandom {
    /// 骚话内容。
    pub text: String,
}

/// 渣男语录。
///
/// 对应接口：`GET /saohua/zhanan`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaohuaZhanan {
    /// 语录内容。
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_answer_fixture() {
        let envelope: ApiResponse<SaohuaAnswer> =
            parse(include_str!("../../tests/fixtures/saohua_answer.json"));
        assert!(envelope.is_success());
        let answer = envelope.data.expect("data 不应为空");
        assert_eq!(answer.answer, "那不值得纠结");
        assert!(answer.hearten.contains("阳光之下"));
    }

    #[test]
    fn parses_content_fixture() {
        let envelope: ApiResponse<SaohuaContent> =
            parse(include_str!("../../tests/fixtures/saohua_content.json"));
        assert!(envelope.is_success());
        assert!(!envelope.data.expect("data 不应为空").text.is_empty());
    }

    #[test]
    fn parses_context_fixture() {
        let envelope: ApiResponse<SaohuaContext> =
            parse(include_str!("../../tests/fixtures/saohua_context.json"));
        assert!(envelope.is_success());
        assert!(!envelope.data.expect("data 不应为空").content.is_empty());
    }

    #[test]
    fn parses_drink_fixture() {
        let envelope: ApiResponse<Vec<String>> =
            parse(include_str!("../../tests/fixtures/saohua_drink.json"));
        assert!(envelope.is_success());
        let drinks = envelope.data.expect("data 不应为空");
        assert_eq!(drinks.len(), 4);
        assert!(drinks.iter().all(|item| !item.is_empty()));
    }

    #[test]
    fn parses_eat_fixture() {
        let envelope: ApiResponse<Vec<String>> =
            parse(include_str!("../../tests/fixtures/saohua_eat.json"));
        assert!(envelope.is_success());
        let foods = envelope.data.expect("data 不应为空");
        assert_eq!(foods.len(), 4);
        assert_eq!(foods[0], "热干面");
    }

    #[test]
    fn parses_random_fixture() {
        let envelope: ApiResponse<SaohuaRandom> =
            parse(include_str!("../../tests/fixtures/saohua_random.json"));
        assert!(envelope.is_success());
        assert!(!envelope.data.expect("data 不应为空").text.is_empty());
    }

    #[test]
    fn parses_zhanan_fixture() {
        let envelope: ApiResponse<SaohuaZhanan> =
            parse(include_str!("../../tests/fixtures/saohua_zhanan.json"));
        assert!(envelope.is_success());
        let zhanan = envelope.data.expect("data 不应为空");
        assert!(zhanan.text.contains("单身"));
    }
}
