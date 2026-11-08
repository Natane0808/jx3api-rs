//! 沙盘据点数据模型。
//!
//! 对应接口：`/sand/records`。

use serde::{Deserialize, Serialize};

/// 单个据点的归属信息。
///
/// 对应接口：`GET /sand/records?server=长安城`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SandRecord {
    /// 帮会 id。
    #[serde(rename = "tongId")]
    pub tong_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
    /// 据点 id。
    #[serde(rename = "castleId")]
    pub castle_id: i64,
    /// 据点名称。
    #[serde(rename = "castleName")]
    pub castle_name: String,
    /// 帮主 id。
    #[serde(rename = "masterId")]
    pub master_id: i64,
    /// 帮主角色名。
    #[serde(rename = "masterName")]
    pub master_name: String,
    /// 阵营 id。
    #[serde(rename = "campId")]
    pub camp_id: i64,
    /// 阵营名称，例如 `浩气盟` / `恶人谷`。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 本周骑乘值。
    #[serde(rename = "ridePiece")]
    pub ride_piece: i64,
    /// 本周被献祭（防守）次数。
    #[serde(rename = "defendCount")]
    pub defend_count: i64,
}

/// 指定服务器的沙盘据点总览。
///
/// 对应接口：`GET /sand/records`，响应 `data` 为该结构。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerSand {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 重置标记（接口原样透传）。
    pub reset: i64,
    /// 数据更新时间（Unix 时间戳，秒）。
    pub update: i64,
    /// 各据点的归属信息。
    pub data: Vec<SandRecord>,
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
        let envelope: ApiResponse<ServerSand> =
            parse(include_str!("../../tests/fixtures/sand_records.json"));
        assert!(envelope.is_success());
        let sand = envelope.data.expect("data 不应为空");
        assert_eq!(sand.zone, "电信区");
        assert_eq!(sand.server, "长安城");
        assert_eq!(sand.reset, 0);
        assert!(sand.update > 0);
        assert!(!sand.data.is_empty());
        let first = &sand.data[0];
        assert_eq!(first.tong_id, 71205);
        assert_eq!(first.tong_name, "追梦烟雨");
        assert_eq!(first.castle_name, "金门关");
        assert_eq!(first.camp_name, "浩气盟");
        assert!(first.ride_piece > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let sand: ServerSand = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(sand.server, "长安城");
        assert!(sand.data.is_empty());
        assert_eq!(sand.update, 0);
    }
}
