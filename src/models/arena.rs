//! 名剑大会相关数据模型。
//!
//! 对应接口：`/arena/awesome`、`/arena/recent`、`/arena/schools`。

use serde::{Deserialize, Serialize};

/// 名剑排行条目。
///
/// 对应接口：`GET /arena/awesome`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArenaAwesome {
    /// 大区名称，例如 `电信区`。
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    /// 服务器名称，例如 `长安城`。
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 门派名称。
    #[serde(rename = "forceName")]
    pub force_name: String,
    /// 角色头像地址。
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    /// 排名名次（官方返回字符串）。
    #[serde(rename = "rankNum")]
    pub rank_num: String,
    /// 名剑分数（官方返回字符串）。
    pub score: String,
    /// 名次变化（官方返回字符串）。
    #[serde(rename = "upNum")]
    pub up_num: String,
    /// 胜率（官方返回字符串）。
    #[serde(rename = "winRate")]
    pub win_rate: String,
}

/// 角色竞技场名片与战绩。
///
/// 对应接口：`GET /arena/recent`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArenaRecent {
    /// 大区名称。
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    /// 服务器名称。
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色 id（官方返回字符串）。
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// 全局角色 id（官方返回字符串）。
    #[serde(rename = "globalId")]
    pub global_id: String,
    /// 门派名称。
    #[serde(rename = "forceName")]
    pub force_name: String,
    /// 门派 id。
    #[serde(rename = "forceId")]
    pub force_id: i64,
    /// 体型名称。
    #[serde(rename = "bodyName")]
    pub body_name: String,
    /// 体型 id。
    #[serde(rename = "bodyId")]
    pub body_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
    /// 帮会 id。
    #[serde(rename = "tongId")]
    pub tong_id: i64,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 阵营 id（官方可能返回数字或字符串，保留原始 JSON）。
    #[serde(rename = "campId")]
    pub camp_id: serde_json::Value,
    /// 各模式战绩表现（官方 `total=False` 字典：`2v2` / `3v3` / `5v5`）。
    pub performance: ArenaRecentPerformance,
    /// 历史对战记录。
    pub history: Vec<ArenaRecentHistory>,
}

/// 各模式战绩表现。
///
/// 对应 pyi 的 `ArenaRecentPerformance`（`total=False`），键为 `2v2` / `3v3` / `5v5`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArenaRecentPerformance {
    /// 2v2 战绩。
    #[serde(rename = "2v2")]
    pub v2v: ArenaPerformanceItem,
    /// 3v3 战绩。
    #[serde(rename = "3v3")]
    pub v3v: ArenaPerformanceItem,
    /// 5v5 战绩（官方结构不固定，保留原始 JSON）。
    #[serde(rename = "5v5")]
    pub v5v: Vec<serde_json::Value>,
}

/// 单个模式的战绩条目（pyi `ArenaPerformanceItem`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ArenaPerformanceItem {
    /// 个人 MMR。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub mmr: i64,
    /// 段位。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub grade: i64,
    /// 排名说明。
    #[serde(deserialize_with = "crate::serde_util::flexible_string")]
    pub ranking: String,
    /// 胜场数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub win_count: i64,
    /// 总场数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub total_count: i64,
    /// MVP 次数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub mvp_count: i64,
    /// 竞技模式（官方返回字符串）。
    #[serde(deserialize_with = "crate::serde_util::flexible_string")]
    pub pvp_type: String,
    /// 胜率。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub win_rate: i64,
}

/// 单场历史对战记录。
///
/// 属于 [`ArenaRecent::history`]。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArenaRecentHistory {
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 平均段位。
    #[serde(rename = "avgGrade")]
    pub avg_grade: i64,
    /// 队伍总 MMR。
    #[serde(rename = "totalMmr")]
    pub total_mmr: i64,
    /// 个人 MMR。
    pub mmr: i64,
    /// 心法名称。
    pub kungfu: String,
    /// 对战模式。
    #[serde(rename = "pvpType")]
    pub pvp_type: i64,
    /// 是否获胜。
    pub won: bool,
    /// 是否 MVP。
    pub mvp: bool,
    /// 开始时间戳（秒）。
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// 结束时间戳（秒）。
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

