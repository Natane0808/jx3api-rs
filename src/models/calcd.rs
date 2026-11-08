//! 装备属性与配装记录数据模型。
//!
//! 对应接口：`/calcd/attribute`、`/calcd/detail`、`/calcd/records`。
//!
//! 这三个接口官方 Python SDK 尚未封装，模型依据 `tests/fixtures/calcd_*.json` 的
//! 真实响应推断。其中 `/calcd/attribute` 的结构极深（装备 → 属性 / 附魔 / 镶嵌 /
//! 彩石），本模块只把核心字段做成强类型，深层子结构统一用
//! [`serde_json::Value`] 兜底，避免官方微调字段就解析失败。

use serde::{Deserialize, Serialize};

/// 角色装备与属性面板。
///
/// 对应接口：`GET /calcd/attribute`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdAttribute {
    /// 大区名称，例如 `电信区`。
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    /// 服务器名称，例如 `长安城`。
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色编号。
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// 全局角色编号。
    #[serde(rename = "globalId")]
    pub global_id: String,
    /// 门派名称。
    #[serde(rename = "forceName")]
    pub force_name: String,
    /// 门派编号。
    #[serde(rename = "forceId")]
    pub force_id: i64,
    /// 心法名称。
    #[serde(rename = "kungfuName")]
    pub kungfu_name: String,
    /// 心法编号（官方返回字符串）。
    #[serde(rename = "kungfuId")]
    pub kungfu_id: String,
    /// 心法类型，例如 `DPS` / `T` / `治疗`。
    #[serde(rename = "kungfuType")]
    pub kungfu_type: String,
    /// 体型名称。
    #[serde(rename = "bodyName")]
    pub body_name: String,
    /// 体型编号。
    #[serde(rename = "bodyId")]
    pub body_id: i64,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 阵营编号。
    #[serde(rename = "campId")]
    pub camp_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
    /// 赛季名称。
    #[serde(rename = "seasonName")]
    pub season_name: String,
    /// 数据缓存时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
    /// 配装类型，例如 `秘境挑战` / `竞技对抗` / `休闲` / `通用`。
    #[serde(rename = "equipType")]
    pub equip_type: String,
    /// 装备评分。
    #[serde(rename = "equipScore")]
    pub equip_score: i64,
    /// 装备列表。
    #[serde(rename = "equipList")]
    pub equip_list: Vec<CalcdEquip>,
    /// 属性面板。
    #[serde(rename = "panelList")]
    pub panel_list: Vec<CalcdPanelEntry>,
    /// 套装效果。
    pub suits: Vec<CalcdSuit>,
    /// 附魔 / 镶嵌汇总。
    #[serde(rename = "enchantEffects")]
    pub enchant_effects: Vec<CalcdEnchantEffect>,
}

