//! WebSocket 事件类型、消息包络与强类型派发。
//!
//! 服务端下发的每条消息都是一个 JSON 对象：
//!
//! ```json
//! { "action": 1001, "status": "success", "detail": { "zone": "电信一区", "server": "长安城" } }
//! ```
//!
//! [`SocketEnvelope`] 负责承载原始包络，[`SocketEvent`] 按 `action` 把 `detail`
//! 解析成对应的强类型结构体，[`SocketAction`] 则是全部 39 个事件的枚举。

use std::fmt;
use std::str::FromStr;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::socket::actions::*;
use crate::socket::error::SocketError;

/// WebSocket 推送的全部 action。
///
/// 变体与 `openapi/openapi.socket.json` 的 `x-action` 一一对应（共 39 个），
/// 可以用 [`SocketAction::ALL`] 遍历，用 [`SocketAction::as_u32`] /
/// [`SocketAction::as_str`] 取回服务端使用的数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum SocketAction {
    /// 奇遇触发（action = `1001`）。
    EventTrigger,
    /// 马驹刷新（action = `1002`）。
    HorseRefresh,
    /// 马驹捕获（action = `1003`）。
    HorseCapture,
    /// 扶摇开启（action = `1005`）。
    FuyaoOpen,
    /// 扶摇点名（action = `1006`）。
    FuyaoRoll,
    /// 的卢每日（action = `1008`）。
    DiluDaily,
    /// 的卢刷新（action = `1009`）。
    DiluRefresh,
    /// 的卢捕获（action = `1010`）。
    DiluCapture,
    /// 的卢拍卖（action = `1011`）。
    DiluAuction,
    /// 副本掉落 / 私货（action = `1012`）。
    DungeonDrop,
    /// 阵营拍卖（action = `1013`）。
    CampAuction,
    /// 诛恶事件（action = `1014`）。
    PunishEvil,
    /// 追魂点名（action = `1015`）。
    SoulChaseRoll,
    /// 阵营祭祀（action = `1017`）。
    CampSacrifice,
    /// 关隘首领（action = `1018`）。
    PassLeader,
    /// 领地宣战·开始（action = `1101`）。
    TerritoryWarStart,
    /// 领地宣战·结束（action = `1102`）。
    TerritoryWarEnd,
    /// 帮会宣战·开始（action = `1103`）。
    GuildWarStart,
    /// 帮会宣战·结束（action = `1104`）。
    GuildWarEnd,
    /// 帮会约战·完胜（action = `1105`）。
    GuildDuelVictory,
    /// 抢占粮仓（action = `1111`）。
    GranaryCapture,
    /// 大旗重置（action = `1112`）。
    BannerReset,
    /// 大旗被夺（action = `1113`）。
    BannerSeized,
    /// 据点占领（action = `1114`）。
    StrongholdCapture,
    /// 据点占领（无帮会）（action = `1115`）。
    StrongholdCaptureNoGuild,
    /// 小攻防贡献（非开战）（action = `1116`）。
    MinorAttackContributionIdle,
    /// 小攻防贡献（action = `1117`）。
    MinorAttackContribution,
    /// 大攻防贡献（action = `1118`）。
    MajorAttackContribution,
    /// 战利品竞拍（action = `1119`）。
    LootAuction,
    /// 小攻防分红（action = `1120`）。
    MinorAttackDividend,
    /// 大攻防分红（action = `1121`）。
    MajorAttackDividend,
    /// 大攻防分红（含指挥）（action = `1122`）。
    MajorAttackDividendChief,
    /// 微博更新（action = `1201`）。
    WeiboUpdate,
    /// 开服状态（action = `2001`）。
    ServerStatus,
    /// 官方新闻（action = `2002`）。
    NewsUpdate,
    /// 版本更新（action = `2003`）。
    VersionUpdate,
    /// 八卦速报（action = `2004`）。
    GossipNews,
    /// 关隘首领（action = `2005`）。
    StrongholdCooldownReset,
    /// 云从预告（action = `2006`）。
    MapEventPreview,
}

