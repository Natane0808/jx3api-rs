//! 帮战记录数据模型。

use serde::{Deserialize, Serialize};

/// 单场帮战记录。
///
/// 对应接口：`GET /battle/records?server=长安城`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BattleRecord {
    /// 宣战帮会名称。
    pub declaring_name: String,
    /// 应战帮会名称。
    pub accepting_name: String,
    /// 开始时间戳。
    pub start_time: i64,
    /// 持续秒数。
    pub duration_seconds: i64,
    /// 结束时间戳，进行中的帮战为 `None`。
    pub end_time: Option<i64>,
    /// 获胜帮会名称（结束后才有）。
    pub winner_name: Option<String>,
    /// 获胜积分（结束后才有）。
    pub winner_score: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    #[test]
    fn parses_fixture() {
        let envelope: ApiResponse<Vec<BattleRecord>> =
            serde_json::from_str(include_str!("../../tests/fixtures/battle_records.json"))
                .expect("夹具应当能解析");
        assert!(envelope.is_success());
        let records = envelope.data.expect("data 不应为空");
        assert_eq!(records.len(), 30);
        let first = &records[0];
        assert_eq!(first.declaring_name, "星火");
        assert_eq!(first.accepting_name, "撒野");
        assert_eq!(first.start_time, 1_793_715_756);
        assert_eq!(first.duration_seconds, 3600);
        assert_eq!(first.end_time, Some(1_793_719_356));
        assert!(first.winner_name.is_none());
        assert!(first.winner_score.is_none());
    }
}