/// 单件装备。
///
/// 官方返回的装备字段多达 40 余个且层级很深，这里保留核心字段，
/// 镶嵌 / 附魔 / 彩石等深层结构用 [`serde_json::Value`] 原样承载。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdEquip {
    /// 分页编号。
    #[serde(rename = "tabId")]
    pub tab_id: i64,
    /// 分页内序号。
    #[serde(rename = "tabIndex")]
    pub tab_index: i64,
    /// 装备名称。
    #[serde(rename = "itemName")]
    pub item_name: String,
    /// 装备在列表中的序号。
    #[serde(rename = "itemIndex")]
    pub item_index: i64,
    /// 图标编号。
    #[serde(rename = "iconId")]
    pub icon_id: i64,
    /// 附魔类型组合，例如 `伤害,会心,破防,加速`。
    #[serde(rename = "magicType")]
    pub magic_type: String,
    /// 装备基础属性。
    #[serde(rename = "itemAttrib")]
    pub item_attrib: Vec<CalcdPanelEntry>,
    /// 装备绿字属性（附魔 / 精炼）。
    #[serde(rename = "itemMagic")]
    pub item_magic: Vec<CalcdItemMagic>,
    /// 需求等级。
    #[serde(rename = "needLevel")]
    pub need_level: i64,
    /// 装备等级。
    #[serde(rename = "itemLevel")]
    pub item_level: i64,
    /// 装备品质。
    #[serde(rename = "itemQuality")]
    pub item_quality: i64,
    /// 装备所属门派编号（官方返回字符串）。
    #[serde(rename = "itemSchool")]
    pub item_school: String,
    /// 装备所属门派名称。
    #[serde(rename = "schoolName")]
    pub school_name: String,
    /// 装备类型，例如 `内功`。
    #[serde(rename = "itemKind")]
    pub item_kind: String,
    /// 可装备门派编号（官方返回字符串）。
    #[serde(rename = "belongSchool")]
    pub belong_school: String,
    /// 已镶嵌五行石（结构随版本变化，保留原始 JSON）。
    #[serde(rename = "mountDiamond")]
    pub mount_diamond: Vec<serde_json::Value>,
    /// 套装编号，可能为空串。
    #[serde(rename = "setId")]
    pub set_id: String,
    /// 最低附魔（结构随版本变化，保留原始 JSON）。
    #[serde(rename = "minEnchant")]
    pub min_enchant: serde_json::Value,
    /// 最高附魔（可能为空数组或对象，结构随版本变化，保留原始 JSON）。
    #[serde(rename = "maxEnchant")]
    pub max_enchant: serde_json::Value,
    /// 精炼等级。
    #[serde(rename = "strengthLevel")]
    pub strength_level: i64,
    /// 精炼等级上限。
    #[serde(rename = "maxStrengthLevel")]
    pub max_strength_level: i64,
    /// 装备来源。
    #[serde(rename = "itemSource")]
    pub item_source: String,
    /// 装备大类。
    #[serde(rename = "itemType")]
    pub item_type: String,
    /// 装备强化标记，例如 `橙武`；普通装备可能为 `null`。
    #[serde(rename = "itemPlus")]
    pub item_plus: Option<String>,
    /// 装备描述（数组结构，保留原始 JSON）。
    #[serde(rename = "itemDesc")]
    pub item_desc: Vec<serde_json::Value>,
    /// 彩石（结构随版本变化，保留原始 JSON）。
    #[serde(rename = "colorDiamond")]
    pub color_diamond: serde_json::Value,
}

/// 装备绿字属性条目。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdItemMagic {
    /// 属性键名，例如 `atVitalityBase`。
    #[serde(rename = "typeName")]
    pub type_name: String,
    /// 属性中文名，例如 `体质`。
    #[serde(rename = "itemName")]
    pub item_name: String,
    /// 属性数值（可能带 `Param1Min` / `Param1Max` 等子字段，保留原始 JSON）。
    #[serde(rename = "itemValue")]
    pub item_value: serde_json::Value,
    /// 属性描述，可能为 `null`。
    #[serde(rename = "itemDesc")]
    pub item_desc: Option<String>,
}

/// 属性面板条目。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdPanelEntry {
    /// 属性名称，例如 `基础攻击`。
    pub name: String,
    /// 属性数值。
    pub value: i64,
    /// 百分比数值；仅会心 / 破防 / 加速等百分比类属性有该键。
    #[serde(default)]
    pub percent: Option<f64>,
}

/// 套装效果。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdSuit {
    /// 套装名称。
    pub name: String,
    /// 已激活件数。
    pub count: i64,
    /// 套装属性。
    pub attributes: Vec<CalcdSuitAttribute>,
    /// 套装激活的配方数量。
    pub recipes: i64,
}

/// 套装属性条目。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdSuitAttribute {
    /// 属性名称。
    pub name: String,
    /// 属性数值。
    pub value: i64,
    /// 激活所需件数。
    pub pieces: i64,
}

