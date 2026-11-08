//! 百度贴吧相关数据模型。
//!
//! 对应接口：`/tieba/item/records`、`/tieba/random`。

use serde::{Deserialize, Serialize};

/// 物品相关的贴吧楼层记录。
///
/// 对应接口：`GET /tieba/item/records`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TiebaItemRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 吧名（贴吧名称）。
    pub name: String,
    /// 帖子编号。
    pub url: i64,
    /// 楼层内容摘要。
    pub context: String,
    /// 回复编号。
    pub reply: i64,
    /// 楼层 token（用于拼楼层链接）。
    pub token: String,
    /// 楼层号。
    pub floor: i64,
    /// 发帖时间戳（Unix 秒）。
    pub time: i64,
}

/// 随机贴吧帖子。
///
/// 对应接口：`GET /tieba/random`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TiebaRandom {
    /// 帖子编号。
    pub id: i64,
    /// 帖子类型（对应官方字段 `class`）。
    #[serde(rename = "class")]
    pub class: String,
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 吧名（贴吧名称）。
    pub name: String,
    /// 帖子标题。
    pub title: String,
    /// 帖子编号（用于拼链接）。
    pub url: i64,
    /// 发帖日期，例如 `2026-11-08`。
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_item_records_fixture() {
        let envelope: ApiResponse<Vec<TiebaItemRecord>> =
            parse(include_str!("../../tests/fixtures/tieba_item_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        let first = &records[0];
        assert_eq!(first.server, "长安城");
        assert_eq!(first.name, "剑三电一长安城");
        assert_eq!(first.url, 11155141542);
        assert_eq!(first.reply, 154110968230);
        assert_eq!(first.floor, 135);
        assert!(!first.context.is_empty());
    }

    #[test]
    fn parses_random_fixture() {
        // 夹具是「未收录」的错误响应，`data` 是 `{}`，按原始 JSON 断言包络。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/tieba_random.json"));
        assert_eq!(envelope.code, 404);
    }

    #[test]
    fn random_accepts_class_keyword() {
        let raw = r#"[{
            "id": 1,
            "class": "玩家",
            "zone": "电信区",
            "server": "长安城",
            "name": "剑三电一长安城",
            "title": "收个外观",
            "url": 11155141542,
            "date": "2026-11-08"
        }]"#;
        let list: Vec<TiebaRandom> = serde_json::from_str(raw).unwrap();
        assert_eq!(list[0].class, "玩家");
        assert_eq!(list[0].title, "收个外观");
        assert_eq!(list[0].url, 11155141542);
        assert_eq!(list[0].date, "2026-11-08");
    }
}
