//! WebSocket 事件的 `detail` 数据模型。
//!
//! 与 `openapi/openapi.socket.json` 中 `/socket/<action>` 的 `parameters`
//! 一一对应：每个 action 一个结构体，字段名保持服务端下发的原始命名。
//! 所有结构体都实现了 [`Default`] 并忽略未知字段，服务端字段增删不会导致解析失败。

use serde::{Deserialize, Serialize};

/// 宽松解析字符串数组：接受 JSON 字符串数组与 `null`。
fn flexible_vec_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<Vec<String>>::deserialize(deserializer)?;
    Ok(value.unwrap_or_default())
}

/// 奇遇触发（action = `1001`，分类：奇遇事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventTriggerDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名/事件名。
    pub name: String,
    /// 奇遇名。
    pub event: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 马驹刷新（action = `1002`，分类：马场事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HorseRefreshDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 地图名。
    pub map_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 马驹捕获（action = `1003`，分类：马场事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HorseCaptureDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名/事件名。
    pub name: String,
    /// 地图名。
    pub map_name: String,
    /// 对应字段 `level`（官方规范未提供说明）。
    pub level: String,
    /// 对应字段 `horse`（官方规范未提供说明）。
    pub horse: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 扶摇开启（action = `1005`，分类：扶摇事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FuyaoOpenDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 扶摇点名（action = `1006`，分类：扶摇事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FuyaoRollDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名/事件名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub name: Vec<String>,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 的卢每日（action = `1008`，分类：的卢事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiluDailyDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 地图名。
    pub map_name: String,
    /// 角色名/事件名。
    pub name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 的卢刷新（action = `1009`，分类：的卢事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiluRefreshDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 地图名。
    pub map_name: String,
    /// 刷新时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub refresh_time: i64,
}

/// 的卢捕获（action = `1010`，分类：的卢事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiluCaptureDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `capture_camp_name`（官方规范未提供说明）。
    pub capture_camp_name: String,
    /// 对应字段 `capture_role_name`（官方规范未提供说明）。
    pub capture_role_name: String,
    /// 地图名。
    pub map_name: String,
    /// 捕获时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub capture_time: i64,
}

/// 的卢拍卖（action = `1011`，分类：的卢事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiluAuctionDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `auction_camp_name`（官方规范未提供说明）。
    pub auction_camp_name: String,
    /// 对应字段 `auction_role_name`（官方规范未提供说明）。
    pub auction_role_name: String,
    /// 对应字段 `auction_amount`（官方规范未提供说明）。
    pub auction_amount: String,
    /// 拍卖时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub auction_time: i64,
}

/// 副本掉落 / 私货（action = `1012`，分类：掉落交易）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DungeonDropDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名。
    pub role_name: String,
    /// 地图名。
    pub map_name: String,
    /// 物品名。
    pub item_name: String,
    /// 成交/价格。
    pub item_amount: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 阵营拍卖（action = `1013`，分类：掉落交易）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CampAuctionDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名。
    pub role_name: String,
    /// 阵营。
    pub camp_name: String,
    /// 物品名。
    pub item_name: String,
    /// 成交/价格。
    pub item_amount: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 诛恶事件（action = `1014`，分类：江湖事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PunishEvilDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 地图名。
    pub map_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 追魂点名（action = `1015`，分类：江湖事件）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SoulChaseRollDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 角色名。
    pub role_name: String,
    /// 角色所在服务器。
    pub role_server: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 阵营祭祀（action = `1017`，分类：阵营祭祀）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CampSacrificeDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    pub tong_name: String,
    /// 角色名。
    pub role_name: String,
    /// 据点名。
    pub castle_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 关隘首领（action = `1018`，分类：关隘首领）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PassLeaderDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 首领名。
    pub leader_name: String,
    /// 阵营。
    pub camp_name: String,
    /// 关隘名。
    pub stage_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 领地宣战·开始（action = `1101`，分类：宣战约战）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TerritoryWarStartDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `battlefield_type`（官方规范未提供说明）。
    pub battlefield_type: String,
    /// 对应字段 `declaring_tong_name`（官方规范未提供说明）。
    pub declaring_tong_name: String,
    /// 对应字段 `accepting_tong_name`（官方规范未提供说明）。
    pub accepting_tong_name: String,
    /// 对应字段 `battlefield_tong_name`（官方规范未提供说明）。
    pub battlefield_tong_name: String,
    /// 开始时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub start_time: i64,
}

/// 领地宣战·结束（action = `1102`，分类：宣战约战）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TerritoryWarEndDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `battlefield_type`（官方规范未提供说明）。
    pub battlefield_type: String,
    /// 对应字段 `declaring_tong_name`（官方规范未提供说明）。
    pub declaring_tong_name: String,
    /// 对应字段 `accepting_tong_name`（官方规范未提供说明）。
    pub accepting_tong_name: String,
    /// 对应字段 `battlefield_tong_name`（官方规范未提供说明）。
    pub battlefield_tong_name: String,
    /// 对应字段 `victory_tong_name`（官方规范未提供说明）。
    pub victory_tong_name: String,
    /// 对应字段 `victory_score`（官方规范未提供说明）。
    pub victory_score: String,
    /// 结束时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub end_time: i64,
}