/// 附魔 / 镶嵌汇总条目。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdEnchantEffect {
    /// 装备序号。
    #[serde(rename = "itemIndex")]
    pub item_index: i64,
    /// 附魔编号。
    #[serde(rename = "enchantId")]
    pub enchant_id: i64,
    /// 附魔名称。
    pub name: String,
    /// 附魔来源，例如 `dwTemporaryEnchantID`。
    pub source: String,
    /// 解析状态，例如 `unresolved`。
    pub status: String,
    /// 是否生效，可能为 `null`。
    pub active: Option<serde_json::Value>,
    /// 附魔属性（结构随版本变化，保留原始 JSON）。
    pub attributes: Vec<serde_json::Value>,
}

/// 角色历史配装记录。
///
/// 对应接口：`GET /calcd/records`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdRecords {
    /// 大区名称。
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    /// 服务器名称。
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色编号。
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// 全局角色编号。
    #[serde(rename = "globalId")]
    pub global_id: String,
    /// 历史配装列表（按时间降序）。
    #[serde(rename = "equipList")]
    pub equip_list: Vec<CalcdRecordItem>,
}

/// 一条历史配装记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalcdRecordItem {
    /// 赛季名称。
    #[serde(rename = "seasonName")]
    pub season_name: String,
    /// 心法名称。
    #[serde(rename = "kungfuName")]
    pub kungfu_name: String,
    /// 配装类型。
    #[serde(rename = "equipType")]
    pub equip_type: String,
    /// 快照时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_attribute_fixture() {
        let envelope: ApiResponse<CalcdAttribute> =
            parse(include_str!("../../tests/fixtures/calcd_attribute.json"));
        assert!(envelope.is_success());
        let data = envelope.data.expect("data 不应为空");
        assert_eq!(data.server_name, "长安城");
        assert_eq!(data.kungfu_name, "冰心诀");
        assert_eq!(data.equip_score, 13170);
        assert_eq!(data.equip_list.len(), 12);
        assert!(!data.panel_list.is_empty());
        assert!(!data.enchant_effects.is_empty());

        let weapon = &data.equip_list[0];
        assert_eq!(weapon.item_name, "龙鲤");
        assert_eq!(weapon.item_plus.as_deref(), Some("橙武"));
        assert!(
            data.equip_list[2].item_plus.is_none(),
            "部分装备的 itemPlus 为 null"
        );
        assert!(!weapon.item_attrib.is_empty());
        assert!(weapon.min_enchant.is_object());
        assert!(weapon.color_diamond.is_object());
        assert!(
            data.equip_list[7].max_enchant.is_object(),
            "部分装备的 maxEnchant 是对象而不是数组"
        );
        assert!(data.equip_list[1].color_diamond.is_null());
        assert!(!data.suits.is_empty());
        assert_eq!(data.suits[0].attributes.len(), 2);

        // 百分比属性：percent 必须被解析出来，不能丢。
        assert_eq!(data.panel_list.len(), 24);
        let crit = &data.panel_list[4];
        assert_eq!(crit.name, "会心等级");
        assert_eq!(crit.value, 1546);
        assert_eq!(crit.percent, Some(16.25));
        assert!(
            data.panel_list[0].percent.is_none(),
            "基础攻击等非百分比属性没有 percent 键"
        );
    }

    #[test]
    fn parses_detail_fixture() {
        // 夹具是错误响应（缺 name 参数），官方未给出该接口的字段结构，
        // 因此 API 层返回原始 JSON，这里只断言包络可解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/calcd_detail.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<CalcdRecords> =
            parse(include_str!("../../tests/fixtures/calcd_records.json"));
        assert!(envelope.is_success());
        let data = envelope.data.expect("data 不应为空");
        assert_eq!(data.role_name, "漓心");
        assert_eq!(data.equip_list.len(), 4);
        assert_eq!(data.equip_list[0].season_name, "暗影千机");
        assert_eq!(data.equip_list[0].equip_type, "秘境挑战");
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let equip: CalcdEquip = serde_json::from_str(r#"{"itemName":"龙鲤"}"#).unwrap();
        assert_eq!(equip.item_name, "龙鲤");
        assert_eq!(equip.item_level, 0);
        assert!(equip.min_enchant.is_null());
    }
}
