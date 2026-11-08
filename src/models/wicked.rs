//! 诛恶事件数据模型。
//!
//! 对应接口：`/wicked/records`。

use serde::{Deserialize, Serialize};

/// 一条恶人（诛恶）刷新记录。
///
/// 对应接口：`GET /wicked/records`，结果按时间倒序。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WickedRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 刷新地图名称（驼峰字段 `mapName`）。
    #[serde(rename = "mapName")]
    pub map_name: String,
    /// 刷新时间（Unix 时间戳，秒）。
    pub time: i64,
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
        let envelope: ApiResponse<Vec<WickedRecord>> =
            parse(include_str!("../../tests/fixtures/wicked_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        let first = &records[0];
        assert_eq!(first.zone, "电信区");
        assert_eq!(first.server, "长安城");
        assert_eq!(first.map_name, "烂柯山·1号分线");
        assert!(first.time > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: WickedRecord = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(record.server, "长安城");
        assert!(record.map_name.is_empty());
        assert_eq!(record.time, 0);
    }
}
