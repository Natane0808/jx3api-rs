//! 排行榜相关数据模型。
//!
//! 对应接口：`/rank/statistics`、`/rank/trials`，以及官方返回未定型
//! 结构、以 [`serde_json::Value`] 透传的 `/rank/arena`、`/rank/championship`、
//! `/rank/constable`、`/rank/outlaw`、`/rank/wanted`。

use serde::{Deserialize, Serialize};

/// 排行榜查询结果（`/rank/statistics`）。
///
/// 官方 `data` 形如 `{ id, zone, server, name, data: [...], time }`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RankStatistics {
    /// 榜单记录 id。
    pub id: i64,
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 榜单名称。
    pub name: String,
    /// 榜单条目。
    pub data: Vec<RankStatisticsItem>,
    /// 榜单时间戳（秒）。
    pub time: i64,
}

/// 排行统计中的单条记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RankStatisticsItem {
    /// 历史最高数量。
    pub max_count: i64,
    /// 当前数量。
    pub now_count: i64,
    /// 帮会名称。
    pub tong_name: String,
    /// 据点名称。
    pub castle_name: String,
    /// 帮主名称。
    pub master_name: String,
    /// 总积分。
    pub total_score: i64,
}

/// 试炼之地排行榜查询结果（`/rank/trials`）。
///
/// 官方 `data` 形如 `{ id, zone, server, name, data: [...], time }`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RankTrials {
    /// 榜单记录 id。
    pub id: i64,
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 榜单名称。
    pub name: String,
    /// 榜单条目。
    pub data: Vec<RankTrialsItem>,
    /// 榜单时间戳（秒）。
    pub time: i64,
}

/// 试炼之地排行榜中的单条记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RankTrialsItem {
    /// 最高层数。
    pub max_level: i64,
    /// 角色名称。
    pub role_name: String,
    /// 装备分数。
    pub equip_score: i64,
    /// 总积分。
    pub total_score: i64,
}

/// 从榜单接口的 `data` 中取出条目列表。
///
/// `/rank/arena`、`/rank/championship`、`/rank/constable`、`/rank/outlaw`、
/// `/rank/wanted` 的 `data` 实际是
/// `{ "server": ..., "name": ..., "mode": ..., "data": [...], "time": ... }`
/// 形式的对象，条目字段官方未定型。本函数兼容「裸数组」与「对象内 `data` 数组」
/// 两种形态，统一返回条目列表，供上层以 [`serde_json::Value`] 透传。
pub(crate) fn extract_rank_entries(value: serde_json::Value) -> Vec<serde_json::Value> {
    match value {
        serde_json::Value::Array(items) => items,
        serde_json::Value::Object(mut map) => match map.remove("data") {
            Some(serde_json::Value::Array(items)) => items,
            Some(serde_json::Value::Null) | None => Vec::new(),
            Some(other) => vec![other],
        },
        serde_json::Value::Null => Vec::new(),
        other => vec![other],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_rank_statistics_fixture() {
        // 夹具是缺少 name 的错误响应，只断言包络可解析。
        let envelope: ApiResponse<RankStatistics> =
            parse(include_str!("../../tests/fixtures/rank_statistics.json"));
        assert!(!envelope.is_success());
    }

    #[test]
    fn parses_rank_trials_fixture() {
        // 夹具是缺少 name 的错误响应，只断言包络可解析。
        let envelope: ApiResponse<RankTrials> =
            parse(include_str!("../../tests/fixtures/rank_trials.json"));
        assert!(!envelope.is_success());
    }

    #[test]
    fn parses_rank_statistics_payload() {
        let raw = r#"{"code":200,"msg":"success","data":{
            "id":1,"zone":"电信区","server":"长安城","name":"帮会威望榜",
            "data":[{"max_count":10,"now_count":5,"tong_name":"风花雪月归故里",
                     "castle_name":"洛阳","master_name":"张三","total_score":123}],
            "time":1794107000
        }}"#;
        let envelope: ApiResponse<RankStatistics> = parse(raw);
        assert!(envelope.is_success());
        let rank = envelope.data.expect("data 不应为空");
        assert_eq!(rank.name, "帮会威望榜");
        assert_eq!(rank.data.len(), 1);
        assert_eq!(rank.data[0].tong_name, "风花雪月归故里");
        assert_eq!(rank.data[0].total_score, 123);
    }

    #[test]
    fn parses_rank_trials_payload() {
        let raw = r#"{"code":200,"msg":"success","data":{
            "id":2,"zone":"电信区","server":"长安城","name":"试炼之地",
            "data":[{"max_level":30,"role_name":"漓心","equip_score":12000,
                     "total_score":350}],
            "time":1794107000
        }}"#;
        let envelope: ApiResponse<RankTrials> = parse(raw);
        assert!(envelope.is_success());
        let rank = envelope.data.expect("data 不应为空");
        assert_eq!(rank.data.len(), 1);
        assert_eq!(rank.data[0].role_name, "漓心");
        assert_eq!(rank.data[0].max_level, 30);
    }

    #[test]
    fn extracts_rank_entries_from_fixtures() {
        let arena: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/rank_arena.json"));
        let entries = extract_rank_entries(arena.data.expect("data 不应为空"));
        assert!(!entries.is_empty());
        assert!(entries[0].get("corpsName").is_some());

        let constable: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/rank_constable.json"));
        let entries = extract_rank_entries(constable.data.expect("data 不应为空"));
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["name"], "姬千凡");

        let outlaw: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/rank_outlaw.json"));
        let entries = extract_rank_entries(outlaw.data.expect("data 不应为空"));
        assert!(!entries.is_empty());
        assert!(entries[0].get("playerId").is_some());

        let championship: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/rank_championship.json"));
        assert!(extract_rank_entries(championship.data.expect("data 不应为空")).is_empty());

        let wanted: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/rank_wanted.json"));
        assert!(extract_rank_entries(wanted.data.expect("data 不应为空")).is_empty());
    }

    #[test]
    fn extracts_bare_array() {
        let entries = extract_rank_entries(serde_json::json!([
            {"name": "姬千凡"},
            {"name": "漓心"}
        ]));
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[1]["name"], "漓心");
    }
}
