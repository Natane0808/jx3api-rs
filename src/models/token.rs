//! 站点令牌配额相关数据模型。
//!
//! 对应接口：`POST /token/stats`（查询令牌的档位、已用次数、剩余次数与充值记录）。
//!
//! 注意：该路径**不在**官方 OpenAPI 快照的 82 条路径内，因此没有对应的
//! [`Endpoint`](crate::Endpoint) 变体，单独在此建模。

use serde::{Deserialize, Serialize};

/// 令牌配额快照。
///
/// 对应接口：`POST /token/stats`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxxxxxx").build()?;
/// let stats = client.quota().stats().await?;
/// println!(
///     "{}：已用 {}，剩余 {}，每次消耗 {}",
///     stats.level_name(),
///     stats.used,
///     stats.remaining,
///     stats.each
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TokenStats {
    /// 令牌档位：`1` = VIP1，`2` = VIP2。官方当前只发放这两档。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub level: i64,
    /// 已消耗次数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub used: i64,
    /// 剩余可用次数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub remaining: i64,
    /// 每次接口调用消耗的次数（当前为 `1`，官方可能调整，**不要写死**）。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub each: i64,
    /// 到期时间戳（秒）；`0` 表示永久有效。
    #[serde(
        rename = "expireAt",
        deserialize_with = "crate::serde_util::flexible_i64"
    )]
    pub expire_at: i64,
    /// 令牌当前是否有效。
    #[serde(deserialize_with = "crate::serde_util::flexible_bool")]
    pub valid: bool,
    /// 充值 / 变动记录，按官方返回顺序（通常为倒序）。
    pub history: Vec<TokenHistoryEntry>,
}

impl TokenStats {
    /// 总配额（已用 + 剩余）。
    ///
    /// 用于计算用量占比：`stats.used as f64 / stats.total_quota() as f64`。
    pub fn total_quota(&self) -> i64 {
        self.used.saturating_add(self.remaining)
    }

    /// 档位的中文名称；未知档位返回 `"未知"`。
    pub fn level_name(&self) -> &'static str {
        match self.level {
            1 => "VIP1",
            2 => "VIP2",
            _ => "未知",
        }
    }

    /// 是否为永久有效的令牌（官方用 `expireAt = 0` 表示不过期）。
    pub fn is_perpetual(&self) -> bool {
        self.expire_at == 0
    }

    /// 到期时间戳；永久有效时返回 `None`。
    pub fn expire_at_unix(&self) -> Option<i64> {
        if self.is_perpetual() {
            None
        } else {
            Some(self.expire_at)
        }
    }
}

/// 令牌充值 / 变动记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TokenHistoryEntry {
    /// 变动类型，例如 `purchase`（购买）。
    pub action: String,
    /// 发生时间戳（秒）。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub at: i64,
    /// 变动前的令牌（通常为 `null`）。
    #[serde(default)]
    pub from: Option<String>,
    /// 变动后的令牌，**服务端已做掩码**，例如 `jx3api::0606****c11a`。
    pub to: String,
    /// 订单号。
    pub no: String,
    /// 本次充值 / 变动的次数。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_fixture() {
        // 真实响应快照：订单号已脱敏，令牌本身不在响应中（服务端只回掩码）。
        let envelope: ApiResponse<TokenStats> =
            parse(include_str!("../../tests/fixtures/token_stats.json"));
        assert!(envelope.is_success());

        let stats = envelope.data.expect("data 不应为空");
        assert_eq!(stats.level, 2);
        assert_eq!(stats.used, 321);
        assert_eq!(stats.remaining, 4679);
        assert_eq!(stats.each, 1);
        assert_eq!(stats.expire_at, 0);
        assert!(stats.valid);
        assert_eq!(stats.history.len(), 1);

        let entry = &stats.history[0];
        assert_eq!(entry.action, "purchase");
        assert_eq!(entry.count, 5000);
        assert_eq!(entry.from, None, "from 为 null 时应解析成 None");
        assert!(entry.to.contains("****"), "服务端返回的令牌应已掩码");
    }

    #[test]
    fn helper_methods() {
        let stats = TokenStats {
            level: 2,
            used: 321,
            remaining: 4679,
            each: 1,
            expire_at: 0,
            valid: true,
            history: Vec::new(),
        };
        assert_eq!(stats.total_quota(), 5000);
        assert_eq!(stats.level_name(), "VIP2");
        assert!(stats.is_perpetual());
        assert_eq!(stats.expire_at_unix(), None);

        let expiring = TokenStats {
            level: 1,
            expire_at: 1_800_000_000,
            ..TokenStats::default()
        };
        assert_eq!(expiring.level_name(), "VIP1");
        assert!(!expiring.is_perpetual());
        assert_eq!(expiring.expire_at_unix(), Some(1_800_000_000));

        let unknown = TokenStats::default();
        assert_eq!(unknown.level_name(), "未知");
        assert_eq!(unknown.total_quota(), 0);
    }

    #[test]
    fn accepts_string_and_null_forms() {
        // 官方数值字段为整数，但沿用仓库约定容忍数字字符串与 null。
        let raw = r#"{"level":"2","used":"10","remaining":null,"each":"1",
                      "expireAt":"0","valid":1,"history":[]}"#;
        let stats: TokenStats = serde_json::from_str(raw).unwrap();
        assert_eq!(stats.level, 2);
        assert_eq!(stats.used, 10);
        assert_eq!(stats.remaining, 0);
        assert!(stats.valid);
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let stats: TokenStats = serde_json::from_str(r#"{"valid":true}"#).unwrap();
        assert!(stats.valid);
        assert_eq!(stats.level, 0);
        assert_eq!(stats.remaining, 0);
        assert!(stats.history.is_empty());
    }
}