impl SocketAction {
    /// 官方规范中的全部 39 个 action（按 action 数值升序）。
    pub const ALL: [Self; 39] = [
        Self::EventTrigger,
        Self::HorseRefresh,
        Self::HorseCapture,
        Self::FuyaoOpen,
        Self::FuyaoRoll,
        Self::DiluDaily,
        Self::DiluRefresh,
        Self::DiluCapture,
        Self::DiluAuction,
        Self::DungeonDrop,
        Self::CampAuction,
        Self::PunishEvil,
        Self::SoulChaseRoll,
        Self::CampSacrifice,
        Self::PassLeader,
        Self::TerritoryWarStart,
        Self::TerritoryWarEnd,
        Self::GuildWarStart,
        Self::GuildWarEnd,
        Self::GuildDuelVictory,
        Self::GranaryCapture,
        Self::BannerReset,
        Self::BannerSeized,
        Self::StrongholdCapture,
        Self::StrongholdCaptureNoGuild,
        Self::MinorAttackContributionIdle,
        Self::MinorAttackContribution,
        Self::MajorAttackContribution,
        Self::LootAuction,
        Self::MinorAttackDividend,
        Self::MajorAttackDividend,
        Self::MajorAttackDividendChief,
        Self::WeiboUpdate,
        Self::ServerStatus,
        Self::NewsUpdate,
        Self::VersionUpdate,
        Self::GossipNews,
        Self::StrongholdCooldownReset,
        Self::MapEventPreview,
    ];

