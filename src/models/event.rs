//! 奇遇相关数据模型。
//!
//! 对应接口：`/event/collect`、`/event/missing`、`/event/recent`、
//! `/event/records`、`/event/statistics`、`/event/strategy`。

use serde::{Deserialize, Serialize};

/// 奇遇记录条目。
///
/// 同时用于 `/event/recent`、`/event/records`、`/event/statistics` 三个接口。
/// 注意其中的 `name` 是**角色名**，`event` 才是**奇遇名**。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 角色名称。
    pub name: String,
    /// 奇遇名称。
    pub event: String,
    /// 奇遇状态。
    pub status: i64,
    /// 数据来源。
    pub source: i64,
    /// 触发时间戳（秒）。
    pub time: i64,
    /// 奇遇等级（部分接口不返回，缺失时为 `0`）。
    pub level: i64,
}

/// 奇遇汇总条目。
///
/// 对应接口：`GET /event/collect`，按奇遇统计最近的触发次数与最后一条记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventCollect {
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 奇遇名称。
    pub event: String,
    /// 统计区间内的触发次数。
    pub count: i64,
    /// 最近一次触发记录。
    pub last: EventLastRecord,
}

/// 奇遇汇总中的最近一条触发记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventLastRecord {
    /// 角色名称。
    pub name: String,
    /// 触发时间戳（秒）。
    pub time: i64,
}

/// 尚未触发的奇遇条目。
///
/// 对应接口：`GET /event/missing`。接口参数 `name` 是**角色名**，
/// 返回的 [`EventMissing::name`] 才是**奇遇名**。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventMissing {
    /// 奇遇名称。
    pub name: String,
    /// 奇遇类型，例如 `世界奇遇`、`绝世奇遇`。
    #[serde(rename = "type")]
    pub r#type: String,
    /// 奇遇等级。
    pub level: i64,
}

/// 奇遇攻略（白名单配置）。
///
/// 对应接口：`GET /event/strategy`。接口参数 `name` 是**奇遇名**。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventStrategy {
    /// 宝箱 id。
    #[serde(rename = "box")]
    pub r#box: i64,
    /// 奇遇类型，例如 `世界奇遇`。
    #[serde(rename = "type")]
    pub r#type: String,
    /// 奇遇名称。
    pub name: String,
    /// 奇遇等级。
    pub level: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_collect_fixture() {
        let envelope: ApiResponse<Vec<EventCollect>> =
            parse(include_str!("../../tests/fixtures/event_collect.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        let first = &items[0];
        assert_eq!(first.zone, "电信区");
        assert_eq!(first.server, "长安城");
        assert!(!first.event.is_empty());
        assert!(first.count > 0);
        assert!(!first.last.name.is_empty());
        assert!(first.last.time > 0);
    }

    #[test]
    fn parses_missing_fixture() {
        let envelope: ApiResponse<Vec<EventMissing>> =
            parse(include_str!("../../tests/fixtures/event_missing.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert!(!items[0].name.is_empty());
        assert!(!items[0].r#type.is_empty());
        assert!(items[0].level > 0);
    }

    #[test]
    fn parses_recent_fixture() {
        let envelope: ApiResponse<Vec<EventRecord>> =
            parse(include_str!("../../tests/fixtures/event_recent.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert_eq!(items[0].server, "长安城");
        assert!(!items[0].name.is_empty());
        assert!(!items[0].event.is_empty());
        // 该接口不返回 level，缺失时应回落为 0。
        assert_eq!(items[0].level, 0);
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<Vec<EventRecord>> =
            parse(include_str!("../../tests/fixtures/event_records.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert_eq!(items[0].name, "漓心");
        assert_eq!(items[0].event, "白日梦");
        assert!(items[0].level >= 1);
    }

    #[test]
    fn parses_statistics_fixture() {
        let envelope: ApiResponse<Vec<EventRecord>> =
            parse(include_str!("../../tests/fixtures/event_statistics.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert_eq!(items[0].event, "白日梦");
    }

    #[test]
    fn parses_strategy_fixture() {
        let envelope: ApiResponse<EventStrategy> =
            parse(include_str!("../../tests/fixtures/event_strategy.json"));
        assert!(envelope.is_success());
        let strategy = envelope.data.expect("data 不应为空");
        assert_eq!(strategy.name, "白日梦");
        assert_eq!(strategy.r#type, "世界奇遇");
        assert_eq!(strategy.level, 1);
        assert!(strategy.r#box > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: EventRecord = serde_json::from_str(r#"{"name":"漓心"}"#).unwrap();
        assert_eq!(record.name, "漓心");
        assert!(record.event.is_empty());
        assert_eq!(record.level, 0);
    }
}
