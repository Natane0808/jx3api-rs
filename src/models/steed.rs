//! 的卢拍卖数据模型。
//!
//! 对应接口：`/steed/records`。

use serde::{Deserialize, Serialize};

/// 一条的卢（坐骑）抓捕与拍卖记录。
///
/// 对应接口：`GET /steed/records?server=长安城`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SteedRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 地图名称（驼峰字段 `mapName`）。
    #[serde(rename = "mapName")]
    pub map_name: String,
    /// 刷新时间（Unix 时间戳，秒）。
    #[serde(rename = "refreshTime")]
    pub refresh_time: i64,
    /// 抓捕阵营名称。
    #[serde(rename = "captureCampName")]
    pub capture_camp_name: String,
    /// 抓捕角色名。
    #[serde(rename = "captureRoleName")]
    pub capture_role_name: String,
    /// 抓捕时间（Unix 时间戳，秒）。
    #[serde(rename = "captureTime")]
    pub capture_time: i64,
    /// 拍卖成交金额，官方为带「金」单位的字符串，例如 `413万7000金`。
    #[serde(rename = "auctionAmount")]
    pub auction_amount: String,
    /// 拍卖阵营名称。
    #[serde(rename = "auctionCampName")]
    pub auction_camp_name: String,
    /// 拍卖角色名。
    #[serde(rename = "auctionRoleName")]
    pub auction_role_name: String,
    /// 拍卖时间（Unix 时间戳，秒）。
    #[serde(rename = "auctionTime")]
    pub auction_time: i64,
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
        let envelope: ApiResponse<Vec<SteedRecord>> =
            parse(include_str!("../../tests/fixtures/steed_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        let first = &records[0];
        assert_eq!(first.server, "长安城");
        assert_eq!(first.map_name, "龙泉府");
        assert!(first.refresh_time > 0);
        assert_eq!(first.capture_role_name, "沐云曦");
        assert!(first.auction_amount.contains('金'));
        assert!(first.auction_time > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: SteedRecord = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(record.server, "长安城");
        assert!(record.map_name.is_empty());
        assert_eq!(record.auction_time, 0);
    }
}
