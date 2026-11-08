//! 奇遇相关接口。
//!
//! 对应 OpenAPI 中的 `/event/*` 路径，全部为 VIP1 接口。
//!
//! 注意不同接口的 `name` 参数语义不同：
//!
//! * [`EventApi::records`]、[`EventApi::missing`] 的 `name` 是**角色名**；
//! * [`EventApi::statistics`]、[`EventApi::strategy`] 的 `name` 是**奇遇名**。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::event::{EventCollect, EventMissing, EventRecord, EventStrategy};

/// 奇遇汇总查询参数。
///
/// 对应接口：`GET /event/collect`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::event::EventCollectQuery;
/// let query = EventCollectQuery::new().server("长安城").num(7);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventCollectQuery {
    /// 服务器名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 统计天数，默认 7，取值范围 `1..=30`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i64>,
}

impl EventCollectQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置统计天数（`1..=30`）。
    pub fn num(mut self, num: i64) -> Self {
        self.num = Some(num);
        self
    }
}

/// 未触发奇遇查询参数。
///
/// 对应接口：`GET /event/missing`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::event::EventMissingQuery;
/// let query = EventMissingQuery::new().server("长安城").name("漓心");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventMissingQuery {
    /// 服务器名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EventMissingQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 近期奇遇查询参数。
///
/// 对应接口：`GET /event/recent`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::event::EventRecentQuery;
/// let query = EventRecentQuery::new().server("长安城").limit(20);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventRecentQuery {
    /// 服务器名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 显示数量，默认 20，取值范围 `1..=50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl EventRecentQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置显示数量（`1..=50`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 奇遇记录查询参数。
///
/// 对应接口：`GET /event/records`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::event::EventRecordsQuery;
/// let query = EventRecordsQuery::new().server("长安城").name("漓心").full(1);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventRecordsQuery {
    /// 目标区服（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否补全未触发奇遇，`0` 否 / `1` 是，默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<i64>,
}

impl EventRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置区服。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否返回完整列表（`0` 否 / `1` 是）。
    pub fn full(mut self, full: i64) -> Self {
        self.full = Some(full);
        self
    }
}

/// 奇遇统计查询参数。
///
/// 对应接口：`GET /event/statistics`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::event::EventStatisticsQuery;
/// let query = EventStatisticsQuery::new().name("白日梦").limit(20);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventStatisticsQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 奇遇名称（必填，注意不是角色名）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 显示数量，默认 20，取值范围 `1..=50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl EventStatisticsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置奇遇名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置显示数量（`1..=50`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 奇遇相关接口。
///
/// 通过 [`Client::event`] 获取。全部接口都需要 VIP1 及以上令牌。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::event::EventRecentQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = EventRecentQuery::new().server("长安城");
/// for record in client.event().recent(&query).await? {
///     println!("{} 触发了 {}", record.name, record.event);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct EventApi {
    inner: Arc<Inner>,
}

impl EventApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 奇遇汇总（VIP1）。
    ///
    /// 统计每个奇遇在最近 `num` 天内于指定服务器的触发次数，并附带历史最新一条记录。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`EventCollectQuery`]。
    ///   `server` 必填，`num` 默认 7，取值范围 `1..=30`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::event::EventCollectQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = EventCollectQuery::new().server("长安城").num(7);
    /// for item in client.event().collect(&query).await? {
    ///     println!("{} 触发 {} 次", item.event, item.count);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collect(&self, query: &EventCollectQuery) -> Result<Vec<EventCollect>> {
        self.inner.get("/event/collect", query).await
    }

    /// 未触发的奇遇（VIP1）。
    ///
    /// 返回该角色尚未触发的奇遇（不在全局绑定 / 服务器奇遇记录中的奇遇）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`EventMissingQuery`]。
    ///   `server` 与 `name` 必填，且 **`name` 是角色名**。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::event::EventMissingQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = EventMissingQuery::new().server("长安城").name("漓心");
    /// for item in client.event().missing(&query).await? {
    ///     println!("未触发：{}（{}）", item.name, item.r#type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn missing(&self, query: &EventMissingQuery) -> Result<Vec<EventMissing>> {
        self.inner.get("/event/missing", query).await
    }

    /// 近期奇遇（VIP1）。
    ///
    /// 返回指定服务器最近触发的奇遇事件，按时间倒序排列。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`EventRecentQuery`]。
    ///   `server` 必填，`limit` 默认 20，取值范围 `1..=50`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::event::EventRecentQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = EventRecentQuery::new().server("长安城");
    /// for record in client.event().recent(&query).await? {
    ///     println!("{} {} {}", record.time, record.name, record.event);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn recent(&self, query: &EventRecentQuery) -> Result<Vec<EventRecord>> {
        self.inner.get("/event/recent", query).await
    }

    /// 奇遇记录（VIP1）。
    ///
    /// 整合角色全局奇遇记录、服务器奇遇记录与魔盒成就，可通过 `full` 补全未触发的奇遇。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`EventRecordsQuery`]。
    ///   `server` 与 `name` 必填，且 **`name` 是角色名**；`full` 默认 `0`。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::event::EventRecordsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = EventRecordsQuery::new().server("长安城").name("漓心");
    /// for record in client.event().records(&query).await? {
    ///     println!("{} 于 {} 触发 {}", record.name, record.time, record.event);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &EventRecordsQuery) -> Result<Vec<EventRecord>> {
        self.inner.get("/event/records", query).await
    }

    /// 奇遇统计（VIP1）。
    ///
    /// 返回某个奇遇最近的触发记录，按时间倒序；指定 `server` 时查单服，否则查全服。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`EventStatisticsQuery`]。
    ///   **`name` 是奇遇名（不是角色名）且必填**，`server` 可选，`limit` 默认 20（`1..=50`）。
    ///
    /// # 注意
    ///
    /// 与 [`EventApi::records`] / [`EventApi::missing`] 不同，本接口的 `name`
    /// 表示**奇遇名称**。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::event::EventStatisticsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = EventStatisticsQuery::new().name("白日梦").limit(20);
    /// for record in client.event().statistics(&query).await? {
    ///     println!("{} 触发了 {}", record.name, record.event);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statistics(&self, query: &EventStatisticsQuery) -> Result<Vec<EventRecord>> {
        self.inner.get("/event/statistics", query).await
    }

    /// 奇遇攻略（VIP1）。
    ///
    /// 返回该奇遇的白名单配置（奇遇配置与触发配置合并后的结果）。
    ///
    /// # 参数
    ///
    /// * `name` —— 奇遇名称（必填，**不是角色名**）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let strategy = client.event().strategy("白日梦").await?;
    /// println!("{} {} 级", strategy.name, strategy.level);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn strategy(&self, name: impl AsRef<str>) -> Result<EventStrategy> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/event/strategy", &query).await
    }
}

impl Client {
    /// 奇遇相关接口。
    pub fn event(&self) -> EventApi {
        EventApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_query_serializes_only_set_fields() {
        let query = EventCollectQuery::new().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(!encoded.contains("num="));

        let query = EventCollectQuery::new().server("长安城").num(7);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("num=7"));
    }

    #[test]
    fn records_query_serializes_name_and_full() {
        let query = EventRecordsQuery::new()
            .server("长安城")
            .name("漓心")
            .full(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(encoded.contains("full=1"));
    }

    #[test]
    fn statistics_query_keeps_optional_server() {
        let query = EventStatisticsQuery::new().name("白日梦");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("name="));
        assert!(!encoded.contains("server="));
    }
}