    /// 取得 action 的数值（与官方 `x-action` 完全一致）。
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::EventTrigger => 1001,
            Self::HorseRefresh => 1002,
            Self::HorseCapture => 1003,
            Self::FuyaoOpen => 1005,
            Self::FuyaoRoll => 1006,
            Self::DiluDaily => 1008,
            Self::DiluRefresh => 1009,
            Self::DiluCapture => 1010,
            Self::DiluAuction => 1011,
            Self::DungeonDrop => 1012,
            Self::CampAuction => 1013,
            Self::PunishEvil => 1014,
            Self::SoulChaseRoll => 1015,
            Self::CampSacrifice => 1017,
            Self::PassLeader => 1018,
            Self::TerritoryWarStart => 1101,
            Self::TerritoryWarEnd => 1102,
            Self::GuildWarStart => 1103,
            Self::GuildWarEnd => 1104,
            Self::GuildDuelVictory => 1105,
            Self::GranaryCapture => 1111,
            Self::BannerReset => 1112,
            Self::BannerSeized => 1113,
            Self::StrongholdCapture => 1114,
            Self::StrongholdCaptureNoGuild => 1115,
            Self::MinorAttackContributionIdle => 1116,
            Self::MinorAttackContribution => 1117,
            Self::MajorAttackContribution => 1118,
            Self::LootAuction => 1119,
            Self::MinorAttackDividend => 1120,
            Self::MajorAttackDividend => 1121,
            Self::MajorAttackDividendChief => 1122,
            Self::WeiboUpdate => 1201,
            Self::ServerStatus => 2001,
            Self::NewsUpdate => 2002,
            Self::VersionUpdate => 2003,
            Self::GossipNews => 2004,
            Self::StrongholdCooldownReset => 2005,
            Self::MapEventPreview => 2006,
        }
    }

    /// 取得 action 的十进制字符串形式。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventTrigger => "1001",
            Self::HorseRefresh => "1002",
            Self::HorseCapture => "1003",
            Self::FuyaoOpen => "1005",
            Self::FuyaoRoll => "1006",
            Self::DiluDaily => "1008",
            Self::DiluRefresh => "1009",
            Self::DiluCapture => "1010",
            Self::DiluAuction => "1011",
            Self::DungeonDrop => "1012",
            Self::CampAuction => "1013",
            Self::PunishEvil => "1014",
            Self::SoulChaseRoll => "1015",
            Self::CampSacrifice => "1017",
            Self::PassLeader => "1018",
            Self::TerritoryWarStart => "1101",
            Self::TerritoryWarEnd => "1102",
            Self::GuildWarStart => "1103",
            Self::GuildWarEnd => "1104",
            Self::GuildDuelVictory => "1105",
            Self::GranaryCapture => "1111",
            Self::BannerReset => "1112",
            Self::BannerSeized => "1113",
            Self::StrongholdCapture => "1114",
            Self::StrongholdCaptureNoGuild => "1115",
            Self::MinorAttackContributionIdle => "1116",
            Self::MinorAttackContribution => "1117",
            Self::MajorAttackContribution => "1118",
            Self::LootAuction => "1119",
            Self::MinorAttackDividend => "1120",
            Self::MajorAttackDividend => "1121",
            Self::MajorAttackDividendChief => "1122",
            Self::WeiboUpdate => "1201",
            Self::ServerStatus => "2001",
            Self::NewsUpdate => "2002",
            Self::VersionUpdate => "2003",
            Self::GossipNews => "2004",
            Self::StrongholdCooldownReset => "2005",
            Self::MapEventPreview => "2006",
        }
    }

    /// 根据数值查找 action；未知数值返回 `None`。
    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            1001 => Some(Self::EventTrigger),
            1002 => Some(Self::HorseRefresh),
            1003 => Some(Self::HorseCapture),
            1005 => Some(Self::FuyaoOpen),
            1006 => Some(Self::FuyaoRoll),
            1008 => Some(Self::DiluDaily),
            1009 => Some(Self::DiluRefresh),
            1010 => Some(Self::DiluCapture),
            1011 => Some(Self::DiluAuction),
            1012 => Some(Self::DungeonDrop),
            1013 => Some(Self::CampAuction),
            1014 => Some(Self::PunishEvil),
            1015 => Some(Self::SoulChaseRoll),
            1017 => Some(Self::CampSacrifice),
            1018 => Some(Self::PassLeader),
            1101 => Some(Self::TerritoryWarStart),
            1102 => Some(Self::TerritoryWarEnd),
            1103 => Some(Self::GuildWarStart),
            1104 => Some(Self::GuildWarEnd),
            1105 => Some(Self::GuildDuelVictory),
            1111 => Some(Self::GranaryCapture),
            1112 => Some(Self::BannerReset),
            1113 => Some(Self::BannerSeized),
            1114 => Some(Self::StrongholdCapture),
            1115 => Some(Self::StrongholdCaptureNoGuild),
            1116 => Some(Self::MinorAttackContributionIdle),
            1117 => Some(Self::MinorAttackContribution),
            1118 => Some(Self::MajorAttackContribution),
            1119 => Some(Self::LootAuction),
            1120 => Some(Self::MinorAttackDividend),
            1121 => Some(Self::MajorAttackDividend),
            1122 => Some(Self::MajorAttackDividendChief),
            1201 => Some(Self::WeiboUpdate),
            2001 => Some(Self::ServerStatus),
            2002 => Some(Self::NewsUpdate),
            2003 => Some(Self::VersionUpdate),
            2004 => Some(Self::GossipNews),
            2005 => Some(Self::StrongholdCooldownReset),
            2006 => Some(Self::MapEventPreview),
            _ => None,
        }
    }

    /// 中文事件名（取自官方 `summary`）。
    pub const fn name(self) -> &'static str {
        match self {
            Self::EventTrigger => "奇遇触发",
            Self::HorseRefresh => "马驹刷新",
            Self::HorseCapture => "马驹捕获",
            Self::FuyaoOpen => "扶摇开启",
            Self::FuyaoRoll => "扶摇点名",
            Self::DiluDaily => "的卢每日",
            Self::DiluRefresh => "的卢刷新",
            Self::DiluCapture => "的卢捕获",
            Self::DiluAuction => "的卢拍卖",
            Self::DungeonDrop => "副本掉落 / 私货",
            Self::CampAuction => "阵营拍卖",
            Self::PunishEvil => "诛恶事件",
            Self::SoulChaseRoll => "追魂点名",
            Self::CampSacrifice => "阵营祭祀",
            Self::PassLeader => "关隘首领",
            Self::TerritoryWarStart => "领地宣战·开始",
            Self::TerritoryWarEnd => "领地宣战·结束",
            Self::GuildWarStart => "帮会宣战·开始",
            Self::GuildWarEnd => "帮会宣战·结束",
            Self::GuildDuelVictory => "帮会约战·完胜",
            Self::GranaryCapture => "抢占粮仓",
            Self::BannerReset => "大旗重置",
            Self::BannerSeized => "大旗被夺",
            Self::StrongholdCapture => "据点占领",
            Self::StrongholdCaptureNoGuild => "据点占领（无帮会）",
            Self::MinorAttackContributionIdle => "小攻防贡献（非开战）",
            Self::MinorAttackContribution => "小攻防贡献",
            Self::MajorAttackContribution => "大攻防贡献",
            Self::LootAuction => "战利品竞拍",
            Self::MinorAttackDividend => "小攻防分红",
            Self::MajorAttackDividend => "大攻防分红",
            Self::MajorAttackDividendChief => "大攻防分红（含指挥）",
            Self::WeiboUpdate => "微博更新",
            Self::ServerStatus => "开服状态",
            Self::NewsUpdate => "官方新闻",
            Self::VersionUpdate => "版本更新",
            Self::GossipNews => "八卦速报",
            Self::StrongholdCooldownReset => "关隘首领",
            Self::MapEventPreview => "云从预告",
        }
    }

    /// 中文分类（取自官方 `tags[0]`）。
    pub const fn category(self) -> &'static str {
        match self {
            Self::EventTrigger => "奇遇事件",
            Self::HorseRefresh => "马场事件",
            Self::HorseCapture => "马场事件",
            Self::FuyaoOpen => "扶摇事件",
            Self::FuyaoRoll => "扶摇事件",
            Self::DiluDaily => "的卢事件",
            Self::DiluRefresh => "的卢事件",
            Self::DiluCapture => "的卢事件",
            Self::DiluAuction => "的卢事件",
            Self::DungeonDrop => "掉落交易",
            Self::CampAuction => "掉落交易",
            Self::PunishEvil => "江湖事件",
            Self::SoulChaseRoll => "江湖事件",
            Self::CampSacrifice => "阵营祭祀",
            Self::PassLeader => "关隘首领",
            Self::TerritoryWarStart => "宣战约战",
            Self::TerritoryWarEnd => "宣战约战",
            Self::GuildWarStart => "宣战约战",
            Self::GuildWarEnd => "宣战约战",
            Self::GuildDuelVictory => "宣战约战",
            Self::GranaryCapture => "据点攻防",
            Self::BannerReset => "据点攻防",
            Self::BannerSeized => "据点攻防",
            Self::StrongholdCapture => "据点攻防",
            Self::StrongholdCaptureNoGuild => "据点攻防",
            Self::MinorAttackContributionIdle => "据点攻防",
            Self::MinorAttackContribution => "据点攻防",
            Self::MajorAttackContribution => "据点攻防",
            Self::LootAuction => "据点攻防",
            Self::MinorAttackDividend => "据点攻防",
            Self::MajorAttackDividend => "据点攻防",
            Self::MajorAttackDividendChief => "据点攻防",
            Self::WeiboUpdate => "社交动态",
            Self::ServerStatus => "系统通知",
            Self::NewsUpdate => "系统通知",
            Self::VersionUpdate => "系统通知",
            Self::GossipNews => "系统通知",
            Self::StrongholdCooldownReset => "系统通知",
            Self::MapEventPreview => "系统通知",
        }
    }
}

