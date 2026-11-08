//! 赤兔数据模型。
//!
//! 对应接口：`/chitu/records`（今日）与 `/chitu/week/records`（本周）。
//!
//! 注意两个接口的模型**不同**：今日记录多出 `id` / `send`，本周记录多出
//! `time` / `window` / `status`，因此分别建模，不要合并。

use serde::{Deserialize, Serialize};

/// 今日赤兔记录。
///
/// 对应接口：`GET /chitu/records`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChituRecord {
    /// 记录编号。
    pub id: i64,
    /// 服务器名称，例如 `蝶恋花`。
    pub server: String,
    /// 地图名称（官方类型标注为 `map_name`，线上响应使用驼峰 `mapName`，两者都兼容）。
    #[serde(rename = "mapName", alias = "map_name")]
    pub map_name: String,
    /// 坐骑名称，例如 `赤兔`。
    pub horse: String,
    /// 送出标记（接口原样透传）。
    pub send: i64,
    /// 日期，例如 `2026-11-07`。
    pub date: String,
}

/// 本周赤兔记录。
///
/// 对应接口：`GET /chitu/week/records`，本周期周二 07:00 重置。
///
/// 与 [`ChituRecord`] 相比没有 `id` / `send`，但多了 `time` / `window` / `status`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChituWeekRecord {
    /// 服务器名称，例如 `蝶恋花`。
    pub server: String,
    /// 地图名称（官方类型标注为 `map_name`，线上响应使用驼峰 `mapName`，两者都兼容）。
    #[serde(rename = "mapName", alias = "map_name")]
    pub map_name: String,
    /// 坐骑名称，例如 `赤兔`。
    pub horse: String,
    /// 日期，例如 `2026-11-07`。
    pub date: String,
    /// 出世时间（Unix 时间戳，秒）。
    pub time: i64,
    /// 刷新时间段，例如 `22:20~22:25`；部分记录为 `null`。
    pub window: Option<String>,
    /// 状态文案，例如 `出世`。
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    /// 今日赤兔夹具是错误响应（`code = 400`、`data = {}`），只断言能解析。
    #[test]
    fn parses_day_error_fixture() {
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/chitu_records.json"));
        assert!(!envelope.is_success());
        assert_eq!(envelope.msg, "暂无数据");
    }

    /// 今日赤兔没有可用的成功夹具，用内联 JSON 校验字段映射。
    #[test]
    fn parses_day_record_from_inline_json() {
        let record: ChituRecord = serde_json::from_str(
            r#"{"id":1,"server":"蝶恋花","mapName":"阴山大草原","horse":"赤兔","send":1,"date":"2026-11-07"}"#,
        )
        .unwrap();
        assert_eq!(record.id, 1);
        assert_eq!(record.server, "蝶恋花");
        assert_eq!(record.map_name, "阴山大草原");
        assert_eq!(record.horse, "赤兔");
        assert_eq!(record.send, 1);
        assert_eq!(record.date, "2026-11-07");
    }

    #[test]
    fn parses_week_fixture() {
        let envelope: ApiResponse<Vec<ChituWeekRecord>> =
            parse(include_str!("../../tests/fixtures/chitu_week_records.json"));
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert!(!records.is_empty());
        let first = &records[0];
        assert_eq!(first.server, "蝶恋花");
        assert_eq!(first.map_name, "阴山大草原");
        assert_eq!(first.horse, "赤兔");
        assert_eq!(first.date, "2026-11-07");
        assert_eq!(first.window.as_deref(), Some("22:20~22:25"));
        assert_eq!(first.status, "出世");
        assert!(first.time > 0);
        assert!(
            records.iter().any(|record| record.window.is_none()),
            "夹具中应存在 window 为 null 的记录"
        );
    }
}
