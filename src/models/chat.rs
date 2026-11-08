//! 角色聊天数据模型。
//!
//! 对应接口：`/chat/records`。官方 Python SDK 未封装该接口，模型字段按真实
//! 响应夹具（`tests/fixtures/chat_records.json`）推断。

use serde::{Deserialize, Serialize};

/// 单条聊天记录。
///
/// 对应接口：`GET /chat/records?server=长安城&name=漓心`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 角色名（响应字段为驼峰 `roleName`）。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色 id（官方以字符串返回，也兼容数字形态）。
    #[serde(
        rename = "roleId",
        deserialize_with = "crate::serde_util::flexible_string"
    )]
    pub role_id: String,
    /// 全局 id（官方以字符串返回，也兼容数字形态）。
    #[serde(
        rename = "globalId",
        deserialize_with = "crate::serde_util::flexible_string"
    )]
    pub global_id: String,
    /// 聊天频道，例如 `世界`。
    pub channel: String,
    /// 聊天内容（已包含频道与角色名前缀）。
    pub message: String,
    /// 发送时间（Unix 时间戳，秒）。
    pub time: i64,
}

/// 聊天记录分页结果。
///
/// 对应接口：`GET /chat/records`，响应 `data` 为 `{ total, list }`，
/// 而不是常见的数组。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatRecords {
    /// 符合条件的记录总数（用于分页）。
    pub total: i64,
    /// 当前页的聊天记录，按 `time` 降序。
    pub list: Vec<ChatRecord>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_fixture() {
        let envelope: ApiResponse<ChatRecords> =
            parse(include_str!("../../tests/fixtures/chat_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert_eq!(records.total, 29);
        assert!(!records.list.is_empty());
        let first = &records.list[0];
        assert_eq!(first.server, "长安城");
        assert_eq!(first.role_name, "漓心");
        assert_eq!(first.role_id, "29450102");
        assert_eq!(first.global_id, "4863887597560921611");
        assert_eq!(first.channel, "世界");
        assert!(!first.message.is_empty());
        assert!(first.time > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let records: ChatRecords = serde_json::from_str(r#"{"total":3}"#).unwrap();
        assert_eq!(records.total, 3);
        assert!(records.list.is_empty());
    }
}
