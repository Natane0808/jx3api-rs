/// Token请求头
pub(crate) const HEADER_TOKEN_KEY: &'static str = "token";
/// API请求前缀
pub(crate) const API_PREFIX: &'static str = "https://www.jx3api.com/data";

/// 活动日历
pub(crate) const CALENDAR: &'static str = "/active/calendar";
/// 活动月历
pub(crate) const LIST_CALENDAR: &'static str = "/active/list/calendar";

#[cfg(feature = "vip")]
mod vip {
    
}
