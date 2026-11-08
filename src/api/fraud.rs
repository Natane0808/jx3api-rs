//! 骗子查询相关接口。
//!
//! 对应 OpenAPI 中的 `/fraud/detail` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::fraud::FraudRecord;

/// 骗子查询参数。
///
/// 对应接口：`GET /fraud/detail`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::fraud::FraudDetailQuery;
/// let query = FraudDetailQuery::new().server("长安城").uid(100001);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct FraudDetailQuery {
    /// 服务器名称（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色 UID（必填），取值范围 `100000..=4000000000`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

impl FraudDetailQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色 UID（`100000..=4000000000`）。
    pub fn uid(mut self, uid: i64) -> Self {
        self.uid = Some(uid);
        self
    }
}

/// 骗子查询接口。
///
/// 通过 [`Client::fraud`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::fraud::FraudDetailQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = FraudDetailQuery::new().server("长安城").uid(100001);
/// for record in client.fraud().detail(&query).await? {
///     println!("{}：{}", record.author, record.title);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FraudApi {
    inner: Arc<Inner>,
}

impl FraudApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 骗子查询（VIP1）。
    ///
    /// 转发到远程风控服务，返回指定 UID 的详细记录。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`FraudDetailQuery`]。
    ///   `uid` 必填（`100000..=4000000000`），`server` 可选。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码；
    /// 远程风控服务尚未实现时服务端会返回 `500`。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::fraud::FraudDetailQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = FraudDetailQuery::new().uid(100001);
    /// let records = client.fraud().detail(&query).await?;
    /// println!("共 {} 条记录", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn detail(&self, query: &FraudDetailQuery) -> Result<Vec<FraudRecord>> {
        self.inner.get("/fraud/detail", query).await
    }
}

impl Client {
    /// 骗子查询接口。
    pub fn fraud(&self) -> FraudApi {
        FraudApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serializes_uid_and_server() {
        let query = FraudDetailQuery::new().server("长安城").uid(100001);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("uid=100001"));
        assert!(encoded.contains("server="));
    }

    #[test]
    fn query_skips_missing_server() {
        let query = FraudDetailQuery::new().uid(100001);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "uid=100001");
    }
}
