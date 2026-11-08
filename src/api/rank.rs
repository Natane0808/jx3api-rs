//! 排行榜相关接口。
//!
//! 对应 OpenAPI 中的 `/rank/*` 路径。
//!
//! 其中 `/rank/statistics`、`/rank/trials` 有完整模型；`/rank/arena`、
//! `/rank/championship`、`/rank/constable`、`/rank/outlaw`、`/rank/wanted`
//! 的条目字段官方未定型，按原始 JSON（[`serde_json::Value`]）返回。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::rank::{RankStatistics, RankTrials};

/// 排行统计查询参数。
///
/// 对应接口：`GET /rank/statistics`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::rank::RankStatisticsQuery;
/// let query = RankStatisticsQuery::new().name("帮会威望榜");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RankStatisticsQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 榜单名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RankStatisticsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置榜单名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 试炼之地排行查询参数。
///
/// 对应接口：`GET /rank/trials`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::rank::RankTrialsQuery;
/// let query = RankTrialsQuery::new().name("试炼之地");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RankTrialsQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 榜单名称（必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RankTrialsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置榜单名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 跨服名剑榜查询参数。
///
/// 对应接口：`GET /rank/arena`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::rank::RankArenaQuery;
/// let query = RankArenaQuery::new().server("长安城").mode(1);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RankArenaQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 竞技模式，`0`=2v2 / `1`=3v3 / `2`=5v5，默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
}

impl RankArenaQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置竞技模式（`0..=2`）。
    pub fn mode(mut self, mode: i64) -> Self {
        self.mode = Some(mode);
        self
    }
}

/// 武林争霸榜查询参数。
///
/// 对应接口：`GET /rank/championship`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::rank::RankChampionshipQuery;
/// let query = RankChampionshipQuery::new().server("长安城").camp(1);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RankChampionshipQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 阵营，`1`=浩气 / `2`=恶人，默认 `1`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camp: Option<i64>,
}

impl RankChampionshipQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置阵营（`1`=浩气 / `2`=恶人）。
    pub fn camp(mut self, camp: i64) -> Self {
        self.camp = Some(camp);
        self
    }
}

/// 决斗挑战榜查询参数。
///
/// 对应接口：`GET /rank/wanted`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::rank::RankWantedQuery;
/// let query = RankWantedQuery::new().server("长安城").mode(1);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct RankWantedQuery {
    /// 服务器名称（可选，不传则查询全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 悬赏类型，`1`=公开 / `2`=私密，默认 `1`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
}

impl RankWantedQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置悬赏类型（`1`=公开 / `2`=私密）。
    pub fn mode(mut self, mode: i64) -> Self {
        self.mode = Some(mode);
        self
    }
}

/// 排行榜接口。
///
/// 通过 [`Client::rank`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # use jx3_api::api::rank::RankStatisticsQuery;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let query = RankStatisticsQuery::new().name("帮会威望榜");
/// let rank = client.rank().statistics(&query).await?;
/// println!("{} 共 {} 条", rank.name, rank.data.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RankApi {
    inner: Arc<Inner>,
}

