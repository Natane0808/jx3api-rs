//! 心法配装相关数据模型。
//!
//! 对应接口：`/school/matrix`、`/school/seniority`、`/school/skills`、
//! `/school/talent`、`/school/search`。
//!
//! `/school/search` 官方返回结构不固定的字典，API 层直接返回
//! [`serde_json::Value`]，本模块不为其建模。

use serde::{Deserialize, Serialize};

/// 阵眼（阵法）效果。
///
/// 对应接口：`GET /school/matrix`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolMatrix {
    /// 心法名称。
    pub name: String,
    /// 阵法技能名称。
    #[serde(rename = "skillName")]
    pub skill_name: String,
    /// 各等级效果描述。
    pub descs: Vec<SchoolMatrixDesc>,
}

/// 阵眼效果描述。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolMatrixDesc {
    /// 效果描述。
    pub desc: String,
    /// 等级。
    pub level: i64,
    /// 效果名称。
    pub name: String,
}

/// 心法技能分组。
///
/// 对应接口：`GET /school/skills`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolSkills {
    /// 分类名称（对应官方字段 `class`）。
    #[serde(rename = "class")]
    pub class: String,
    /// 该分类下的招式技能。
    pub data: Vec<SchoolSkillItem>,
}

/// 单个招式技能。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolSkillItem {
    /// 技能名称。
    pub name: String,
    /// 简述。
    #[serde(rename = "simpleDesc")]
    pub simple_desc: String,
    /// 完整描述。
    pub desc: String,
    /// 特殊描述。
    #[serde(rename = "specialDesc")]
    pub special_desc: String,
    /// 运功 / 冷却间隔。
    pub interval: String,
    /// 消耗。
    pub consumption: String,
    /// 距离。
    pub distance: String,
    /// 图标地址。
    pub icon: String,
    /// 技能类型。
    pub kind: String,
    /// 技能子类型。
    #[serde(rename = "subKind")]
    pub sub_kind: String,
    /// 释放类型。
    #[serde(rename = "releaseType")]
    pub release_type: String,
    /// 关联武器。
    pub weapon: String,
}

/// 某一等级的招式 / 奇穴列表。
///
/// 对应接口：`GET /school/talent`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolTalent {
    /// 等级。
    pub level: i64,
    /// 该等级的技能列表。
    pub data: Vec<SchoolTalentItem>,
}

/// 奇穴 / 招式条目。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolTalentItem {
    /// 技能名称。
    pub name: String,
    /// 类型标记（对应官方字段 `class`）：`1` 为招式，`0` 为奇穴。
    #[serde(rename = "class")]
    pub class: i64,
    /// 描述。
    pub desc: String,
    /// 图标地址。
    pub icon: String,
    /// 技能类型。
    pub kind: String,
    /// 技能子类型。
    #[serde(rename = "subKind")]
    pub sub_kind: String,
}

/// 资历榜单条目。
///
/// 对应接口：`GET /school/seniority`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolSeniority {
    /// 榜单名次说明。
    pub name: String,
    /// 角色名称。
    pub role: String,
    /// 门派名称。
    pub school: String,
    /// 服务器名称。
    pub server: String,
    /// 大区名称。
    pub zone: String,
    /// 资历值。
    pub value: i64,
    /// 角色头像地址。
    pub avatar: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    // 本域五个夹具都是「缺参数」的错误响应，因此除包络外还用文档给出的字段结构
    // 做一次模型自检，确保驼峰重命名生效。

    #[test]
    fn parses_matrix_fixture() {
        let envelope: ApiResponse<SchoolMatrix> =
            parse(include_str!("../../tests/fixtures/school_matrix.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_skills_fixture() {
        // 错误响应的 `data` 是 `{}`（不是数组），因此按原始 JSON 断言包络。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/school_skills.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_talent_fixture() {
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/school_talent.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_seniority_fixture() {
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/school_seniority.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_search_fixture() {
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/school_search.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn matrix_accepts_camel_case_fields() {
        let raw = r#"{
            "name": "冰心诀",
            "skillName": "剑气长江",
            "descs": [{"desc": "会心提高 10%", "level": 1, "name": "剑心通明"}]
        }"#;
        let matrix: SchoolMatrix = serde_json::from_str(raw).unwrap();
        assert_eq!(matrix.name, "冰心诀");
        assert_eq!(matrix.skill_name, "剑气长江");
        assert_eq!(matrix.descs[0].level, 1);
        assert_eq!(matrix.descs[0].name, "剑心通明");
    }

    #[test]
    fn skills_accept_class_keyword() {
        let raw = r#"{
            "class": "冰心诀",
            "data": [{
                "name": "剑气长江",
                "simpleDesc": "对目标造成伤害",
                "desc": "完整描述",
                "specialDesc": "特殊描述",
                "interval": "8秒",
                "consumption": "20点内力",
                "distance": "20尺",
                "icon": "https://example.com/icon.png",
                "kind": "内功",
                "subKind": "伤害",
                "releaseType": "瞬时",
                "weapon": "双兵"
            }]
        }"#;
        let skills: SchoolSkills = serde_json::from_str(raw).unwrap();
        assert_eq!(skills.class, "冰心诀");
        assert_eq!(skills.data.len(), 1);
        assert_eq!(skills.data[0].simple_desc, "对目标造成伤害");
        assert_eq!(skills.data[0].sub_kind, "伤害");
        assert_eq!(skills.data[0].release_type, "瞬时");
    }

    #[test]
    fn talent_accepts_class_keyword() {
        let raw = r#"{
            "level": 1,
            "data": [{"name": "剑心通明", "class": 0, "desc": "奇穴描述", "icon": "i.png", "kind": "奇穴", "subKind": "被动"}]
        }"#;
        let talent: SchoolTalent = serde_json::from_str(raw).unwrap();
        assert_eq!(talent.level, 1);
        assert_eq!(talent.data[0].class, 0);
        assert_eq!(talent.data[0].sub_kind, "被动");
    }

    #[test]
    fn seniority_round_trip() {
        let raw = r#"[{"name":"第 1 名","role":"漓心","school":"七秀","server":"长安城","zone":"电信区","value":142120,"avatar":"a.png"}]"#;
        let list: Vec<SchoolSeniority> = serde_json::from_str(raw).unwrap();
        assert_eq!(list[0].role, "漓心");
        assert_eq!(list[0].value, 142120);
        assert_eq!(list[0].avatar, "a.png");
    }
}
