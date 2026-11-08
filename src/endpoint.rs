//! 接口元数据。
//!
//! 本文件由 `scripts/gen_endpoints.py` 依据官方 OpenAPI 规范自动生成，请勿手工修改。
//!
//! [`Endpoint`] 为每个 HTTP 接口提供了路径、所需档位与中文名，方便运行期自检、
//! 生成文档或做权限判断。

use serde::{Deserialize, Serialize};

/// 接口所需档位。
///
/// 与官方 OpenAPI 的 `x-level` 字段一致，也与站点令牌档位对应：
///
/// * [`Level::Free`] —— 免费接口，无需令牌；
/// * [`Level::Vip1`] —— 需要 VIP1 令牌；
/// * [`Level::Vip2`] —— 需要 VIP2 令牌。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// 免费接口。
    Free,
    /// 需要 VIP1 令牌。
    Vip1,
    /// 需要 VIP2 令牌。
    Vip2,
}

impl Level {
    /// 档位的中文描述。
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Free => "免费",
            Level::Vip1 => "VIP1",
            Level::Vip2 => "VIP2",
        }
    }

    /// 档位数值，与 OpenAPI 的 `x-level` 对应。
    pub fn as_u8(self) -> u8 {
        match self {
            Level::Free => 0,
            Level::Vip1 => 1,
            Level::Vip2 => 2,
        }
    }

    /// 是否为免费接口。
    pub fn is_free(self) -> bool {
        matches!(self, Level::Free)
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 单个 HTTP 接口的元数据。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endpoint {
    /// 活动日历（`/active/calendar`）
    ActiveCalendar,
    /// 地图活动（`/active/celebs`）
    ActiveCelebs,
    /// 名剑排行（`/arena/awesome`）
    ArenaAwesome,
    /// 名剑战绩（`/arena/recent`）
    ArenaRecent,
    /// 名剑统计（`/arena/schools`）
    ArenaSchools,
    /// 阵营拍卖（`/auction/records`）
    AuctionRecords,
    /// 帮战记录（`/battle/records`）
    BattleRecords,
    /// 装备属性（`/calcd/attribute`）
    CalcdAttribute,
    /// 装备详情（`/calcd/detail`）
    CalcdDetail,
    /// 装备记录（`/calcd/records`）
    CalcdRecords,
    /// 名片缓存（`/card/cached`）
    CardCached,
    /// 名片预设（`/card/preset`）
    CardPreset,
    /// 随机名片（`/card/random`）
    CardRandom,
    /// 名片记录（`/card/record`）
    CardRecord,
    /// 名片历史（`/card/records`）
    CardRecords,
    /// 关隘首领（`/castle/status`）
    CastleStatus,
    /// 角色聊天（`/chat/records`）
    ChatRecords,
    /// 今日赤兔（`/chitu/records`）
    ChituRecords,
    /// 本周赤兔（`/chitu/week/records`）
    ChituWeekRecords,
    /// 统战歪歪（`/duowan/statistics`）
    DuowanStatistics,
    /// 奇遇汇总（`/event/collect`）
    EventCollect,
    /// 未触发的（`/event/missing`）
    EventMissing,
    /// 近期奇遇（`/event/recent`）
    EventRecent,
    /// 奇遇记录（`/event/records`）
    EventRecords,
    /// 奇遇统计（`/event/statistics`）
    EventStatistics,
    /// 奇遇攻略（`/event/strategy`）
    EventStrategy,
    /// 科举答案（`/exam/search`）
    ExamSearch,
    /// 烟花记录（`/firework/records`）
    FireworkRecords,
    /// 小吃小药（`/food/list`）
    FoodList,
    /// 骗子查询（`/fraud/detail`）
    FraudDetail,
    /// 家园鲜花（`/home/flower`）
    HomeFlower,
    /// 家园装饰（`/home/furniture`）
    HomeFurniture,
    /// 器物图谱（`/home/travel`）
    HomeTravel,
    /// 秘境方位（`/mech/decrypt`）
    MechDecrypt,
    /// 师徒列表（`/mentor/search`）
    MentorSearch,
    /// 角色百战（`/monster/records`）
    MonsterRecords,
    /// 百战首领（`/monster/weekly`）
    MonsterWeekly,
    /// 维护公告（`/news/announce`）
    NewsAnnounce,
    /// 新闻资讯（`/news/records`）
    NewsRecords,
    /// 马场预告（`/ranch/chat`）
    RanchChat,
    /// 马场事件（`/ranch/records`）
    RanchRecords,
    /// 跨服名剑（`/rank/arena`）
    RankArena,
    /// 武林争霸（`/rank/championship`）
    RankChampionship,
    /// 捕快荣誉（`/rank/constable`）
    RankConstable,
    /// 江湖浪客（`/rank/outlaw`）
    RankOutlaw,
    /// 排行统计（`/rank/statistics`）
    RankStatistics,
    /// 试炼之地（`/rank/trials`）
    RankTrials,
    /// 决斗挑战（`/rank/wanted`）
    RankWanted,
    /// 团队招募（`/recruit/search`）
    RecruitSearch,
    /// 掉落统计（`/reward/statistics`）
    RewardStatistics,
    /// 成就查询（`/role/achievement`）
    RoleAchievement,
    /// 角色详情（`/role/detail`）
    RoleDetail,
    /// 沙盘据点（`/sand/records`）
    SandRecords,
    /// 答案之书（`/saohua/answer`）
    SaohuaAnswer,
    /// 舔狗日志（`/saohua/content`）
    SaohuaContent,
    /// 分类语录（`/saohua/context`）
    SaohuaContext,
    /// 喝什么（`/saohua/drink`）
    SaohuaDrink,
    /// 吃什么（`/saohua/eat`）
    SaohuaEat,
    /// 随机骚话（`/saohua/random`）
    SaohuaRandom,
    /// 渣男语录（`/saohua/zhanan`）
    SaohuaZhanan,
    /// 阵眼效果（`/school/matrix`）
    SchoolMatrix,
    /// 配装搜索（`/school/search`）
    SchoolSearch,
    /// 资历榜单（`/school/seniority`）
    SchoolSeniority,
    /// 技能信息（`/school/skills`）
    SchoolSkills,
    /// 奇穴信息（`/school/talent`）
    SchoolTalent,
    /// 大区状态（`/server/status/check`）
    ServerStatusCheck,
    /// 急速计算（`/skill/calculate`）
    SkillCalculate,
    /// 技改记录（`/skill/rework`）
    SkillRework,
    /// 语音合成（`/sound/converter`）
    SoundConverter,
    /// 的卢拍卖（`/steed/records`）
    SteedRecords,
    /// 外观记录（`/tieba/item/records`）
    TiebaItemRecords,
    /// 百度贴吧（`/tieba/random`）
    TiebaRandom,
    /// 金价行情（`/trade/demon`）
    TradeDemon,
    /// 外观价格（`/trade/item/records`）
    TradeItemRecords,
    /// 外观搜索（`/trade/item/search`）
    TradeItemSearch,
    /// 成本计算（`/trade/manufacture`）
    TradeManufacture,
    /// 物品价格（`/trade/records`）
    TradeRecords,
    /// 编号搜索（`/trade/wanbaolou`）
    TradeWanbaolou,
    /// 秒伤计算（`/trial/bosses`）
    TrialBosses,
    /// 赛季列表（`/trial/seasons`）
    TrialSeasons,
    /// 资历分布（`/tuilan/achievement`）
    TuilanAchievement,
    /// 诛恶事件（`/wicked/records`）
    WickedRecords,
}

impl Endpoint {
    /// 全部接口。
    pub const ALL: &'static [Endpoint] = &[
        Endpoint::ActiveCalendar,
        Endpoint::ActiveCelebs,
        Endpoint::ArenaAwesome,
        Endpoint::ArenaRecent,
        Endpoint::ArenaSchools,
        Endpoint::AuctionRecords,
        Endpoint::BattleRecords,
        Endpoint::CalcdAttribute,
        Endpoint::CalcdDetail,
        Endpoint::CalcdRecords,
        Endpoint::CardCached,
        Endpoint::CardPreset,
        Endpoint::CardRandom,
        Endpoint::CardRecord,
        Endpoint::CardRecords,
        Endpoint::CastleStatus,
        Endpoint::ChatRecords,
        Endpoint::ChituRecords,
        Endpoint::ChituWeekRecords,
        Endpoint::DuowanStatistics,
        Endpoint::EventCollect,
        Endpoint::EventMissing,
        Endpoint::EventRecent,
        Endpoint::EventRecords,
        Endpoint::EventStatistics,
        Endpoint::EventStrategy,
        Endpoint::ExamSearch,
        Endpoint::FireworkRecords,
        Endpoint::FoodList,
        Endpoint::FraudDetail,
        Endpoint::HomeFlower,
        Endpoint::HomeFurniture,
        Endpoint::HomeTravel,
        Endpoint::MechDecrypt,
        Endpoint::MentorSearch,
        Endpoint::MonsterRecords,
        Endpoint::MonsterWeekly,
        Endpoint::NewsAnnounce,
        Endpoint::NewsRecords,
        Endpoint::RanchChat,
        Endpoint::RanchRecords,
        Endpoint::RankArena,
        Endpoint::RankChampionship,
        Endpoint::RankConstable,
        Endpoint::RankOutlaw,
        Endpoint::RankStatistics,
        Endpoint::RankTrials,
        Endpoint::RankWanted,
        Endpoint::RecruitSearch,
        Endpoint::RewardStatistics,
        Endpoint::RoleAchievement,
        Endpoint::RoleDetail,
        Endpoint::SandRecords,
        Endpoint::SaohuaAnswer,
        Endpoint::SaohuaContent,
        Endpoint::SaohuaContext,
        Endpoint::SaohuaDrink,
        Endpoint::SaohuaEat,
        Endpoint::SaohuaRandom,
        Endpoint::SaohuaZhanan,
        Endpoint::SchoolMatrix,
        Endpoint::SchoolSearch,
        Endpoint::SchoolSeniority,
        Endpoint::SchoolSkills,
        Endpoint::SchoolTalent,
        Endpoint::ServerStatusCheck,
        Endpoint::SkillCalculate,
        Endpoint::SkillRework,
        Endpoint::SoundConverter,
        Endpoint::SteedRecords,
        Endpoint::TiebaItemRecords,
        Endpoint::TiebaRandom,
        Endpoint::TradeDemon,
        Endpoint::TradeItemRecords,
        Endpoint::TradeItemSearch,
        Endpoint::TradeManufacture,
        Endpoint::TradeRecords,
        Endpoint::TradeWanbaolou,
        Endpoint::TrialBosses,
        Endpoint::TrialSeasons,
        Endpoint::TuilanAchievement,
        Endpoint::WickedRecords,
    ];

    /// 接口路径，形如 `/active/calendar`。
    pub fn path(self) -> &'static str {
        match self {
            Endpoint::ActiveCalendar => "/active/calendar",
            Endpoint::ActiveCelebs => "/active/celebs",
            Endpoint::ArenaAwesome => "/arena/awesome",
            Endpoint::ArenaRecent => "/arena/recent",
            Endpoint::ArenaSchools => "/arena/schools",
            Endpoint::AuctionRecords => "/auction/records",
            Endpoint::BattleRecords => "/battle/records",
            Endpoint::CalcdAttribute => "/calcd/attribute",
            Endpoint::CalcdDetail => "/calcd/detail",
            Endpoint::CalcdRecords => "/calcd/records",
            Endpoint::CardCached => "/card/cached",
            Endpoint::CardPreset => "/card/preset",
            Endpoint::CardRandom => "/card/random",
            Endpoint::CardRecord => "/card/record",
            Endpoint::CardRecords => "/card/records",
            Endpoint::CastleStatus => "/castle/status",
            Endpoint::ChatRecords => "/chat/records",
            Endpoint::ChituRecords => "/chitu/records",
            Endpoint::ChituWeekRecords => "/chitu/week/records",
            Endpoint::DuowanStatistics => "/duowan/statistics",
            Endpoint::EventCollect => "/event/collect",
            Endpoint::EventMissing => "/event/missing",
            Endpoint::EventRecent => "/event/recent",
            Endpoint::EventRecords => "/event/records",
            Endpoint::EventStatistics => "/event/statistics",
            Endpoint::EventStrategy => "/event/strategy",
            Endpoint::ExamSearch => "/exam/search",
            Endpoint::FireworkRecords => "/firework/records",
            Endpoint::FoodList => "/food/list",
            Endpoint::FraudDetail => "/fraud/detail",
            Endpoint::HomeFlower => "/home/flower",
            Endpoint::HomeFurniture => "/home/furniture",
            Endpoint::HomeTravel => "/home/travel",
            Endpoint::MechDecrypt => "/mech/decrypt",
            Endpoint::MentorSearch => "/mentor/search",
            Endpoint::MonsterRecords => "/monster/records",
            Endpoint::MonsterWeekly => "/monster/weekly",
            Endpoint::NewsAnnounce => "/news/announce",
            Endpoint::NewsRecords => "/news/records",
            Endpoint::RanchChat => "/ranch/chat",
            Endpoint::RanchRecords => "/ranch/records",
            Endpoint::RankArena => "/rank/arena",
            Endpoint::RankChampionship => "/rank/championship",
            Endpoint::RankConstable => "/rank/constable",
            Endpoint::RankOutlaw => "/rank/outlaw",
            Endpoint::RankStatistics => "/rank/statistics",
            Endpoint::RankTrials => "/rank/trials",
            Endpoint::RankWanted => "/rank/wanted",
            Endpoint::RecruitSearch => "/recruit/search",
            Endpoint::RewardStatistics => "/reward/statistics",
            Endpoint::RoleAchievement => "/role/achievement",
            Endpoint::RoleDetail => "/role/detail",
            Endpoint::SandRecords => "/sand/records",
            Endpoint::SaohuaAnswer => "/saohua/answer",
            Endpoint::SaohuaContent => "/saohua/content",
            Endpoint::SaohuaContext => "/saohua/context",
            Endpoint::SaohuaDrink => "/saohua/drink",
            Endpoint::SaohuaEat => "/saohua/eat",
            Endpoint::SaohuaRandom => "/saohua/random",
            Endpoint::SaohuaZhanan => "/saohua/zhanan",
            Endpoint::SchoolMatrix => "/school/matrix",
            Endpoint::SchoolSearch => "/school/search",
            Endpoint::SchoolSeniority => "/school/seniority",
            Endpoint::SchoolSkills => "/school/skills",
            Endpoint::SchoolTalent => "/school/talent",
            Endpoint::ServerStatusCheck => "/server/status/check",
            Endpoint::SkillCalculate => "/skill/calculate",
            Endpoint::SkillRework => "/skill/rework",
            Endpoint::SoundConverter => "/sound/converter",
            Endpoint::SteedRecords => "/steed/records",
            Endpoint::TiebaItemRecords => "/tieba/item/records",
            Endpoint::TiebaRandom => "/tieba/random",
            Endpoint::TradeDemon => "/trade/demon",
            Endpoint::TradeItemRecords => "/trade/item/records",
            Endpoint::TradeItemSearch => "/trade/item/search",
            Endpoint::TradeManufacture => "/trade/manufacture",
            Endpoint::TradeRecords => "/trade/records",
            Endpoint::TradeWanbaolou => "/trade/wanbaolou",
            Endpoint::TrialBosses => "/trial/bosses",
            Endpoint::TrialSeasons => "/trial/seasons",
            Endpoint::TuilanAchievement => "/tuilan/achievement",
            Endpoint::WickedRecords => "/wicked/records",
        }
    }

    /// 所需档位。
    pub fn level(self) -> Level {
        match self {
            Endpoint::ActiveCalendar => Level::Free,
            Endpoint::ActiveCelebs => Level::Free,
            Endpoint::ArenaAwesome => Level::Vip1,
            Endpoint::ArenaRecent => Level::Vip1,
            Endpoint::ArenaSchools => Level::Vip1,
            Endpoint::AuctionRecords => Level::Vip2,
            Endpoint::BattleRecords => Level::Free,
            Endpoint::CalcdAttribute => Level::Vip2,
            Endpoint::CalcdDetail => Level::Vip2,
            Endpoint::CalcdRecords => Level::Vip2,
            Endpoint::CardCached => Level::Vip2,
            Endpoint::CardPreset => Level::Vip2,
            Endpoint::CardRandom => Level::Vip2,
            Endpoint::CardRecord => Level::Vip2,
            Endpoint::CardRecords => Level::Vip2,
            Endpoint::CastleStatus => Level::Vip2,
            Endpoint::ChatRecords => Level::Vip2,
            Endpoint::ChituRecords => Level::Vip2,
            Endpoint::ChituWeekRecords => Level::Vip2,
            Endpoint::DuowanStatistics => Level::Free,
            Endpoint::EventCollect => Level::Vip1,
            Endpoint::EventMissing => Level::Vip1,
            Endpoint::EventRecent => Level::Vip1,
            Endpoint::EventRecords => Level::Vip1,
            Endpoint::EventStatistics => Level::Vip1,
            Endpoint::EventStrategy => Level::Vip1,
            Endpoint::ExamSearch => Level::Free,
            Endpoint::FireworkRecords => Level::Vip2,
            Endpoint::FoodList => Level::Free,
            Endpoint::FraudDetail => Level::Vip1,
            Endpoint::HomeFlower => Level::Free,
            Endpoint::HomeFurniture => Level::Free,
            Endpoint::HomeTravel => Level::Free,
            Endpoint::MechDecrypt => Level::Vip1,
            Endpoint::MentorSearch => Level::Vip1,
            Endpoint::MonsterRecords => Level::Vip2,
            Endpoint::MonsterWeekly => Level::Vip2,
            Endpoint::NewsAnnounce => Level::Free,
            Endpoint::NewsRecords => Level::Free,
            Endpoint::RanchChat => Level::Vip2,
            Endpoint::RanchRecords => Level::Vip1,
            Endpoint::RankArena => Level::Vip2,
            Endpoint::RankChampionship => Level::Vip2,
            Endpoint::RankConstable => Level::Vip2,
            Endpoint::RankOutlaw => Level::Vip2,
            Endpoint::RankStatistics => Level::Vip1,
            Endpoint::RankTrials => Level::Vip1,
            Endpoint::RankWanted => Level::Vip2,
            Endpoint::RecruitSearch => Level::Vip1,
            Endpoint::RewardStatistics => Level::Vip1,
            Endpoint::RoleAchievement => Level::Vip2,
            Endpoint::RoleDetail => Level::Vip1,
            Endpoint::SandRecords => Level::Vip1,
            Endpoint::SaohuaAnswer => Level::Free,
            Endpoint::SaohuaContent => Level::Free,
            Endpoint::SaohuaContext => Level::Vip2,
            Endpoint::SaohuaDrink => Level::Free,
            Endpoint::SaohuaEat => Level::Free,
            Endpoint::SaohuaRandom => Level::Free,
            Endpoint::SaohuaZhanan => Level::Free,
            Endpoint::SchoolMatrix => Level::Vip1,
            Endpoint::SchoolSearch => Level::Vip2,
            Endpoint::SchoolSeniority => Level::Vip1,
            Endpoint::SchoolSkills => Level::Vip1,
            Endpoint::SchoolTalent => Level::Vip1,
            Endpoint::ServerStatusCheck => Level::Free,
            Endpoint::SkillCalculate => Level::Vip2,
            Endpoint::SkillRework => Level::Free,
            Endpoint::SoundConverter => Level::Free,
            Endpoint::SteedRecords => Level::Vip2,
            Endpoint::TiebaItemRecords => Level::Vip1,
            Endpoint::TiebaRandom => Level::Vip1,
            Endpoint::TradeDemon => Level::Vip1,
            Endpoint::TradeItemRecords => Level::Vip1,
            Endpoint::TradeItemSearch => Level::Vip1,
            Endpoint::TradeManufacture => Level::Vip2,
            Endpoint::TradeRecords => Level::Vip1,
            Endpoint::TradeWanbaolou => Level::Vip2,
            Endpoint::TrialBosses => Level::Vip1,
            Endpoint::TrialSeasons => Level::Vip1,
            Endpoint::TuilanAchievement => Level::Vip2,
            Endpoint::WickedRecords => Level::Vip2,
        }
    }

    /// 接口中文名。
    pub fn summary(self) -> &'static str {
        match self {
            Endpoint::ActiveCalendar => "活动日历",
            Endpoint::ActiveCelebs => "地图活动",
            Endpoint::ArenaAwesome => "名剑排行",
            Endpoint::ArenaRecent => "名剑战绩",
            Endpoint::ArenaSchools => "名剑统计",
            Endpoint::AuctionRecords => "阵营拍卖",
            Endpoint::BattleRecords => "帮战记录",
            Endpoint::CalcdAttribute => "装备属性",
            Endpoint::CalcdDetail => "装备详情",
            Endpoint::CalcdRecords => "装备记录",
            Endpoint::CardCached => "名片缓存",
            Endpoint::CardPreset => "名片预设",
            Endpoint::CardRandom => "随机名片",
            Endpoint::CardRecord => "名片记录",
            Endpoint::CardRecords => "名片历史",
            Endpoint::CastleStatus => "关隘首领",
            Endpoint::ChatRecords => "角色聊天",
            Endpoint::ChituRecords => "今日赤兔",
            Endpoint::ChituWeekRecords => "本周赤兔",
            Endpoint::DuowanStatistics => "统战歪歪",
            Endpoint::EventCollect => "奇遇汇总",
            Endpoint::EventMissing => "未触发的",
            Endpoint::EventRecent => "近期奇遇",
            Endpoint::EventRecords => "奇遇记录",
            Endpoint::EventStatistics => "奇遇统计",
            Endpoint::EventStrategy => "奇遇攻略",
            Endpoint::ExamSearch => "科举答案",
            Endpoint::FireworkRecords => "烟花记录",
            Endpoint::FoodList => "小吃小药",
            Endpoint::FraudDetail => "骗子查询",
            Endpoint::HomeFlower => "家园鲜花",
            Endpoint::HomeFurniture => "家园装饰",
            Endpoint::HomeTravel => "器物图谱",
            Endpoint::MechDecrypt => "秘境方位",
            Endpoint::MentorSearch => "师徒列表",
            Endpoint::MonsterRecords => "角色百战",
            Endpoint::MonsterWeekly => "百战首领",
            Endpoint::NewsAnnounce => "维护公告",
            Endpoint::NewsRecords => "新闻资讯",
            Endpoint::RanchChat => "马场预告",
            Endpoint::RanchRecords => "马场事件",
            Endpoint::RankArena => "跨服名剑",
            Endpoint::RankChampionship => "武林争霸",
            Endpoint::RankConstable => "捕快荣誉",
            Endpoint::RankOutlaw => "江湖浪客",
            Endpoint::RankStatistics => "排行统计",
            Endpoint::RankTrials => "试炼之地",
            Endpoint::RankWanted => "决斗挑战",
            Endpoint::RecruitSearch => "团队招募",
            Endpoint::RewardStatistics => "掉落统计",
            Endpoint::RoleAchievement => "成就查询",
            Endpoint::RoleDetail => "角色详情",
            Endpoint::SandRecords => "沙盘据点",
            Endpoint::SaohuaAnswer => "答案之书",
            Endpoint::SaohuaContent => "舔狗日志",
            Endpoint::SaohuaContext => "分类语录",
            Endpoint::SaohuaDrink => "喝什么",
            Endpoint::SaohuaEat => "吃什么",
            Endpoint::SaohuaRandom => "随机骚话",
            Endpoint::SaohuaZhanan => "渣男语录",
            Endpoint::SchoolMatrix => "阵眼效果",
            Endpoint::SchoolSearch => "配装搜索",
            Endpoint::SchoolSeniority => "资历榜单",
            Endpoint::SchoolSkills => "技能信息",
            Endpoint::SchoolTalent => "奇穴信息",
            Endpoint::ServerStatusCheck => "大区状态",
            Endpoint::SkillCalculate => "急速计算",
            Endpoint::SkillRework => "技改记录",
            Endpoint::SoundConverter => "语音合成",
            Endpoint::SteedRecords => "的卢拍卖",
            Endpoint::TiebaItemRecords => "外观记录",
            Endpoint::TiebaRandom => "百度贴吧",
            Endpoint::TradeDemon => "金价行情",
            Endpoint::TradeItemRecords => "外观价格",
            Endpoint::TradeItemSearch => "外观搜索",
            Endpoint::TradeManufacture => "成本计算",
            Endpoint::TradeRecords => "物品价格",
            Endpoint::TradeWanbaolou => "编号搜索",
            Endpoint::TrialBosses => "秒伤计算",
            Endpoint::TrialSeasons => "赛季列表",
            Endpoint::TuilanAchievement => "资历分布",
            Endpoint::WickedRecords => "诛恶事件",
        }
    }

    /// 按路径查找接口。
    pub fn from_path(path: &str) -> Option<Endpoint> {
        let path = path.trim_start_matches('/');
        Endpoint::ALL
            .iter()
            .copied()
            .find(|endpoint| endpoint.path().trim_start_matches('/') == path)
    }

    /// 是否为免费接口。
    pub fn is_free(self) -> bool {
        self.level().is_free()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_paths_are_unique() {
        assert!(!Endpoint::ALL.is_empty());
        let unique = Endpoint::ALL
            .iter()
            .map(|endpoint| endpoint.path())
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(Endpoint::ALL.len(), unique.len(), "接口路径不应重复");
    }

    #[test]
    fn lookup_by_path() {
        let endpoint = Endpoint::from_path("/active/calendar").expect("应存在该接口");
        assert_eq!(endpoint.path(), "/active/calendar");
        assert!(endpoint.is_free());
    }
}