/// 名剑门派统计条目。
///
/// 对应接口：`GET /arena/schools`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArenaSchools {
    /// 门派名称。
    pub name: String,
    /// 本周统计数量。
    pub this: i64,
    /// 上周统计数量。
    pub last: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    // 三个 arena 夹具都是「缺少 ticket」的错误响应，只断言包络可解析。
    #[test]
    fn parses_arena_error_fixtures() {
        let awesome: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/arena_awesome.json"));
        assert!(!awesome.is_success());
        let recent: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/arena_recent.json"));
        assert!(!recent.is_success());
        let schools: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/arena_schools.json"));
        assert!(!schools.is_success());
    }

    #[test]
    fn parses_awesome_payload() {
        let raw = r#"{"code":200,"msg":"success","data":[
            {"zoneName":"电信区","serverName":"长安城","roleName":"漓心","forceName":"万花",
             "avatarUrl":"https://example.com/a.png","rankNum":"1","score":"2500",
             "upNum":"3","winRate":"68%"}
        ]}"#;
        let envelope: ApiResponse<Vec<ArenaAwesome>> = parse(raw);
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].zone_name, "电信区");
        assert_eq!(items[0].role_name, "漓心");
        assert_eq!(items[0].rank_num, "1");
        assert_eq!(items[0].win_rate, "68%");
    }

    #[test]
    fn parses_recent_payload() {
        let raw = r#"{"code":200,"msg":"success","data":{
            "zoneName":"电信区","serverName":"长安城","roleName":"漓心","roleId":"123",
            "globalId":"456","forceName":"万花","forceId":2,"bodyName":"成女","bodyId":2,
            "tongName":"情醉","tongId":53417,"campName":"浩气盟","campId":"1",
            "performance":{
                "2v2":{"mmr":2400,"grade":12,"ranking":"第 1 名","winCount":10,
                       "totalCount":12,"mvpCount":3,"pvpType":"2v2","winRate":83},
                "3v3":{"mmr":2300,"grade":11,"ranking":"第 2 名","winCount":6,
                       "totalCount":10,"mvpCount":1,"pvpType":"3v3","winRate":60},
                "5v5":[{"mmr":2500}]
            },
            "history":[{"zone":"电信区","server":"长安城","avgGrade":12,"totalMmr":2500,
                        "mmr":2400,"kungfu":"花间游","pvpType":1,"won":true,"mvp":false,
                        "startTime":1794000000,"endTime":1794000600}]
        }}"#;
        let envelope: ApiResponse<ArenaRecent> = parse(raw);
        assert!(envelope.is_success());
        let recent = envelope.data.expect("data 不应为空");
        assert_eq!(recent.role_name, "漓心");
        assert_eq!(recent.force_id, 2);
        assert_eq!(recent.camp_id, serde_json::json!("1"));
        assert_eq!(recent.performance.v2v.mmr, 2400);
        assert_eq!(recent.performance.v2v.grade, 12);
        assert_eq!(recent.performance.v2v.ranking, "第 1 名");
        assert_eq!(recent.performance.v2v.win_count, 10);
        assert_eq!(recent.performance.v2v.total_count, 12);
        assert_eq!(recent.performance.v2v.mvp_count, 3);
        assert_eq!(recent.performance.v2v.pvp_type, "2v2");
        assert_eq!(recent.performance.v2v.win_rate, 83);
        assert_eq!(recent.performance.v3v.mmr, 2300);
        assert_eq!(recent.performance.v5v.len(), 1);
        assert_eq!(recent.performance.v5v[0]["mmr"], serde_json::json!(2500));
        assert_eq!(recent.history.len(), 1);
        assert_eq!(recent.history[0].kungfu, "花间游");
        assert!(recent.history[0].won);
    }

    #[test]
    fn parses_schools_payload() {
        let raw = r#"{"code":200,"msg":"success","data":[
            {"name":"万花","this":100,"last":90}
        ]}"#;
        let envelope: ApiResponse<Vec<ArenaSchools>> = parse(raw);
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "万花");
        assert_eq!(items[0].this, 100);
        assert_eq!(items[0].last, 90);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let item: ArenaSchools = serde_json::from_str(r#"{"name":"万花"}"#).unwrap();
        assert_eq!(item.name, "万花");
        assert_eq!(item.this, 0);
        assert_eq!(item.last, 0);
    }
}
