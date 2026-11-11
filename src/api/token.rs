//! 站点令牌配额查询接口。
//!
//! 对应 `POST /token/stats`：查询令牌的档位、已用次数、剩余次数与充值记录。
//!
//! # 与其它业务域的区别
//!
//! * 该路径**不在**官方 OpenAPI 快照的 82 条 GET 路径内，因此没有对应的
//!   [`Endpoint`](crate::Endpoint) 变体，也不参与 `scripts/verify_coverage.py`
//!   的路径覆盖统计；
//! * 它是本库唯一使用 `POST` + JSON 请求体的接口，令牌放在**请求体**而不是查询串；
//! * 查询本身**不消耗次数**（实测连续两次查询 `used` 不变）。
//!
//! # 快速开始
//!
//! ```no_run
//! # use jx3_api::Client;
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::builder().token("jx3api::xxxxxxxx").build()?;
//!
//! let stats = client.quota().stats().await?;
//! println!("{} 剩余 {} / {}", stats.level_name(), stats.remaining, stats.total_quota());
//!
//! // 也可以在不改动客户端配置的前提下，单独校验另一枚令牌
//! let other = client.quota().stats_of("jx3api::yyyyyyyy").await?;
//! println!("另一枚令牌剩余 {}", other.remaining);
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::{Error, Result};
use crate::models::token::TokenStats;

/// `/token/stats` 的请求体。
#[derive(Debug, Serialize)]
struct TokenStatsBody<'a> {
    /// 要查询的站点令牌。
    token: &'a str,
}

/// 令牌配额查询接口。
///
/// 通过 [`Client::quota`] 获取。
#[derive(Debug, Clone)]
pub struct TokenApi {
    inner: Arc<Inner>,
}

impl TokenApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 查询**当前客户端令牌**的配额。
    ///
    /// 返回档位、已用 / 剩余次数、每次消耗、到期时间与充值记录。
    ///
    /// # Errors
    ///
    /// * [`Error::MissingToken`] —— 客户端未配置令牌（本接口需要令牌，且不会发请求）；
    /// * [`Error::Api`] —— 服务端拒绝，例如令牌不存在（`code = 404`，`msg = "令牌不存在"`）；
    /// * [`Error::Http`] / [`Error::Decode`] —— 网络异常或响应无法解析。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxxxxxx").build()?;
    /// let stats = client.quota().stats().await?;
    /// if !stats.is_perpetual() {
    ///     println!("将于时间戳 {} 到期", stats.expire_at);
    /// }
    /// println!("剩余 {} 次（每次消耗 {}）", stats.remaining, stats.each);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats(&self) -> Result<TokenStats> {
        let token = self.inner.token().ok_or_else(|| Error::MissingToken {
            path: "/token/stats".to_owned(),
        })?;
        self.query(&token).await
    }

    /// 查询**指定令牌**的配额，不影响客户端上的令牌配置。
    ///
    /// 适合在把令牌写入客户端之前先校验它是否有效、还剩多少次。
    ///
    /// # Errors
    ///
    /// * [`Error::MissingToken`] —— 传入的令牌为空字符串；
    /// * [`Error::Api`] —— 服务端拒绝（令牌不存在时 `code = 404`）；
    /// * [`Error::Http`] / [`Error::Decode`] —— 网络异常或响应无法解析。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::{Client, Error};
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// match client.quota().stats_of("jx3api::xxxxxxxx").await {
    ///     Ok(stats) => println!("令牌有效，剩余 {} 次", stats.remaining),
    ///     Err(Error::Api { code: 404, .. }) => println!("令牌不存在"),
    ///     Err(err) => eprintln!("查询失败：{err}"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats_of(&self, token: impl AsRef<str>) -> Result<TokenStats> {
        let token = token.as_ref().trim();
        if token.is_empty() {
            return Err(Error::MissingToken {
                path: "/token/stats".to_owned(),
            });
        }
        self.query(token).await
    }

    /// 实际发起 `POST /token/stats`。
    async fn query(&self, token: &str) -> Result<TokenStats> {
        self.inner
            .post_json("/token/stats", &TokenStatsBody { token })
            .await
    }
}

impl Client {
    /// 令牌配额查询接口（`POST /token/stats`）。
    ///
    /// # 命名说明
    ///
    /// [`Client::token`] 已被「读取当前令牌字符串」占用，因此这里用
    /// `quota()`（配额）作为访问器。
    pub fn quota(&self) -> TokenApi {
        TokenApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_serializes_token_only() {
        let body = TokenStatsBody {
            token: "jx3api::abc",
        };
        let json = serde_json::to_string(&body).unwrap();
        assert_eq!(json, r#"{"token":"jx3api::abc"}"#);
    }

    #[tokio::test]
    async fn stats_without_token_fails_before_request() {
        let client = Client::new().unwrap();
        match client.quota().stats().await {
            Err(Error::MissingToken { path }) => assert_eq!(path, "/token/stats"),
            other => panic!("未配置令牌时应返回 MissingToken，实际 {other:?}"),
        }
    }

    #[tokio::test]
    async fn stats_of_empty_token_fails_before_request() {
        let client = Client::with_token("jx3api::configured").unwrap();
        match client.quota().stats_of("   ").await {
            Err(Error::MissingToken { .. }) => {}
            other => panic!("空令牌应返回 MissingToken，实际 {other:?}"),
        }
    }
}
