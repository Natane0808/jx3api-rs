//! 团队招募相关数据模型。
//!
//! 对应接口：`/recruit/search`。

use serde::{Deserialize, Serialize};

/// 团队招募条目。
///
/// 对应接口：`GET /recruit/search`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecruitRecord {
    /// 活动 id。
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// 活动名称。
    #[serde(rename = "activityName")]
    pub activity_name: String,
    /// 服务类型名称。
    #[serde(rename = "typeName")]
    pub type_name: String,
    /// 服务子类型名称。
    #[serde(rename = "subTypeName")]
    pub sub_type_name: String,
    /// 团长头像 id。
    #[serde(rename = "miniAvatarId")]
    pub mini_avatar_id: i64,
    /// 创建时间戳（秒）。
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// 当前人数。
    #[serde(rename = "currentMemberCount")]
    pub current_member_count: i64,
    /// 最大人数。
    #[serde(rename = "maxMemberCount")]
    pub max_member_count: i64,
    /// 门派 id。
    #[serde(rename = "forceId")]
    pub force_id: i64,
    /// 最近修改时间戳（秒）。
    #[serde(rename = "lastModifyTime")]
    pub last_modify_time: i64,
    /// 团长角色名。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 团长体型 id。
    #[serde(rename = "roleType")]
    pub role_type: i64,
    /// 团长等级。
    #[serde(rename = "roleLevel")]
    pub role_level: i64,
    /// 心法 id。
    #[serde(rename = "kungfuId")]
    pub kungfu_id: i64,
    /// 阵营 id。
    #[serde(rename = "campId")]
    pub camp_id: i64,
    /// 标记位。
    #[serde(rename = "flagBits")]
    pub flag_bits: i64,
    /// 招募留言。
    #[serde(rename = "commentText")]
    pub comment_text: String,
    /// 标签 id 列表。
    #[serde(rename = "labelIdTable")]
    pub label_id_table: Vec<i64>,
    /// 标签备注。
    #[serde(rename = "labelIdComment")]
    pub label_id_comment: String,
    /// 跨服 / 发布者变体信息（pyi `RecruitVariant`，可能缺失）。
    #[serde(rename = "variantInfo")]
    pub variant_info: Option<RecruitVariant>,
}

/// 招募的跨服 / 发布者变体信息（pyi `RecruitVariant`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RecruitVariant {
    /// 是否切换服务器。
    pub is_switch_server: bool,
    /// 发布者 id。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub publisher_id: i64,
    /// 推送 id。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub push_id: i64,
    /// 角色 id。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub role_id: i64,
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
        // 夹具是「暂无招募数据」的 404 错误响应，只断言能解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/recruit_search.json"));
        assert!(!envelope.is_success());
        assert_eq!(envelope.code, 404);
    }

    #[test]
    fn parses_success_payload() {
        let raw = r#"{"code":200,"msg":"success","data":[
            {"activityId":"1024","activityName":"英雄冷龙峰","typeName":"团队",
             "subTypeName":"教学团","miniAvatarId":18,"createTime":1794000000,
             "currentMemberCount":10,"maxMemberCount":25,"forceId":2,
             "lastModifyTime":1794000100,"roleName":"漓心","roleType":2,
             "roleLevel":50,"kungfuId":10002,"campId":1,"flagBits":0,
             "commentText":"来熟练","labelIdTable":[1,2],"labelIdComment":"速刷",
             "variantInfo":{"isSwitchServer":false,"publisherId":1024,"pushId":2048,"roleId":29450102}}
        ]}"#;
        let envelope: ApiResponse<Vec<RecruitRecord>> = parse(raw);
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].activity_name, "英雄冷龙峰");
        assert_eq!(items[0].role_name, "漓心");
        assert_eq!(items[0].label_id_table, vec![1, 2]);
        assert_eq!(items[0].comment_text, "来熟练");
        let variant = items[0].variant_info.as_ref().expect("应解析 variantInfo");
        assert!(!variant.is_switch_server);
        assert_eq!(variant.publisher_id, 1024);
        assert_eq!(variant.push_id, 2048);
        assert_eq!(variant.role_id, 29_450_102);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: RecruitRecord = serde_json::from_str(r#"{"roleName":"漓心"}"#).unwrap();
        assert_eq!(record.role_name, "漓心");
        assert_eq!(record.current_member_count, 0);
        assert!(record.label_id_table.is_empty());
    }
}
