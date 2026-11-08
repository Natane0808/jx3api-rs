//! 烟花记录接口。
//!
//! 对应 OpenAPI 中的 `/firework/records` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::firework::FireworkRecord;

/// 烟花记录查询参数。
///
/// 对应接口：`GET /firework/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::firework::FireworkRecordsQuery;
/// let query = FireworkRecordsQuery::default()
///     .server("长安城")
///     .name("漓心");
/// println!("{query:?}");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct FireworkRecordsQuery {
    /// 服务器名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FireworkRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 烟花记录接口。
///
/// 通过 [`Client::firework`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::firework::FireworkRecordsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let query = FireworkRecordsQuery::default().server("长安城").name("漓心");
/// let records = client.firework().records(&query).await?;
/// println!("{} 条烟花记录", records.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FireworkApi {
    inner: Arc<Inner>,
}

impl FireworkApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 烟花记录（VIP2）。
    ///
    /// 查询角色作为送花人或收花人的烟花记录，结果按时间倒序。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`FireworkRecordsQuery`]；其中 `server`、`name`
    ///   均为必填。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码
    /// （例如缺少 `server` / `name`）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::firework::FireworkRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// let query = FireworkRecordsQuery::default().server("长安城").name("漓心");
    /// for record in client.firework().records(&query).await? {
    ///     println!("{} → {} {}", record.sender, record.receiver, record.firework);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &FireworkRecordsQuery) -> Result<Vec<FireworkRecord>> {
        self.inner.get("/firework/records", query).await
    }
}

impl Client {
    /// 烟花记录接口。
    pub fn firework(&self) -> FireworkApi {
        FireworkApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_skips_unset_fields() {
        let query = FireworkRecordsQuery::default().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.starts_with("server="));
        assert!(!encoded.contains("name="));
    }

    #[test]
    fn query_serializes_all_fields() {
        let query = FireworkRecordsQuery::default()
            .server("长安城")
            .name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
    }
}
