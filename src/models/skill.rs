//! 技能数据模型。
//!
//! 对应接口：`/skill/calculate`、`/skill/rework`。

use serde::{Deserialize, Serialize};

/// 单个急速档位的读条信息。
///
/// 对应接口：`GET /skill/calculate?cooldown=1.5`（VIP2）。
///
/// 官方字段是驼峰命名，Rust 侧统一转成 `snake_case` 并用 `rename` 映射回 JSON。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkillCalculate {
    /// 实际读条时长（秒）。
    #[serde(deserialize_with = "crate::serde_util::flexible_f64")]
    pub duration: f64,
    /// 该档位所需急速百分比。
    #[serde(
        rename = "hastePercent",
        deserialize_with = "crate::serde_util::flexible_f64"
    )]
    pub haste_percent: f64,
    /// 带 buff 的急速百分比上限。
    #[serde(
        rename = "hastePercentLimit",
        deserialize_with = "crate::serde_util::flexible_f64"
    )]
    pub haste_percent_limit: f64,
    /// 当前帧数。
    #[serde(
        rename = "nowFrame",
        deserialize_with = "crate::serde_util::flexible_i64"
    )]
    pub now_frame: i64,
    /// 带 buff 的帧数（`u` 前缀为 buff 版字段）。
    #[serde(
        rename = "uNowFrame",
        deserialize_with = "crate::serde_util::flexible_i64"
    )]
    pub u_now_frame: i64,
    /// 剩余帧数。
    #[serde(
        rename = "surplusNowFrame",
        deserialize_with = "crate::serde_util::flexible_i64"
    )]
    pub surplus_now_frame: i64,
    /// 所需急速等级。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub level: i64,
}

/// 一条技改公告。
///
/// 对应接口：`GET /skill/rework`（免费）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkillRework {
    /// 公告编号。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub id: i64,
    /// 公告标题。
    pub title: String,
    /// 公告链接。
    pub url: String,
    /// 发布时间，例如 `2026-10-29 09:07:27`。
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
    fn parses_calculate_fixture() {
        let envelope: ApiResponse<Vec<SkillCalculate>> =
            parse(include_str!("../../tests/fixtures/skill_calculate.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 6);
        let first = &items[0];
        assert_eq!(first.duration, 1.5);
        assert_eq!(first.now_frame, 24);
        assert_eq!(first.u_now_frame, 24);
        assert_eq!(first.surplus_now_frame, 1);
        assert_eq!(first.level, 1);
        assert!(first.haste_percent > 0.0);
        assert_eq!(first.haste_percent, first.haste_percent_limit);
    }

    #[test]
    fn parses_rework_fixture() {
        let envelope: ApiResponse<Vec<SkillRework>> =
            parse(include_str!("../../tests/fixtures/skill_rework.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 20);
        let first = &items[0];
        assert_eq!(first.id, 1335886);
        assert!(first.title.contains("武学调整"));
        assert!(first.url.starts_with("https://jx3.xoyo.com/"));
        assert_eq!(first.time, "2026-10-29 09:07:27");
    }

    #[test]
    fn calculate_accepts_camel_case_json() {
        let item: SkillCalculate = serde_json::from_str(
            r#"{"duration":"1.5","hastePercent":"0.5","hastePercentLimit":0.5,
                "nowFrame":"24","uNowFrame":24,"surplusNowFrame":1,"level":"1"}"#,
        )
        .unwrap();
        assert_eq!(item.duration, 1.5);
        assert_eq!(item.now_frame, 24);
        assert_eq!(item.level, 1);
    }
}
