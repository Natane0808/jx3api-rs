//! 活动相关数据模型。
//!
//! 对应接口：`/active/calendar`、`/active/celebs`。

use serde::{Deserialize, Serialize};

/// 活动日历（`mode=day`）返回的当天活动。
///
/// 对应接口：`GET /active/calendar?mode=day`。
///
/// 注意：实际响应中 `date`/`week`/`war`/`battle`/`orecar`/`rescue`/`school` 一定存在，
/// `card`/`lucky`/`draw`/`leader` 视当天活动而定，`weekly` 仅部分日期出现。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveCalendar {
    /// 日期，例如 `2026-11-08`。
    pub date: String,
    /// 星期，例如 `日`。
    pub week: String,
    /// 大战。
    pub war: String,
    /// 战场。
    pub battle: String,
    /// 矿车活动。
    pub orecar: String,
    /// 驰援事件。
    pub rescue: String,
    /// 门派事件。
    pub school: String,
    /// 名片掉落地图。
    pub card: Vec<String>,
    /// 宠物奇遇 NPC。
    pub lucky: Vec<String>,
    /// 美人图（仅星期三、五、六、日有）。
    pub draw: Vec<String>,
    /// 世界首领（仅星期三、五有）。
    pub leader: Vec<String>,
    /// 周常活动，仅部分日期出现。
    pub weekly: Option<ActiveWeekly>,
}

/// 周常活动。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveWeekly {
    /// 五人周常。
    pub conn: Vec<String>,
    /// 团队周常。
    pub raid: Vec<String>,
}

/// 活动月历（`mode=list`）中的当天信息。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveCalendarToday {
    /// 日期，例如 `2026-11-08`。
    pub date: String,
    /// 星期，例如 `日`。
    pub week: String,
    /// 年，例如 `2026`。
    pub year: String,
    /// 月，例如 `11`。
    pub month: String,
    /// 日，例如 `08`。
    pub day: String,
}

/// 活动月历（`mode=list`）中的单日活动。
///
/// 与 [`ActiveCalendar`] 相比少了 `team` / `weekly`，多了 `leader`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveCalendarItem {
    /// 日期，例如 `2026-11-06`。
    pub date: String,
    /// 星期，例如 `五`。
    pub week: String,
    /// 大战。
    pub war: String,
    /// 战场。
    pub battle: String,
    /// 矿车活动。
    pub orecar: String,
    /// 驰援事件。
    pub rescue: String,
    /// 门派事件。
    pub school: String,
    /// 名片掉落地图。
    pub card: Vec<String>,
    /// 宠物奇遇 NPC。
    pub lucky: Vec<String>,
    /// 美人图。
    pub draw: Vec<String>,
    /// 世界首领。
    pub leader: Vec<String>,
}

/// 活动月历（`mode=list`）的返回结果。
///
/// 对应接口：`GET /active/calendar?mode=list&num=N`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveListCalendar {
    /// 当天信息。
    pub today: ActiveCalendarToday,
    /// 日期区间内的活动（字段名是 `total` 而不是 `data`）。
    pub total: Vec<ActiveCalendarItem>,
}

/// 地图活动（行侠事件）条目。
///
/// 对应接口：`GET /active/celebs?name=楚天社`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveCelebs {
    /// 地图名称。
    pub map: String,
    /// 事件阶段。
    pub stage: String,
    /// 事件地点。
    pub site: String,
    /// 事件描述。
    pub desc: String,
    /// 事件图标编号。
    pub icon: String,
    /// 事件时间，例如 `11:06`。
    pub time: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_calendar_day_fixture() {
        let envelope: ApiResponse<ActiveCalendar> =
            parse(include_str!("../../tests/fixtures/active_calendar.json"));
        assert!(envelope.is_success());
        let calendar = envelope.data.expect("data 不应为空");
        assert_eq!(calendar.date, "2026-11-08");
        assert_eq!(calendar.war, "大战！英雄大阳津");
        assert!(!calendar.card.is_empty(), "当天应有名片掉落");
    }

    #[test]
    fn parses_calendar_list_fixture() {
        let envelope: ApiResponse<ActiveListCalendar> = parse(include_str!(
            "../../tests/fixtures/active_calendar_list.json"
        ));
        assert!(envelope.is_success());
        let calendar = envelope.data.expect("data 不应为空");
        assert_eq!(calendar.today.year, "2026");
        assert!(!calendar.total.is_empty(), "月度区间应有活动");
        assert!(!calendar.total[0].war.is_empty());
    }

    #[test]
    fn parses_celebs_fixture() {
        let envelope: ApiResponse<Vec<ActiveCelebs>> =
            parse(include_str!("../../tests/fixtures/active_celebs.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert!(!items.is_empty());
        assert!(!items[0].map.is_empty());
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let calendar: ActiveCalendar = serde_json::from_str(r#"{"date":"2026-11-08"}"#).unwrap();
        assert_eq!(calendar.date, "2026-11-08");
        assert!(calendar.war.is_empty());
        assert!(calendar.weekly.is_none());
    }
}
