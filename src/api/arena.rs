//! 名剑大会相关接口。
//!
//! 对应 OpenAPI 中的 `/arena/*` 路径。三个接口都要求推栏 `ticket`，由
//! [`Client`] 全局注入，不需要（也不应该）写进查询结构体。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::arena::{ArenaAwesome, ArenaRecent, ArenaSchools};

/// 名剑排行查询参数。
///
/// 对应接口：`GET /arena/awesome`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::arena::ArenaAwesomeQuery;
/// let query = ArenaAwesomeQuery::new().mode(33).limit(20);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct ArenaAwesomeQuery {
    /// 比赛模式编号，官方默认 `33`（必填，未设置时由服务端取默认值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// 显示数量，默认 20，取值范围 `1..=100`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ArenaAwesomeQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置比赛模式编号。
    pub fn mode(mut self, mode: i64) -> Self {
        self.mode = Some(mode);
        self
    }

    /// 设置显示数量（`1..=100`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 名剑战绩查询参数。
///
/// 对应接口：`GET /arena/recent`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::arena::ArenaRecentQuery;
/// let query = ArenaRecentQuery::new().server("长安城").name("漓心");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct ArenaRecentQuery {
    /// 服务器名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 比赛模式，可选，官方默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl ArenaRecentQuery {
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

    /// 设置比赛模式。
    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }
}

/// 名剑大会接口。
///
/// 通过 [`Client::arena`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::arena::ArenaAwesomeQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").ticket("ticket").build()?;
/// let query = ArenaAwesomeQuery::new().mode(33).limit(20);
/// for item in client.arena().awesome(&query).await? {
///     println!("{} {} {}", item.rank_num, item.role_name, item.score);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ArenaApi {
    inner: Arc<Inner>,
}

impl ArenaApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 名剑排行（VIP1）。
    ///
    /// 返回指定比赛模式下的竞技场排行榜前 `limit` 名。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`ArenaAwesomeQuery`]。
    ///   `mode` 为比赛模式编号（默认 `33`），`limit` 为显示数量（默认 20，
    ///   取值范围 `1..=100`）。
    ///
    /// # 注意
    ///
    /// 该接口需要推栏 `ticket`，请通过 `ClientBuilder::ticket` 或
    /// [`Client::set_ticket`] 配置，SDK 会自动注入查询串。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 ticket，或服务端返回错误码（例如参数为空）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::arena::ArenaAwesomeQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").ticket("ticket").build()?;
    /// let query = ArenaAwesomeQuery::new().mode(33).limit(20);
    /// let ranks = client.arena().awesome(&query).await?;
    /// println!("共 {} 名", ranks.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn awesome(&self, query: &ArenaAwesomeQuery) -> Result<Vec<ArenaAwesome>> {
        self.inner.get("/arena/awesome", query).await
    }

    /// 名剑战绩（VIP1）。
    ///
    /// 返回角色竞技场名片、各模式战绩表现与历史对战记录；指定 `mode` 时附带趋势。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`ArenaRecentQuery`]。
    ///   `server` 与 `name` 必填，`mode` 可选（默认 `0`）。
    ///
    /// # 注意
    ///
    /// 该接口需要推栏 `ticket`，请通过 `ClientBuilder::ticket` 或
    /// [`Client::set_ticket`] 配置。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 ticket，或服务端返回错误码（例如缺少角色名）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::arena::ArenaRecentQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").ticket("ticket").build()?;
    /// let query = ArenaRecentQuery::new().server("长安城").name("漓心");
    /// let recent = client.arena().recent(&query).await?;
    /// println!("{} 共 {} 场历史对战", recent.role_name, recent.history.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn recent(&self, query: &ArenaRecentQuery) -> Result<ArenaRecent> {
        self.inner.get("/arena/recent", query).await
    }

    /// 名剑统计（VIP1）。
    ///
    /// 返回指定比赛模式下各门派的竞技场周统计。
    ///
    /// # 参数
    ///
    /// * `mode` —— 比赛模式编号，`None` 时服务端使用默认值 `33`。
    ///
    /// # 注意
    ///
    /// 该接口需要推栏 `ticket`，请通过 `ClientBuilder::ticket` 或
    /// [`Client::set_ticket`] 配置。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 ticket，或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").ticket("ticket").build()?;
    /// for school in client.arena().schools(Some(33)).await? {
    ///     println!("{} 本周 {} 上周 {}", school.name, school.this, school.last);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn schools(&self, mode: Option<i64>) -> Result<Vec<ArenaSchools>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(mode) = mode {
            query.push(("mode", mode.to_string()));
        }
        self.inner.get("/arena/schools", &query).await
    }
}

impl Client {
    /// 名剑大会接口。
    pub fn arena(&self) -> ArenaApi {
        ArenaApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn awesome_query_serializes_only_set_fields() {
        let empty = ArenaAwesomeQuery::new();
        assert_eq!(serde_urlencoded::to_string(&empty).unwrap(), "");

        let query = ArenaAwesomeQuery::new().mode(33).limit(20);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("mode=33"));
        assert!(encoded.contains("limit=20"));
    }

    #[test]
    fn recent_query_serializes_all_fields() {
        let query = ArenaRecentQuery::new()
            .server("长安城")
            .name("漓心")
            .mode("1");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(encoded.contains("mode=1"));
    }
}
