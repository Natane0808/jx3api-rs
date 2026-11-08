//! 百战相关数据模型。
//!
//! 对应接口：`/monster/records`、`/monster/weekly`。

use serde::{Deserialize, Serialize};

/// 角色的百战技能配装。
///
/// 对应接口：`GET /monster/records`。未打过百战的角色会返回全服默认技能。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonsterRecord {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色编号。
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// 全局角色编号。
    #[serde(rename = "globalId")]
    pub global_id: String,
    /// 剩余体力。
    #[serde(rename = "skillStamina")]
    pub skill_stamina: i64,
    /// 剩余精力。
    #[serde(rename = "skillEnergy")]
    pub skill_energy: i64,
    /// 已配置技能数量。
    #[serde(rename = "skillCount")]
    pub skill_count: i64,
    /// 技能列表。
    #[serde(rename = "skillList")]
    pub skill_list: Vec<MonsterSkill>,
    /// 数据更新时间戳（Unix 秒）。
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// 百战技能。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonsterSkill {
    /// 是否已废弃。
    #[serde(
        rename = "bDeprecated",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub b_deprecated: bool,
    /// 客户端内部技能 id。
    #[serde(rename = "dwInSkillID")]
    pub dw_in_skill_id: i64,
    /// 客户端对外技能 id。
    #[serde(rename = "dwOutSkillID")]
    pub dw_out_skill_id: i64,
    /// 技能颜色等级。
    #[serde(rename = "nColor")]
    pub n_color: i64,
    /// 技能消耗。
    #[serde(rename = "nCost")]
    pub n_cost: i64,
    /// 技能等级。
    #[serde(rename = "nLevel")]
    pub n_level: i64,
    /// 适用性别。
    #[serde(rename = "nSex")]
    pub n_sex: i64,
    /// 首领编号。
    #[serde(rename = "szBoss")]
    pub sz_boss: String,
    /// 首领名称。
    #[serde(rename = "szBossName")]
    pub sz_boss_name: String,
    /// 技能名称。
    #[serde(rename = "szSkillName")]
    pub sz_skill_name: String,
    /// 技能类型编号组合，例如 `2;8`。
    #[serde(rename = "szType")]
    pub sz_type: String,
}

/// 本周百战首领与 buff 效果。
///
/// 对应接口：`GET /monster/weekly`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonsterWeekly {
    /// 周数，例如 `44`。
    pub week: String,
    /// 周期开始时间戳（Unix 秒）。
    pub start: i64,
    /// 周期结束时间戳（Unix 秒）。
    pub end: i64,
    /// 当周荡剑恩仇轮换首领。
    pub boss: String,
    /// 首领列表。
    pub list: Vec<MonsterWeeklyItem>,
}

/// 百战周常中的单个首领。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonsterWeeklyItem {
    /// 排序序号。
    pub index: i64,
    /// 首领名称。
    pub name: String,
    /// 技能名称列表。
    pub skill: Vec<String>,
    /// 首领效果。
    pub data: MonsterWeeklyEffect,
}

/// 首领效果说明。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonsterWeeklyEffect {
    /// 效果名称，无特殊效果时为 `无`。
    pub name: String,
    /// 效果条目（官方可能返回字符串或对象，保留原始 JSON）。
    pub list: Vec<serde_json::Value>,
    /// 效果描述。
    pub desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<MonsterRecord> =
            parse(include_str!("../../tests/fixtures/monster_records.json"));
        assert!(envelope.is_success());
        let record = envelope.data.expect("data 不应为空");
        assert_eq!(record.zone, "电信区");
        assert_eq!(record.server, "长安城");
        assert_eq!(record.skill_stamina, 142120);
        assert_eq!(record.skill_count, 151);
        assert_eq!(record.skill_list.len(), 156);
        let skill = &record.skill_list[0];
        assert_eq!(skill.dw_in_skill_id, 28029);
        assert_eq!(skill.sz_boss_name, "冯度");
        assert_eq!(skill.sz_skill_name, "空穴来风");
        assert!(!skill.b_deprecated);
    }

    #[test]
    fn parses_weekly_fixture() {
        let envelope: ApiResponse<MonsterWeekly> =
            parse(include_str!("../../tests/fixtures/monster_weekly.json"));
        assert!(envelope.is_success());
        let weekly = envelope.data.expect("data 不应为空");
        assert_eq!(weekly.week, "44");
        assert_eq!(weekly.boss, "罗伊客");
        assert_eq!(weekly.list.len(), 100);
        let first = &weekly.list[0];
        assert_eq!(first.index, 1);
        assert_eq!(first.name, "上衫勇刀");
        assert_eq!(first.skill.len(), 5);
        assert_eq!(first.data.name, "无");
        assert_eq!(first.data.desc, "无");
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: MonsterRecord = serde_json::from_str(r#"{"server":"长安城"}"#).unwrap();
        assert_eq!(record.server, "长安城");
        assert_eq!(record.skill_count, 0);
        assert!(record.skill_list.is_empty());
    }
}
