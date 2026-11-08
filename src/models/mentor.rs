//! 师徒列表相关数据模型。
//!
//! 对应接口：`/mentor/search`。

use serde::{Deserialize, Serialize};

/// 师徒招募条目。
///
/// 对应接口：`GET /mentor/search`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MentorRecord {
    /// 门派 id。
    #[serde(rename = "forceId")]
    pub force_id: i64,
    /// 门派名称。
    #[serde(rename = "forceName")]
    pub force_name: String,
    /// 地图 id。
    #[serde(rename = "mapId")]
    pub map_id: i64,
    /// 头像 id（官方字段名拼写为 `miniAvatorId`）。
    #[serde(rename = "miniAvatorId")]
    pub mini_avator_id: i64,
    /// 角色 id。
    #[serde(rename = "roleId")]
    pub role_id: i64,
    /// 体型 id。
    #[serde(rename = "bodyId")]
    pub body_id: i64,
    /// 体型名称。
    #[serde(rename = "bodyName")]
    pub body_name: String,
    /// 阵营 id。
    #[serde(rename = "campId")]
    pub camp_id: i64,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 角色等级。
    #[serde(rename = "roleLevel")]
    pub role_level: i64,
    /// 帮会人数。
    #[serde(rename = "tongMemberCount")]
    pub tong_member_count: i64,
    /// 招募留言。
    #[serde(rename = "commentText")]
    pub comment_text: String,
    /// 角色名称。
    #[serde(rename = "roleName")]
    pub role_name: String,
    /// 帮主名称。
    #[serde(rename = "tongMasterName")]
    pub tong_master_name: String,
    /// 帮会 id。
    #[serde(rename = "tongId")]
    pub tong_id: i64,
    /// 帮会名称。
    #[serde(rename = "tongName")]
    pub tong_name: String,
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
        let envelope: ApiResponse<Vec<MentorRecord>> =
            parse(include_str!("../../tests/fixtures/mentor_search.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        let first = &items[0];
        assert_eq!(first.force_name, "少林");
        assert_eq!(first.role_name, "黑本大王");
        assert_eq!(first.body_name, "成男");
        assert_eq!(first.camp_name, "浩气盟");
        assert!(!first.comment_text.is_empty());
        assert!(first.tong_member_count > 0);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: MentorRecord = serde_json::from_str(r#"{"roleName":"漓心"}"#).unwrap();
        assert_eq!(record.role_name, "漓心");
        assert_eq!(record.force_id, 0);
        assert!(record.comment_text.is_empty());
    }
}