impl RankApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 排行统计（VIP1）。
    ///
    /// 查询指定榜单；指定 `server` 时查该服，否则查全服。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RankStatisticsQuery`]。
    ///   `name`（榜单名称）必填，`server` 可选。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::rank::RankStatisticsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RankStatisticsQuery::new().server("长安城").name("帮会威望榜");
    /// for item in client.rank().statistics(&query).await?.data {
    ///     println!("{} {}", item.tong_name, item.total_score);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statistics(&self, query: &RankStatisticsQuery) -> Result<RankStatistics> {
        self.inner.get("/rank/statistics", query).await
    }

    /// 试炼之地排行（VIP1）。
    ///
    /// 查询试炼之地榜单；山居剑意会归并到藏剑，返回的行字段已重建成具名字段。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RankTrialsQuery`]。
    ///   `name`（榜单名称）必填，`server` 可选。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::rank::RankTrialsQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RankTrialsQuery::new().name("试炼之地");
    /// for item in client.rank().trials(&query).await?.data {
    ///     println!("{} 最高 {} 层", item.role_name, item.max_level);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trials(&self, query: &RankTrialsQuery) -> Result<RankTrials> {
        self.inner.get("/rank/trials", query).await
    }

    /// 跨服名剑榜（VIP2）。
    ///
    /// 按竞技模式查询军团赛季战绩榜；指定 `server` 时查单服，否则查全服。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RankArenaQuery`]。
    ///   `server` 可选，`mode` 默认 `0`（`0..=2`）。
    ///
    /// # 返回
    ///
    /// 官方条目字段未定型，逐条以 [`serde_json::Value`] 返回；SDK 会从
    /// `{ server, name, mode, data: [...], time }` 包络中取出 `data` 数组。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::rank::RankArenaQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RankArenaQuery::new().server("长安城").mode(1);
    /// for item in client.rank().arena(&query).await? {
    ///     println!("{}", item["corpsName"]);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn arena(&self, query: &RankArenaQuery) -> Result<Vec<serde_json::Value>> {
        let value: serde_json::Value = self.inner.get("/rank/arena", query).await?;
        Ok(crate::models::rank::extract_rank_entries(value))
    }

    /// 武林争霸榜（VIP2）。
    ///
    /// 按阵营查询帮会争霸赛季榜（与名剑榜共用赛季合集，每赛季清零）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RankChampionshipQuery`]。
    ///   `server` 可选，`camp` 默认 `1`（`1`=浩气 / `2`=恶人）。
    ///
    /// # 返回
    ///
    /// 官方条目字段未定型，逐条以 [`serde_json::Value`] 返回；SDK 会从
    /// `{ server, name, mode, data: [...], time }` 包络中取出 `data` 数组。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::rank::RankChampionshipQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RankChampionshipQuery::new().camp(1);
    /// println!("共 {} 个帮会", client.rank().championship(&query).await?.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn championship(
        &self,
        query: &RankChampionshipQuery,
    ) -> Result<Vec<serde_json::Value>> {
        let value: serde_json::Value = self.inner.get("/rank/championship", query).await?;
        Ok(crate::models::rank::extract_rank_entries(value))
    }

    /// 捕快荣誉榜（VIP2）。
    ///
    /// 与江湖浪客榜共用 `bounty` 合集的字段格式。
    ///
    /// # 参数
    ///
    /// * `server` —— 服务器名称，`None` 时查询全服。
    ///
    /// # 返回
    ///
    /// 官方条目字段未定型，逐条以 [`serde_json::Value`] 返回；SDK 会从
    /// `{ server, mode, name, data: [...], time }` 包络中取出 `data` 数组。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// for item in client.rank().constable(Some("长安城")).await? {
    ///     println!("{}", item["name"]);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn constable(&self, server: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(server) = server {
            query.push(("server", server.to_owned()));
        }
        let value: serde_json::Value = self.inner.get("/rank/constable", &query).await?;
        Ok(crate::models::rank::extract_rank_entries(value))
    }

    /// 江湖浪客榜（VIP2）。
    ///
    /// 实时榜（`bounty` 合集），显式列出字段，但不返回排名序号。
    ///
    /// # 参数
    ///
    /// * `server` —— 服务器名称，`None` 时查询全服。
    ///
    /// # 返回
    ///
    /// 官方条目字段未定型，逐条以 [`serde_json::Value`] 返回；SDK 会从
    /// `{ server, mode, name, data: [...], time }` 包络中取出 `data` 数组。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// for item in client.rank().outlaw(Some("长安城")).await? {
    ///     println!("{} {}", item["name"], item["score"]);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn outlaw(&self, server: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(server) = server {
            query.push(("server", server.to_owned()));
        }
        let value: serde_json::Value = self.inner.get("/rank/outlaw", &query).await?;
        Ok(crate::models::rank::extract_rank_entries(value))
    }

    /// 决斗挑战榜（VIP2）。
    ///
    /// 按类型查询决斗悬赏榜（`bounty` 合集，`mode` 在官方数据中以字符串存储）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RankWantedQuery`]。
    ///   `server` 可选，`mode` 默认 `1`（`1`=公开 / `2`=私密）。
    ///
    /// # 返回
    ///
    /// 官方条目字段未定型，逐条以 [`serde_json::Value`] 返回；SDK 会从
    /// `{ mode, name, server, data: [...], time }` 包络中取出 `data` 数组。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # use jx3_api::api::rank::RankWantedQuery;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let query = RankWantedQuery::new().server("长安城").mode(1);
    /// println!("共 {} 条悬赏", client.rank().wanted(&query).await?.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wanted(&self, query: &RankWantedQuery) -> Result<Vec<serde_json::Value>> {
        let value: serde_json::Value = self.inner.get("/rank/wanted", query).await?;
        Ok(crate::models::rank::extract_rank_entries(value))
    }
}

impl Client {
    /// 排行榜接口。
    pub fn rank(&self) -> RankApi {
        RankApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statistics_query_serializes_name() {
        let query = RankStatisticsQuery::new()
            .server("长安城")
            .name("帮会威望榜");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));

        let query = RankStatisticsQuery::new().name("帮会威望榜");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("server="));
    }

    #[test]
    fn arena_query_serializes_mode() {
        let query = RankArenaQuery::new().server("长安城").mode(2);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("mode=2"));
        assert!(encoded.contains("server="));
    }

    #[test]
    fn championship_and_wanted_queries() {
        let query = RankChampionshipQuery::new().camp(2);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "camp=2");

        let query = RankWantedQuery::new().mode(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "mode=1");
    }
}