impl From<SocketAction> for u32 {
    fn from(value: SocketAction) -> Self {
        value.as_u32()
    }
}

impl TryFrom<u32> for SocketAction {
    type Error = SocketError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(SocketError::UnknownAction(value))
    }
}

impl fmt::Display for SocketAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SocketAction {
    type Err = SocketError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        let action = trimmed
            .parse::<u32>()
            .map_err(|_| SocketError::ActionParse(trimmed.to_owned()))?;
        Self::try_from(action)
    }
}

impl Serialize for SocketAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.as_u32())
    }
}

impl<'de> Deserialize<'de> for SocketAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let action = u32::deserialize(deserializer)?;
        Self::from_u32(action).ok_or_else(|| DeError::custom(format!("未知的 action: {action}")))
    }
}

/// 宽容地解析 `action` 字段：同时接受 JSON 整数与十进制字符串。
fn deserialize_action<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawAction {
        Number(u32),
        Text(String),
    }

    match Option::<RawAction>::deserialize(deserializer)? {
        None => Ok(0),
        Some(RawAction::Number(value)) => Ok(value),
        Some(RawAction::Text(text)) => text
            .trim()
            .parse::<u32>()
            .map_err(|_| DeError::custom(format!("无法把 `{text}` 解析为 action"))),
    }
}

/// WebSocket 推送的统一包络。
///
/// 官方格式为 `{ "action": 1001, "status": "success", "detail": { ... } }`；
/// 为兼容旧版协议，`detail` 同时接受字段名 `data`，`action` 同时接受整数与字符串。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SocketEnvelope {
    /// 事件 action（官方规范为整数）。
    #[serde(deserialize_with = "deserialize_action")]
    pub action: u32,
    /// 服务端状态，正常推送为 `success`。
    pub status: String,
    /// 事件明细 JSON；字段结构由 `action` 决定。
    #[serde(alias = "data")]
    pub detail: serde_json::Value,
}

impl SocketEnvelope {
    /// `action` 对应的枚举；未知 action 返回 `None`。
    pub fn action(&self) -> Option<SocketAction> {
        SocketAction::from_u32(self.action)
    }

    /// 状态是否为 `success`。
    pub fn is_success(&self) -> bool {
        self.status == "success"
    }

