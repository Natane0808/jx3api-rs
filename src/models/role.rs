//! 角色相关数据模型。
//!
//! 对应接口：`/role/detail`、`/role/achievement`。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 角色名片。
///
/// 对应接口：`GET /role/detail`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoleDetailed {
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
    /// 体型名称。
    #[serde(rename = "bodyName")]
    pub body_name: String,
    /// 体型编号。
    #[serde(rename = "bodyId")]
    pub body_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
    /// 帮会编号。
    #[serde(rename = "tongId")]
    pub tong_id: i64,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 阵营编号。
    #[serde(rename = "campId")]
    pub camp_id: i64,
}

/// 角色成就查询结果。
///
/// 对应接口：`GET /role/achievement`。
///
/// 官方结构是 `total=False` 的超宽字典，因此这里只给名片与成就列表做强类型，
/// 其余未知字段统一收进 [`RoleAchievement::extra`]，避免官方加字段就整体丢数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoleAchievement {
    /// 大区名称，例如 `电信区`。
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    /// 服务器名称，例如 `长安城`。
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 角色编号（官方可能返回字符串或数字）。
    #[serde(rename = "roleId")]
    pub role_id: serde_json::Value,
    /// 全局角色编号（官方可能返回字符串或数字）。
    #[serde(rename = "globalRoleId")]
    pub global_role_id: serde_json::Value,
    /// 门派名称。
    #[serde(rename = "forceName")]
    pub force_name: String,
    /// 门派编号。
    #[serde(rename = "forceId")]
    pub force_id: i64,
    /// 体型名称。
    #[serde(rename = "bodyName")]
    pub body_name: String,
    /// 体型编号。
    #[serde(rename = "bodyId")]
    pub body_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
    /// 帮会编号。
    #[serde(rename = "tongId")]
    pub tong_id: i64,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 阵营编号。
    #[serde(rename = "campId")]
    pub camp_id: i64,
    /// 玩家昵称（推栏名称）。
    #[serde(rename = "personName")]
    pub person_name: String,
    /// 玩家编号。
    #[serde(rename = "personId")]
    pub person_id: String,
    /// 玩家头像地址。
    #[serde(rename = "personAvatar")]
    pub person_avatar: String,
    /// 命中的成就列表。
    pub data: Vec<RoleAchievementItem>,
    /// 官方新增的未知字段，原样保留。
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 单条成就。
///
/// 与 [`RoleAchievement`] 一样是 `total=False` 结构，未知字段收进
/// [`RoleAchievementItem::extra`]。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoleAchievementItem {
    /// 成就编号。
    pub id: i64,
    /// 成就图标地址。
    pub icon: String,
    /// 点赞数。
    pub likes: i64,
    /// 成就名称。
    pub name: String,
    /// 成就分类，例如 `江湖`。
    pub class: String,
    /// 成就子类。
    #[serde(rename = "subClass")]
    pub sub_class: String,
    /// 成就描述。
    pub desc: String,
    /// 成就详情。
    pub detail: String,
    /// 可完成地图（结构不固定，保留原始 JSON）。
    pub maps: Vec<serde_json::Value>,
    /// 角色是否已完成该成就。
    #[serde(
        rename = "isFinished",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub is_finished: bool,
    /// 是否已收藏。
    #[serde(
        rename = "isFav",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub is_fav: bool,
    /// 成就类型。
    #[serde(rename = "type")]
    pub kind: String,
    /// 当前进度值。
    #[serde(rename = "currentValue")]
    pub current_value: i64,
    /// 触发进度值。
    #[serde(rename = "triggerValue")]
    pub trigger_value: i64,
    /// 成就子集（结构不固定，保留原始 JSON）。
    pub subset: Vec<serde_json::Value>,
    /// 奖励物品。
    #[serde(rename = "rewardItem")]
    pub reward_item: String,
    /// 奖励资历点。
    #[serde(rename = "rewardPoint")]
    pub reward_point: i64,
    /// 奖励称号前缀。
    #[serde(rename = "rewardPrefix")]
    pub reward_prefix: String,
    /// 奖励称号后缀。
    #[serde(rename = "rewardSuffix")]
    pub reward_suffix: String,
    /// 官方新增的未知字段，原样保留。
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_detail_fixture() {
        let envelope: ApiResponse<RoleDetailed> =
            parse(include_str!("../../tests/fixtures/role_detail.json"));
        assert!(envelope.is_success());
        let detail = envelope.data.expect("data 不应为空");
        assert_eq!(detail.server_name, "长安城");
        assert_eq!(detail.role_name, "漓心");
        assert_eq!(detail.force_name, "七秀");
        assert_eq!(detail.camp_name, "恶人谷");
        assert_eq!(detail.tong_id, 16900);
    }

    #[test]
    fn parses_achievement_fixture() {
        // 夹具是错误响应（缺 name 参数），只断言包络可解析。
        let envelope: ApiResponse<RoleAchievement> =
            parse(include_str!("../../tests/fixtures/role_achievement.json"));
        assert_eq!(envelope.code, 400);
        let achievement = envelope.data.expect("data 不应为空");
        assert!(achievement.data.is_empty());
    }

    #[test]
    fn achievement_keeps_unknown_fields() {
        let raw = r#"{
            "zoneName": "电信区",
            "serverName": "长安城",
            "roleName": "漓心",
            "roleId": 29450102,
            "globalRoleId": "4863887597560921611",
            "personName": "漓心",
            "data": [
                {
                    "id": 1,
                    "name": "初入江湖",
                    "class": "江湖",
                    "subClass": "阅历",
                    "isFinished": true,
                    "triggerValue": 1,
                    "rewardSuffix": "·初",
                    "futureField": 42
                }
            ],
            "futureTopField": "保留"
        }"#;
        let achievement: RoleAchievement = serde_json::from_str(raw).unwrap();
        assert_eq!(achievement.zone_name, "电信区");
        assert_eq!(achievement.role_id, serde_json::json!(29450102));
        assert_eq!(achievement.data.len(), 1);
        assert_eq!(achievement.data[0].sub_class, "阅历");
        assert_eq!(achievement.data[0].reward_suffix, "·初");
        assert!(achievement.data[0].is_finished);
        assert_eq!(
            achievement.data[0].extra["futureField"],
            serde_json::json!(42)
        );
        assert_eq!(
            achievement.extra["futureTopField"],
            serde_json::json!("保留")
        );
    }
}
