//! 烟花记录数据模型。
//!
//! 对应接口：`/firework/records`。

use serde::{Deserialize, Serialize};

/// 一条烟花记录。
///
/// 对应接口：`GET /firework/records?server=长安城&name=漓心`，结果按时间倒序。
///
/// 注意响应字段驼峰与蛇形混用：`sender` / `receiver` / `firework` 为蛇形，
/// 而 `mapName` 为驼峰，这里严格按夹具命名，不做统一。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FireworkRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 送花人角色名（蛇形字段）。
    pub sender: String,
    /// 燃放地图（驼峰字段 `mapName`）。
    #[serde(rename = "mapName")]
    pub map_name: String,
    /// 收花人角色名（蛇形字段）。
    pub receiver: String,
    /// 烟花名称，例如 `星海之约`。
    pub firework: String,
    /// 配对标记（接口原样透传）。
    pub matched: i64,
    /// 状态标记（接口原样透传）。
    pub status: i64,
    /// 燃放时间（Unix 时间戳，秒）。
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
        let envelope: ApiResponse<Vec<FireworkRecord>> =
            parse(include_str!("../../tests/fixtures/firework_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        let first = &records[0];
        assert_eq!(first.server, "长安城");
        assert_eq!(first.sender, "漓心");
        assert_eq!(first.map_name, "成都");
        assert_eq!(first.receiver, "一哞叽");
        assert_eq!(first.firework, "星海之约");
        assert_eq!(first.matched, 1);
        assert_eq!(first.status, 1);
        assert!(first.time > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: FireworkRecord = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(record.server, "长安城");
        assert!(record.map_name.is_empty());
        assert_eq!(record.matched, 0);
    }
}
