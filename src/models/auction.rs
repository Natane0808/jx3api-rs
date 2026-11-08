//! 阵营拍卖数据模型。
//!
//! 对应接口：`/auction/records`。

use serde::{Deserialize, Serialize};

/// 一条阵营拍卖记录。
///
/// 对应接口：`GET /auction/records?server=长安城`，结果按时间倒序。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AuctionRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 竞拍角色名。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 阵营名称，例如 `恶人谷` / `浩气盟`。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 物品名称。
    #[serde(rename = "itemName")]
    pub item_name: String,
    /// 成交金额，官方为带「金」单位的字符串，例如 `2万7600金`。
    #[serde(rename = "itemAmount")]
    pub item_amount: String,
    /// 成交时间（Unix 时间戳，秒）。
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
        let envelope: ApiResponse<Vec<AuctionRecord>> =
            parse(include_str!("../../tests/fixtures/auction_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        assert_eq!(records[0].zone, "电信区");
        assert_eq!(records[0].server, "长安城");
        assert!(!records[0].role_name.is_empty());
        assert!(!records[0].item_name.is_empty());
        assert!(records[0].item_amount.contains('金'));
        assert!(records[0].time > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: AuctionRecord = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(record.server, "长安城");
        assert!(record.role_name.is_empty());
        assert_eq!(record.time, 0);
    }
}