    /// 把包络解析成强类型事件。
    ///
    /// # Errors
    ///
    /// `action` 未知，或 `detail` 与对应结构体不匹配时返回 [`SocketError`]。
    pub fn event(&self) -> Result<SocketEvent, SocketError> {
        SocketEvent::from_envelope(self)
    }
}

/// 强类型 WebSocket 事件：每个 action 一个变体，变体内携带解析后的明细。
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SocketEvent {
    /// 奇遇触发（action = `1001`）。
    EventTrigger(EventTriggerDetail),
    /// 马驹刷新（action = `1002`）。
    HorseRefresh(HorseRefreshDetail),
    /// 马驹捕获（action = `1003`）。
    HorseCapture(HorseCaptureDetail),
    /// 扶摇开启（action = `1005`）。
    FuyaoOpen(FuyaoOpenDetail),
    /// 扶摇点名（action = `1006`）。
    FuyaoRoll(FuyaoRollDetail),
    /// 的卢每日（action = `1008`）。
    DiluDaily(DiluDailyDetail),
    /// 的卢刷新（action = `1009`）。
    DiluRefresh(DiluRefreshDetail),
    /// 的卢捕获（action = `1010`）。
    DiluCapture(DiluCaptureDetail),
    /// 的卢拍卖（action = `1011`）。
    DiluAuction(DiluAuctionDetail),
    /// 副本掉落 / 私货（action = `1012`）。
    DungeonDrop(DungeonDropDetail),
    /// 阵营拍卖（action = `1013`）。
    CampAuction(CampAuctionDetail),
    /// 诛恶事件（action = `1014`）。
    PunishEvil(PunishEvilDetail),
    /// 追魂点名（action = `1015`）。
    SoulChaseRoll(SoulChaseRollDetail),
    /// 阵营祭祀（action = `1017`）。
    CampSacrifice(CampSacrificeDetail),
    /// 关隘首领（action = `1018`）。
    PassLeader(PassLeaderDetail),
    /// 领地宣战·开始（action = `1101`）。
    TerritoryWarStart(TerritoryWarStartDetail),
    /// 领地宣战·结束（action = `1102`）。
    TerritoryWarEnd(TerritoryWarEndDetail),
    /// 帮会宣战·开始（action = `1103`）。
    GuildWarStart(GuildWarStartDetail),
    /// 帮会宣战·结束（action = `1104`）。
    GuildWarEnd(GuildWarEndDetail),
    /// 帮会约战·完胜（action = `1105`）。
    GuildDuelVictory(GuildDuelVictoryDetail),
    /// 抢占粮仓（action = `1111`）。
    GranaryCapture(GranaryCaptureDetail),
    /// 大旗重置（action = `1112`）。
    BannerReset(BannerResetDetail),
    /// 大旗被夺（action = `1113`）。
    BannerSeized(BannerSeizedDetail),
    /// 据点占领（action = `1114`）。
    StrongholdCapture(StrongholdCaptureDetail),
    /// 据点占领（无帮会）（action = `1115`）。
    StrongholdCaptureNoGuild(StrongholdCaptureNoGuildDetail),
    /// 小攻防贡献（非开战）（action = `1116`）。
    MinorAttackContributionIdle(MinorAttackContributionIdleDetail),
    /// 小攻防贡献（action = `1117`）。
    MinorAttackContribution(MinorAttackContributionDetail),
    /// 大攻防贡献（action = `1118`）。
    MajorAttackContribution(MajorAttackContributionDetail),
    /// 战利品竞拍（action = `1119`）。
    LootAuction(LootAuctionDetail),
    /// 小攻防分红（action = `1120`）。
    MinorAttackDividend(MinorAttackDividendDetail),
    /// 大攻防分红（action = `1121`）。
    MajorAttackDividend(MajorAttackDividendDetail),
    /// 大攻防分红（含指挥）（action = `1122`）。
    MajorAttackDividendChief(MajorAttackDividendChiefDetail),
    /// 微博更新（action = `1201`）。
    WeiboUpdate(WeiboUpdateDetail),
    /// 开服状态（action = `2001`）。
    ServerStatus(ServerStatusDetail),
    /// 官方新闻（action = `2002`）。
    NewsUpdate(NewsUpdateDetail),
    /// 版本更新（action = `2003`）。
    VersionUpdate(VersionUpdateDetail),
    /// 八卦速报（action = `2004`）。
    GossipNews(GossipNewsDetail),
    /// 关隘首领（action = `2005`）。
    StrongholdCooldownReset(StrongholdCooldownResetDetail),
    /// 云从预告（action = `2006`）。
    MapEventPreview(MapEventPreviewDetail),
}