/// 帮会宣战·开始（action = `1103`，分类：宣战约战）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GuildWarStartDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `battlefield_type`（官方规范未提供说明）。
    pub battlefield_type: String,
    /// 对应字段 `declaring_tong_name`（官方规范未提供说明）。
    pub declaring_tong_name: String,
    /// 对应字段 `accepting_tong_name`（官方规范未提供说明）。
    pub accepting_tong_name: String,
    /// 对应字段 `duration_hours`（官方规范未提供说明）。
    pub duration_hours: String,
    /// 开始时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub start_time: i64,
}

/// 帮会宣战·结束（action = `1104`，分类：宣战约战）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GuildWarEndDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `battlefield_type`（官方规范未提供说明）。
    pub battlefield_type: String,
    /// 对应字段 `declaring_tong_name`（官方规范未提供说明）。
    pub declaring_tong_name: String,
    /// 对应字段 `accepting_tong_name`（官方规范未提供说明）。
    pub accepting_tong_name: String,
    /// 结束时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub end_time: i64,
}

/// 帮会约战·完胜（action = `1105`，分类：宣战约战）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GuildDuelVictoryDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 对应字段 `challenge_tong_name`（官方规范未提供说明）。
    pub challenge_tong_name: String,
    /// 对应字段 `opponent_tong_name`（官方规范未提供说明）。
    pub opponent_tong_name: String,
    /// 对应字段 `victory_tong_name`（官方规范未提供说明）。
    pub victory_tong_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 抢占粮仓（action = `1111`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GranaryCaptureDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 据点名。
    pub castle_name: String,
    /// 阵营。
    pub camp_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 大旗重置（action = `1112`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BannerResetDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 据点名。
    pub castle_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 大旗被夺（action = `1113`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BannerSeizedDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 地图名。
    pub map_name: String,
    /// 据点名。
    pub castle_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 据点占领（action = `1114`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StrongholdCaptureDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    pub tong_name: String,
    /// 据点名。
    pub castle_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 据点占领（无帮会）（action = `1115`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StrongholdCaptureNoGuildDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 据点名。
    pub castle_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 小攻防贡献（非开战）（action = `1116`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MinorAttackContributionIdleDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_name: Vec<String>,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 小攻防贡献（action = `1117`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MinorAttackContributionDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_name: Vec<String>,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 大攻防贡献（action = `1118`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MajorAttackContributionDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_name: Vec<String>,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 战利品竞拍（action = `1119`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LootAuctionDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 角色名。
    pub role_name: String,
    /// 成交/价格。
    pub item_amount: String,
    /// 物品名。
    pub item_name: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 小攻防分红（action = `1120`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MinorAttackDividendDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_name: Vec<String>,
    /// 对应字段 `split_amount`（官方规范未提供说明）。
    pub split_amount: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 大攻防分红（action = `1121`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MajorAttackDividendDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_name: Vec<String>,
    /// 对应字段 `split_amount`（官方规范未提供说明）。
    pub split_amount: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 大攻防分红（含指挥）（action = `1122`，分类：据点攻防）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MajorAttackDividendChiefDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 阵营。
    pub camp_name: String,
    /// 帮会名列表。
    #[serde(deserialize_with = "flexible_vec_string")]
    pub tong_names: Vec<String>,
    /// 对应字段 `chief_tong_name`（官方规范未提供说明）。
    pub chief_tong_name: String,
    /// 对应字段 `split_amount`（官方规范未提供说明）。
    pub split_amount: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 微博更新（action = `1201`，分类：社交动态）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WeiboUpdateDetail {
    /// 微博用户。
    pub user_name: String,
    /// 用户ID。
    pub user_id: String,
    /// 微博ID。
    pub article_id: String,
    /// 正文。
    pub article_text: String,
    /// 发布时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub created_at: i64,
    /// 链接。
    pub url: String,
}

/// 开服状态（action = `2001`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerStatusDetail {
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 开服状态(0关/1开)。
    pub status: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}

/// 官方新闻（action = `2002`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewsUpdateDetail {
    /// 类型。
    pub r#type: String,
    /// 标题。
    pub title: String,
    /// 链接。
    pub url: String,
    /// 日期。
    pub date: String,
}

/// 版本更新（action = `2003`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VersionUpdateDetail {
    /// 当前版本。
    pub now_version: String,
    /// 最新版本。
    pub new_version: String,
    /// 更新包数。
    pub package_num: String,
    /// 更新大小。
    pub package_size: String,
}

/// 八卦速报（action = `2004`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GossipNewsDetail {
    /// 分类。
    pub tags: String,
    /// 大区。
    pub zone: String,
    /// 服务器。
    pub server: String,
    /// 贴吧名。
    pub tieba: String,
    /// 标题。
    pub title: String,
    /// 链接。
    pub url: String,
    /// 日期。
    pub date: String,
}

/// 关隘首领（action = `2005`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StrongholdCooldownResetDetail {
    /// 服务器。
    pub server: String,
    /// 据点/关卡。
    pub stage: String,
    /// 开始时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub start: i64,
}

/// 云从预告（action = `2006`，分类：系统通知）的 `detail` 明细。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MapEventPreviewDetail {
    /// 角色名/事件名。
    pub name: String,
    /// 地点。
    pub site: String,
    /// 描述。
    pub desc: String,
    /// 事件时间戳。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub time: i64,
}
