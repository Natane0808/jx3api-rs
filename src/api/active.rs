//! 活动相关接口。
//!
//! 对应 OpenAPI 中的 `/active/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::active::{ActiveCalendar, ActiveCelebs, ActiveListCalendar};

/// 活动日历查询方式。
///
/// `/active/calendar` 通过 `mode` 参数决定返回「单日」还是「前后区间」，两者的
/// 返回结构完全不同，因此用独立的两个方法 [`ActiveApi::calendar`] /
/// [`ActiveApi::list_calendar`] 表达，避免调用方拿到 `serde_json::Value`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarMode {
    /// 单日推演。
    Day,
    /// 前后区间推演。
    List,
}

impl CalendarMode {
    /// 查询参数取值，`day` 或 `list`。
    pub fn as_str(self) -> &'static str {
        match self {
            CalendarMode::Day => "day",
            CalendarMode::List => "list",
        }
    }
}

impl std::fmt::Display for CalendarMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 活动相关接口。
///
/// 通过 [`Client::active`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let today = client.active().calendar(None).await?;
/// println!("{} 大战：{}", today.date, today.war);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ActiveApi {
    inner: Arc<Inner>,
}

impl ActiveApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 活动日历（单日）。
    ///
    /// 查询今天、明天、后天的日常活动。
    ///
    /// # 参数
    ///
    /// * `num` —— 天数偏移，`None` 时获取当天；`0` 当天、`1` 明天、`2` 后天，
    ///   取值范围 `0..=15`。
    ///
    /// # 注意
    ///
    /// 美人图仅在星期三、五、六、日提供；世界首领仅在星期三、五存在。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如参数越界）。
    pub async fn calendar(&self, num: Option<u32>) -> Result<ActiveCalendar> {
        let mut query: Vec<(&str, String)> = vec![("mode", CalendarMode::Day.to_string())];
        if let Some(num) = num {
            query.push(("num", num.to_string()));
        }
        self.inner.get("/active/calendar", &query).await
    }

    /// 活动月历（前后区间）。
    ///
    /// 返回今天前后 `num` 天的活动推演。
    ///
    /// # 参数
    ///
    /// * `num` —— 区间半径，`None` 时使用服务端默认值 0（当天），取值范围 `0..=15`。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let month = client.active().list_calendar(Some(7)).await?;
    /// for day in &month.total {
    ///     println!("{} {} 大战：{}", day.date, day.week, day.war);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_calendar(&self, num: Option<u32>) -> Result<ActiveListCalendar> {
        let mut query: Vec<(&str, String)> = vec![("mode", CalendarMode::List.to_string())];
        if let Some(num) = num {
            query.push(("num", num.to_string()));
        }
        self.inner.get("/active/calendar", &query).await
    }

    /// 地图活动（行侠事件）。
    ///
    /// 返回楚天社 / 云从社 / 披风会接下来 10 条排期。
    ///
    /// # 参数
    ///
    /// * `name` —— 事件名称，可取 [`CelebClub`] 中的三种社团。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::active::CelebClub;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let events = client.active().celebs(CelebClub::Cts).await?;
    /// for event in events {
    ///     println!("{} {} {}", event.time, event.map, event.stage);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn celebs(&self, name: impl AsRef<str>) -> Result<Vec<ActiveCelebs>> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/active/celebs", &query).await
    }
}

/// 行侠事件所属社团。
///
/// 用于 [`ActiveApi::celebs`] 的 `name` 参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CelebClub {
    /// 楚天社。
    Cts,
    /// 云从社。
    Ycs,
    /// 披风会。
    Pfh,
}

impl CelebClub {
    /// 社团中文名，即接口参数取值。
    pub fn as_str(self) -> &'static str {
        match self {
            CelebClub::Cts => "楚天社",
            CelebClub::Ycs => "云从社",
            CelebClub::Pfh => "披风会",
        }
    }

    /// 从中文名解析，非法值返回 `None`。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "楚天社" => Some(CelebClub::Cts),
            "云从社" => Some(CelebClub::Ycs),
            "披风会" => Some(CelebClub::Pfh),
            _ => None,
        }
    }

    /// 全部社团。
    pub const ALL: &'static [CelebClub] = &[CelebClub::Cts, CelebClub::Ycs, CelebClub::Pfh];
}

impl std::fmt::Display for CelebClub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for CelebClub {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Client {
    /// 活动相关接口。
    pub fn active(&self) -> ActiveApi {
        ActiveApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_mode_values() {
        assert_eq!(CalendarMode::Day.as_str(), "day");
        assert_eq!(CalendarMode::List.as_str(), "list");
    }

    #[test]
    fn celeb_club_round_trip() {
        for club in CelebClub::ALL {
            assert_eq!(CelebClub::from_str(club.as_str()), Some(*club));
        }
        assert_eq!(CelebClub::from_str("不存在的社团"), None);
    }
}
