//! 掉落统计相关数据模型。
//!
//! 对应接口：`/reward/statistics`。

use serde::{Deserialize, Serialize};

/// 掉落记录条目。
///
/// 对应接口：`GET /reward/statistics`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RewardRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 地图名称。
    #[serde(rename = "mapName")]
    pub map_name: String,
    /// 物品名称。
    #[serde(rename = "itemName")]
    pub item_name: String,
    /// 物品数量。
    #[serde(rename = "itemAmount")]
    pub item_amount: i64,
    /// 掉落时间戳（秒）。
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
    fn parses_error_fixture() {
        // 夹具是缺少 name 的错误响应，只断言能解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/reward_statistics.json"));
        assert!(!envelope.is_success());
    }

    #[test]
    fn parses_success_payload() {
        let raw = r#"{"code":200,"msg":"success","data":[
            {"zone":"电信区","server":"长安城","roleName":"漓心",
             "mapName":"英雄冷龙峰","itemName":"玄晶","itemAmount":1,
             "time":1794000000}
        ]}"#;
        let envelope: ApiResponse<Vec<RewardRecord>> = parse(raw);
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].role_name, "漓心");
        assert_eq!(items[0].item_name, "玄晶");
        assert_eq!(items[0].item_amount, 1);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: RewardRecord = serde_json::from_str(r#"{"itemName":"玄晶"}"#).unwrap();
        assert_eq!(record.item_name, "玄晶");
        assert_eq!(record.item_amount, 0);
        assert!(record.role_name.is_empty());
    }
}
