//! 马场相关数据模型。
//!
//! 对应接口：`/ranch/records`、`/ranch/chat`。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 马场事件预告。
///
/// 对应接口：`GET /ranch/records`。`data` 是「地图名 → 播报文本列表」的映射。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RanchRecords {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 各地图的马场播报，键为地图名，值为播报文本列表。
    pub data: HashMap<String, Vec<String>>,
    /// 附加提示，例如本轮赤兔出世情况。
    pub note: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<RanchRecords> =
            parse(include_str!("../../tests/fixtures/ranch_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert_eq!(records.zone, "电信区");
        assert_eq!(records.server, "长安城");
        assert!(!records.data.is_empty());
        assert!(!records.note.is_empty());
        assert!(records.data.contains_key("阴山大草原"));
    }

    #[test]
    fn parses_chat_fixture() {
        // /ranch/chat 直接返回 Map<String, Vec<String>>。
        let envelope: ApiResponse<HashMap<String, Vec<String>>> =
            parse(include_str!("../../tests/fixtures/ranch_chat.json"));
        assert!(envelope.is_success());
        let chat = envelope.data.expect("data 不应为空");
        assert!(!chat.is_empty());
        assert!(chat.contains_key("阴山大草原"));
        assert!(chat.contains_key("鲲鹏岛"));
        assert!(chat["鲲鹏岛"].is_empty());
        assert!(!chat["黑戈壁"].is_empty());
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let records: RanchRecords = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(records.server, "长安城");
        assert!(records.zone.is_empty());
        assert!(records.data.is_empty());
        assert!(records.note.is_empty());
    }
}