impl SocketEvent {
    /// 事件对应的 action。
    pub fn action(&self) -> SocketAction {
        match self {
            Self::EventTrigger(_) => SocketAction::EventTrigger,
            Self::HorseRefresh(_) => SocketAction::HorseRefresh,
            Self::HorseCapture(_) => SocketAction::HorseCapture,
            Self::FuyaoOpen(_) => SocketAction::FuyaoOpen,
            Self::FuyaoRoll(_) => SocketAction::FuyaoRoll,
            Self::DiluDaily(_) => SocketAction::DiluDaily,
            Self::DiluRefresh(_) => SocketAction::DiluRefresh,
            Self::DiluCapture(_) => SocketAction::DiluCapture,
            Self::DiluAuction(_) => SocketAction::DiluAuction,
            Self::DungeonDrop(_) => SocketAction::DungeonDrop,
            Self::CampAuction(_) => SocketAction::CampAuction,
            Self::PunishEvil(_) => SocketAction::PunishEvil,
            Self::SoulChaseRoll(_) => SocketAction::SoulChaseRoll,
            Self::CampSacrifice(_) => SocketAction::CampSacrifice,
            Self::PassLeader(_) => SocketAction::PassLeader,
            Self::TerritoryWarStart(_) => SocketAction::TerritoryWarStart,
            Self::TerritoryWarEnd(_) => SocketAction::TerritoryWarEnd,
            Self::GuildWarStart(_) => SocketAction::GuildWarStart,
            Self::GuildWarEnd(_) => SocketAction::GuildWarEnd,
            Self::GuildDuelVictory(_) => SocketAction::GuildDuelVictory,
            Self::GranaryCapture(_) => SocketAction::GranaryCapture,
            Self::BannerReset(_) => SocketAction::BannerReset,
            Self::BannerSeized(_) => SocketAction::BannerSeized,
            Self::StrongholdCapture(_) => SocketAction::StrongholdCapture,
            Self::StrongholdCaptureNoGuild(_) => SocketAction::StrongholdCaptureNoGuild,
            Self::MinorAttackContributionIdle(_) => SocketAction::MinorAttackContributionIdle,
            Self::MinorAttackContribution(_) => SocketAction::MinorAttackContribution,
            Self::MajorAttackContribution(_) => SocketAction::MajorAttackContribution,
            Self::LootAuction(_) => SocketAction::LootAuction,
            Self::MinorAttackDividend(_) => SocketAction::MinorAttackDividend,
            Self::MajorAttackDividend(_) => SocketAction::MajorAttackDividend,
            Self::MajorAttackDividendChief(_) => SocketAction::MajorAttackDividendChief,
            Self::WeiboUpdate(_) => SocketAction::WeiboUpdate,
            Self::ServerStatus(_) => SocketAction::ServerStatus,
            Self::NewsUpdate(_) => SocketAction::NewsUpdate,
            Self::VersionUpdate(_) => SocketAction::VersionUpdate,
            Self::GossipNews(_) => SocketAction::GossipNews,
            Self::StrongholdCooldownReset(_) => SocketAction::StrongholdCooldownReset,
            Self::MapEventPreview(_) => SocketAction::MapEventPreview,
        }
    }

