//! 关隘首领数据模型。
//!
//! 对应接口：`/castle/status`。

use serde::{Deserialize, Serialize};

/// 单个关隘的首领状态。
///
/// 对应接口：`GET /castle/status`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CastleRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `斗转星移`。
    pub server: String,
    /// 关隘首领名，格式为「角色名·服务器」。
    pub leader: String,
    /// 阵营名称，例如 `浩气盟` / `恶人谷`。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 关隘名称，例如 `回雁关`。
    pub castle: String,
    /// 状态编号，例如 `4` 表示保护期。
    pub status: i64,
    /// 状态文案，例如 `保护期`。
    #[serde(rename = "statusText")]
    pub status_text: String,
    /// 状态开始时间（Unix 时间戳，秒）。
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// 状态结束时间（Unix 时间戳，秒）。
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

/// 按服务器分组的关隘首领状态。
///
/// 对应接口：`GET /castle/status`，响应 `data` 为该结构的数组。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CastleStatus {
    /// 服务器名称，例如 `斗转星移`。
    pub server: String,
    /// 该服的关隘首领状态列表。
    pub data: Vec<CastleRecord>,
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
        let envelope: ApiResponse<Vec<CastleStatus>> =
            parse(include_str!("../../tests/fixtures/castle_status.json"));
        assert!(envelope.is_success());
        let statuses = envelope.data.expect("data 不应为空");
        assert!(!statuses.is_empty());
        assert_eq!(statuses[0].server, "斗转星移");
        assert!(!statuses[0].data.is_empty());
        assert_eq!(statuses[0].data[0].castle, "回雁关");
        assert_eq!(statuses[0].data[0].camp_name, "浩气盟");
        assert_eq!(statuses[0].data[0].status_text, "保护期");
        assert!(statuses[0].data[0].end_time > statuses[0].data[0].start_time);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let status: CastleStatus = serde_json::from_str(r#"{"server":"天鹅坪"}"#).unwrap();
        assert_eq!(status.server, "天鹅坪");
        assert!(status.data.is_empty());
    }
}
