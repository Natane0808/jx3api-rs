//! 名片相关数据模型。
//!
//! 对应接口：`/card/cached`、`/card/preset`、`/card/random`、`/card/record`、
//! `/card/records`。
//!
//! `/card/preset` 官方返回结构不固定的字典，API 层直接返回 [`serde_json::Value`]，
//! 本模块不为其建模。

use serde::{Deserialize, Serialize};

/// 已缓存的名片。
///
/// 对应接口：`GET /card/cached`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CardCached {
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
    /// 全局角色编号（跨大区唯一）。
    #[serde(rename = "globalRoleId")]
    pub global_role_id: String,
    /// 点赞数。
    #[serde(rename = "showLike")]
    pub show_like: i64,
    /// 名片哈希（用于拼图片地址）。
    #[serde(rename = "showHash")]
    pub show_hash: String,
    /// 名片序号。
    #[serde(rename = "showIndex")]
    pub show_index: i64,
    /// 名片图片地址。
    #[serde(rename = "showAvatar")]
    pub show_avatar: String,
    /// 名片文件名。
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// 缓存时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
}

/// 随机名片。
///
/// 对应接口：`GET /card/random`。
///
/// 官方 TypedDict 只声明了基础字段，实际响应还会带上 `showLike`、`showActive`
/// 等展示态字段，这里一并建模以免调用方丢失数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CardRandom {
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
    /// 全局角色编号（跨大区唯一）。
    #[serde(rename = "globalRoleId")]
    pub global_role_id: String,
    /// 点赞数。
    #[serde(rename = "showLike")]
    pub show_like: i64,
    /// 名片哈希（用于拼图片地址）。
    #[serde(rename = "showHash")]
    pub show_hash: String,
    /// 名片是否处于展示状态。
    #[serde(
        rename = "showActive",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub show_active: bool,
    /// 名片序号。
    #[serde(rename = "showIndex")]
    pub show_index: i64,
    /// 名片图片地址。
    #[serde(rename = "showAvatar")]
    pub show_avatar: String,
    /// 名片文件名。
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// 缓存时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
}

/// 角色名片。
///
/// 对应接口：`GET /card/record`，比 [`CardCached`] 多一个 `showActive` 字段。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CardRecord {
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
    /// 全局角色编号（跨大区唯一）。
    #[serde(rename = "globalRoleId")]
    pub global_role_id: String,
    /// 点赞数。
    #[serde(rename = "showLike")]
    pub show_like: i64,
    /// 名片哈希（用于拼图片地址）。
    #[serde(rename = "showHash")]
    pub show_hash: String,
    /// 名片是否处于展示状态。
    #[serde(
        rename = "showActive",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub show_active: bool,
    /// 名片序号。
    #[serde(rename = "showIndex")]
    pub show_index: i64,
    /// 名片图片地址。
    #[serde(rename = "showAvatar")]
    pub show_avatar: String,
    /// 名片文件名。
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// 缓存时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
}

/// 历史名片。
///
/// 对应接口：`GET /card/records`。
///
/// 官方 TypedDict 声明了 `lastEditTime`（最近编辑时间），但当前采样响应未返回该字段，
/// 缺失时回落为 `0`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CardHistory {
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
    /// 全局角色编号（跨大区唯一）。
    #[serde(rename = "globalRoleId")]
    pub global_role_id: String,
    /// 点赞数。
    #[serde(rename = "showLike")]
    pub show_like: i64,
    /// 名片哈希（用于拼图片地址）。
    #[serde(rename = "showHash")]
    pub show_hash: String,
    /// 名片是否处于展示状态。
    #[serde(
        rename = "showActive",
        deserialize_with = "crate::serde_util::flexible_bool"
    )]
    pub show_active: bool,
    /// 名片序号。
    #[serde(rename = "showIndex")]
    pub show_index: i64,
    /// 名片图片地址。
    #[serde(rename = "showAvatar")]
    pub show_avatar: String,
    /// 名片文件名。
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// 缓存时间戳（Unix 秒）。
    #[serde(rename = "cacheTime")]
    pub cache_time: i64,
    /// 最近编辑时间戳（Unix 秒），部分响应可能不返回。
    #[serde(rename = "lastEditTime")]
    pub last_edit_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_cached_fixture() {
        // 该夹具是错误响应（缺 name 参数），只断言包络可解析。
        let envelope: ApiResponse<CardCached> =
            parse(include_str!("../../tests/fixtures/card_cached.json"));
        assert_eq!(envelope.code, 400);
        assert!(envelope.data.is_some());
    }

    #[test]
    fn parses_preset_fixture() {
        // `/card/preset` 返回裸字典，夹具同样是错误响应。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/card_preset.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_random_fixture() {
        let envelope: ApiResponse<CardRandom> =
            parse(include_str!("../../tests/fixtures/card_random.json"));
        assert!(envelope.is_success());
        let card = envelope.data.expect("data 不应为空");
        assert_eq!(card.zone_name, "电信区");
        assert_eq!(card.role_name, "流霜飞雪");
        assert_eq!(card.global_role_id, "396316767215296990");
        assert_eq!(card.show_like, 2165);
        assert!(card.show_active);
        assert!(!card.show_avatar.is_empty());
    }

    #[test]
    fn parses_record_fixture() {
        let envelope: ApiResponse<CardRecord> =
            parse(include_str!("../../tests/fixtures/card_record.json"));
        assert!(envelope.is_success());
        let card = envelope.data.expect("data 不应为空");
        assert_eq!(card.server_name, "长安城");
        assert_eq!(card.show_index, 2);
        assert!(card.show_active);
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<Vec<CardHistory>> =
            parse(include_str!("../../tests/fixtures/card_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].show_index, 1);
        assert!(!records[0].show_active);
        assert_eq!(
            records[2].file_name,
            "87452d6f21bb6cc5ee87c21b9b8559f8b5f61c5a3602bc18410.png"
        );
        // 采样响应未返回 lastEditTime，缺失时回落为 0。
        assert_eq!(records[0].last_edit_time, 0);
    }
}