    /// 事件的中文名（取自官方 `summary`）。
    pub fn name(&self) -> &'static str {
        self.action().name()
    }

    /// 把变体内的明细序列化成 JSON。
    pub fn detail_value(&self) -> serde_json::Value {
        match self {
            Self::EventTrigger(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::HorseRefresh(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::HorseCapture(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::FuyaoOpen(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::FuyaoRoll(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::DiluDaily(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::DiluRefresh(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::DiluCapture(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::DiluAuction(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::DungeonDrop(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::CampAuction(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::PunishEvil(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::SoulChaseRoll(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::CampSacrifice(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::PassLeader(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::TerritoryWarStart(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::TerritoryWarEnd(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::GuildWarStart(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::GuildWarEnd(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::GuildDuelVictory(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::GranaryCapture(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::BannerReset(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::BannerSeized(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::StrongholdCapture(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::StrongholdCaptureNoGuild(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MinorAttackContributionIdle(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MinorAttackContribution(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MajorAttackContribution(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::LootAuction(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MinorAttackDividend(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MajorAttackDividend(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MajorAttackDividendChief(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::WeiboUpdate(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::ServerStatus(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::NewsUpdate(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::VersionUpdate(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::GossipNews(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::StrongholdCooldownReset(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
            Self::MapEventPreview(detail) => {
                serde_json::to_value(detail).unwrap_or(serde_json::Value::Null)
            }
        }
    }

    /// 组装成与服务端一致的消息包络。
    pub fn envelope(&self) -> SocketEnvelope {
        SocketEnvelope {
            action: self.action().as_u32(),
            status: "success".to_owned(),
            detail: self.detail_value(),
        }
    }

    /// 从原始包络解析出强类型事件。
    ///
    /// # Errors
    ///
    /// `action` 未知，或 `detail` 与对应结构体不匹配时返回 [`SocketError`]。
    pub fn from_envelope(envelope: &SocketEnvelope) -> Result<Self, SocketError> {
        let action = SocketAction::try_from(envelope.action)?;
        let detail = envelope.detail.clone();
        Ok(match action {
            SocketAction::EventTrigger => Self::EventTrigger(serde_json::from_value(detail)?),
            SocketAction::HorseRefresh => Self::HorseRefresh(serde_json::from_value(detail)?),
            SocketAction::HorseCapture => Self::HorseCapture(serde_json::from_value(detail)?),
            SocketAction::FuyaoOpen => Self::FuyaoOpen(serde_json::from_value(detail)?),
            SocketAction::FuyaoRoll => Self::FuyaoRoll(serde_json::from_value(detail)?),
            SocketAction::DiluDaily => Self::DiluDaily(serde_json::from_value(detail)?),
            SocketAction::DiluRefresh => Self::DiluRefresh(serde_json::from_value(detail)?),
            SocketAction::DiluCapture => Self::DiluCapture(serde_json::from_value(detail)?),
            SocketAction::DiluAuction => Self::DiluAuction(serde_json::from_value(detail)?),
            SocketAction::DungeonDrop => Self::DungeonDrop(serde_json::from_value(detail)?),
            SocketAction::CampAuction => Self::CampAuction(serde_json::from_value(detail)?),
            SocketAction::PunishEvil => Self::PunishEvil(serde_json::from_value(detail)?),
            SocketAction::SoulChaseRoll => Self::SoulChaseRoll(serde_json::from_value(detail)?),
            SocketAction::CampSacrifice => Self::CampSacrifice(serde_json::from_value(detail)?),
            SocketAction::PassLeader => Self::PassLeader(serde_json::from_value(detail)?),
            SocketAction::TerritoryWarStart => {
                Self::TerritoryWarStart(serde_json::from_value(detail)?)
            }
            SocketAction::TerritoryWarEnd => Self::TerritoryWarEnd(serde_json::from_value(detail)?),
            SocketAction::GuildWarStart => Self::GuildWarStart(serde_json::from_value(detail)?),
            SocketAction::GuildWarEnd => Self::GuildWarEnd(serde_json::from_value(detail)?),
            SocketAction::GuildDuelVictory => {
                Self::GuildDuelVictory(serde_json::from_value(detail)?)
            }
            SocketAction::GranaryCapture => Self::GranaryCapture(serde_json::from_value(detail)?),
            SocketAction::BannerReset => Self::BannerReset(serde_json::from_value(detail)?),
            SocketAction::BannerSeized => Self::BannerSeized(serde_json::from_value(detail)?),
            SocketAction::StrongholdCapture => {
                Self::StrongholdCapture(serde_json::from_value(detail)?)
            }
            SocketAction::StrongholdCaptureNoGuild => {
                Self::StrongholdCaptureNoGuild(serde_json::from_value(detail)?)
            }
            SocketAction::MinorAttackContributionIdle => {
                Self::MinorAttackContributionIdle(serde_json::from_value(detail)?)
            }
            SocketAction::MinorAttackContribution => {
                Self::MinorAttackContribution(serde_json::from_value(detail)?)
            }
            SocketAction::MajorAttackContribution => {
                Self::MajorAttackContribution(serde_json::from_value(detail)?)
            }
            SocketAction::LootAuction => Self::LootAuction(serde_json::from_value(detail)?),
            SocketAction::MinorAttackDividend => {
                Self::MinorAttackDividend(serde_json::from_value(detail)?)
            }
            SocketAction::MajorAttackDividend => {
                Self::MajorAttackDividend(serde_json::from_value(detail)?)
            }
            SocketAction::MajorAttackDividendChief => {
                Self::MajorAttackDividendChief(serde_json::from_value(detail)?)
            }
            SocketAction::WeiboUpdate => Self::WeiboUpdate(serde_json::from_value(detail)?),
            SocketAction::ServerStatus => Self::ServerStatus(serde_json::from_value(detail)?),
            SocketAction::NewsUpdate => Self::NewsUpdate(serde_json::from_value(detail)?),
            SocketAction::VersionUpdate => Self::VersionUpdate(serde_json::from_value(detail)?),
            SocketAction::GossipNews => Self::GossipNews(serde_json::from_value(detail)?),
            SocketAction::StrongholdCooldownReset => {
                Self::StrongholdCooldownReset(serde_json::from_value(detail)?)
            }
            SocketAction::MapEventPreview => Self::MapEventPreview(serde_json::from_value(detail)?),
        })
    }
}

impl Serialize for SocketEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.envelope().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SocketEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let envelope = SocketEnvelope::deserialize(deserializer)?;
        Self::from_envelope(&envelope).map_err(DeError::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_actions_roundtrip() {
        assert_eq!(SocketAction::ALL.len(), 39);

        let mut seen = HashSet::new();
        for action in SocketAction::ALL {
            assert!(seen.insert(action.as_u32()), "重复的 action: {action}");
            assert_eq!(action.as_str().parse::<u32>().unwrap(), action.as_u32());
            assert_eq!(SocketAction::from_u32(action.as_u32()), Some(action));
            assert_eq!(SocketAction::from_str(action.as_str()).unwrap(), action);
            assert_eq!(action.to_string(), action.as_str());
            assert_eq!(u32::from(action), action.as_u32());
            assert_eq!(SocketAction::try_from(action.as_u32()).unwrap(), action);
            assert!(!action.name().is_empty());
            assert!(!action.category().is_empty());
        }
        assert_eq!(seen.len(), 39);
    }

    #[test]
    fn parses_envelope_with_typed_detail() {
        let raw = r#"{"action":1001,"status":"success","detail":{"zone":"电信一区","server":"长安城","name":"叶英","event":"黑白路","time":1794106931}}"#;
        let envelope: SocketEnvelope = serde_json::from_str(raw).expect("包络应当能解析");
        assert!(envelope.is_success());
        assert_eq!(envelope.action(), Some(SocketAction::EventTrigger));

        let event = envelope.event().expect("明细应当能解析");
        assert_eq!(event.action(), SocketAction::EventTrigger);
        assert_eq!(event.name(), "奇遇触发");
        match &event {
            SocketEvent::EventTrigger(detail) => {
                assert_eq!(detail.zone, "电信一区");
                assert_eq!(detail.server, "长安城");
                assert_eq!(detail.name, "叶英");
                assert_eq!(detail.event, "黑白路");
                assert_eq!(detail.time, 1_794_106_931);
            }
            other => panic!("action 派发错误: {other:?}"),
        }
    }

    #[test]
    fn parses_string_action_array_detail_and_legacy_data_field() {
        let raw = r#"{"action":"1006","status":"success","data":{"zone":"双梦镇","server":"梦江南","name":["甲","乙"],"time":"1794106931"}}"#;
        let event: SocketEvent = serde_json::from_str(raw).expect("事件应当能解析");
        assert_eq!(event.action(), SocketAction::FuyaoRoll);
        match &event {
            SocketEvent::FuyaoRoll(detail) => {
                assert_eq!(detail.zone, "双梦镇");
                assert_eq!(detail.name, vec!["甲".to_owned(), "乙".to_owned()]);
                assert_eq!(detail.time, 1_794_106_931);
            }
            other => panic!("action 派发错误: {other:?}"),
        }
    }

    #[test]
    fn event_serializes_back_to_envelope() {
        let raw = r#"{"action":2001,"status":"success","detail":{"zone":"电信一区","server":"长安城","status":"开服","time":1794106931}}"#;
        let event: SocketEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(event.action(), SocketAction::ServerStatus);

        let envelope = event.envelope();
        assert_eq!(envelope.action, 2001);
        assert_eq!(envelope.status, "success");
        assert_eq!(envelope.detail["server"], "长安城");

        let json = serde_json::to_string(&event).unwrap();
        let back: SocketEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(back, event);
    }

    #[test]
    fn rejects_unknown_action() {
        let raw = r#"{"action":9999,"status":"success","detail":{}}"#;
        assert!(serde_json::from_str::<SocketEvent>(raw).is_err());
        assert!(matches!(
            SocketAction::from_str("9999"),
            Err(SocketError::UnknownAction(9999))
        ));
        assert!(SocketAction::from_str("not-a-number").is_err());
    }
}
