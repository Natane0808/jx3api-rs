//! 百战相关接口。
//!
//! 对应 OpenAPI 中的 `/monster/*` 路径，两个接口均为 VIP2。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::monster::{MonsterRecord, MonsterWeekly};

/// 角色百战查询参数。
///
/// 对应接口：`GET /monster/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::monster::MonsterRecordsQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let record = client.monster().records(&query).await?;
/// println!("{} 体力 {}", record.role_name, record.skill_stamina);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct MonsterRecordsQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MonsterRecordsQuery {
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

/// 百战相关接口。
///
/// 通过 [`Client::monster`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let weekly = client.monster().weekly().await?;
/// println!("第 {} 周轮换首领：{}", weekly.week, weekly.boss);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MonsterApi {
    inner: Arc<Inner>,
}

impl MonsterApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 角色百战（VIP2）。
    ///
    /// 返回角色的百战技能配装（体力 / 精力 / 场次 / 技能列表）；未打过百战的角色
    /// 取全服默认技能。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`MonsterRecordsQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::monster::MonsterRecordsQuery::default()
    ///     .server("长安城")
    ///     .name("漓心");
    /// let record = client.monster().records(&query).await?;
    /// println!("{} 个技能，剩余体力 {}", record.skill_list.len(), record.skill_stamina);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &MonsterRecordsQuery) -> Result<MonsterRecord> {
        self.inner.get("/monster/records", query).await
    }

    /// 本周百战首领（VIP2）。
    ///
    /// 返回本周首领列表及 buff 效果，并附荡剑恩仇轮换首领。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for boss in client.monster().weekly().await?.list {
    ///     println!("{} {}", boss.index, boss.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn weekly(&self) -> Result<MonsterWeekly> {
        self.inner
            .get("/monster/weekly", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 百战相关接口。
    pub fn monster(&self) -> MonsterApi {
        MonsterApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_query_serializes_all_fields() {
        let query = MonsterRecordsQuery::default().server("长安城").name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
        assert!(encoded.contains("name=%E6%BC%93%E5%BF%83"));
    }

    #[test]
    fn empty_query_serializes_to_empty_string() {
        let encoded = serde_urlencoded::to_string(MonsterRecordsQuery::default()).unwrap();
        assert!(encoded.is_empty());
    }
}
