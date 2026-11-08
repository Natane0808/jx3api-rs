//! 试炼之地相关数据模型。
//!
//! 对应接口：`/trial/bosses`、`/trial/seasons`。
//!
//! 这两个接口官方 Python SDK 尚未封装，模型依据 `tests/fixtures/trial_*.json`
//! 的真实响应推断。

use serde::{Deserialize, Serialize};

/// 某赛季某层的 BOSS 血量与秒伤。
///
/// 对应接口：`GET /trial/bosses`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrialBosses {
    /// 赛季名称，例如 `暗影千机`。
    pub season: String,
    /// 层数。
    pub floor: i64,
    /// 血量成长系数（采样中为 `null`，官方未给出结构，保留原始 JSON）。
    #[serde(rename = "hpGrowth")]
    pub hp_growth: Option<serde_json::Value>,
    /// BOSS 列表。
    pub bosses: Vec<TrialBoss>,
}

/// 单个 BOSS 的秒伤数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrialBoss {
    /// BOSS 名称。
    pub boss: String,
    /// 血量。
    pub hp: i64,
    /// 分组：`main` / `short` / `clone`（分身无秒伤）。
    pub group: String,
    /// 完美循环秒伤，分身等无秒伤时为 `null`。
    #[serde(rename = "perfectDps")]
    pub perfect_dps: Option<i64>,
    /// 通关秒伤，分身等无秒伤时为 `null`。
    #[serde(rename = "clearDps")]
    pub clear_dps: Option<i64>,
    /// 备注，例如 `仅计算本体承伤`。
    pub note: Option<String>,
}

/// 赛季信息。
///
/// 对应接口：`GET /trial/seasons`（返回数组）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrialSeason {
    /// 赛季名称。
    pub season: String,
    /// 赛季最高层数。
    #[serde(rename = "maxFloor")]
    pub max_floor: i64,
    /// 该赛季的 BOSS 顺序。
    pub bosses: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_bosses_fixture() {
        let envelope: ApiResponse<TrialBosses> =
            parse(include_str!("../../tests/fixtures/trial_bosses.json"));
        assert!(envelope.is_success());
        let bosses = envelope.data.expect("data 不应为空");
        assert_eq!(bosses.season, "暗影千机");
        assert_eq!(bosses.floor, 1);
        assert!(bosses.hp_growth.is_none());
        assert_eq!(bosses.bosses.len(), 10);
        let first = &bosses.bosses[0];
        assert_eq!(first.boss, "雷神");
        assert_eq!(first.hp, 125991632);
        assert_eq!(first.group, "short");
        assert_eq!(first.perfect_dps, Some(1049930));
        assert!(first.note.is_none());
        let clone = bosses
            .bosses
            .iter()
            .find(|item| item.group == "clone")
            .expect("夹具里应有分身");
        assert!(clone.perfect_dps.is_none());
        let noted = bosses
            .bosses
            .iter()
            .find(|item| item.note.is_some())
            .expect("夹具里应有带备注的 BOSS");
        assert_eq!(noted.note.as_deref(), Some("仅计算本体承伤"));
    }

    #[test]
    fn parses_seasons_fixture() {
        let envelope: ApiResponse<Vec<TrialSeason>> =
            parse(include_str!("../../tests/fixtures/trial_seasons.json"));
        assert!(envelope.is_success());
        let seasons = envelope.data.expect("data 不应为空");
        assert_eq!(seasons.len(), 4);
        assert_eq!(seasons[0].season, "暗影千机");
        assert_eq!(seasons[0].max_floor, 120);
        assert_eq!(seasons[0].bosses.len(), 10);
        assert_eq!(seasons[3].season, "丝路风语");
        assert!(seasons[3].bosses.contains(&"子".to_owned()));
    }
}
