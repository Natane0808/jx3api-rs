//! 名片相关接口。
//!
//! 对应 OpenAPI 中的 `/card/*` 路径，全部为 VIP2 接口。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::card::{CardCached, CardHistory, CardRandom, CardRecord};

/// 名片缓存查询参数。
///
/// 对应接口：`GET /card/cached`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::card::CardCachedQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let card = client.card().cached(&query).await?;
/// println!("{} 的名片：{}", card.role_name, card.show_avatar);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CardCachedQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CardCachedQuery {
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

/// 名片预设查询参数。
///
/// 对应接口：`GET /card/preset`。
///
/// 该接口还需要推栏 `ticket`，SDK 会从 [`Client`] 上的全局配置自动注入，
/// 因此无需（也不应）写进本结构体。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CardPresetQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CardPresetQuery {
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

/// 随机名片查询参数。
///
/// 对应接口：`GET /card/random`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CardRandomQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 门派名称，可选，例如 `七秀`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,
    /// 体型名称，可选，例如 `成女`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl CardRandomQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置门派名称。
    pub fn force(mut self, force: impl Into<String>) -> Self {
        self.force = Some(force.into());
        self
    }

    /// 设置体型名称。
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

/// 角色名片查询参数。
///
/// 对应接口：`GET /card/record`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CardRecordQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CardRecordQuery {
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

/// 名片历史查询参数。
///
/// 对应接口：`GET /card/records`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CardRecordsQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否只看已点赞的名片，默认 `false`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<bool>,
}

impl CardRecordsQuery {
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

    /// 设置是否只看已点赞的名片。
    pub fn like(mut self, like: bool) -> Self {
        self.like = Some(like);
        self
    }
}

/// 名片相关接口。
///
/// 通过 [`Client::card`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::card::CardRecordQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let record = client.card().record(&query).await?;
/// println!("{} 获赞 {}", record.role_name, record.show_like);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CardApi {
    inner: Arc<Inner>,
}

impl CardApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 名片缓存（VIP2）。
    ///
    /// 转发远程取已缓存的名片。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CardCachedQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码（例如 `name` 为空）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::card::CardCachedQuery::default()
    ///     .server("长安城")
    ///     .name("漓心");
    /// let card = client.card().cached(&query).await?;
    /// println!("{}", card.file_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cached(&self, query: &CardCachedQuery) -> Result<CardCached> {
        self.inner.get("/card/cached", query).await
    }

    /// 名片预设（VIP2）。
    ///
    /// 取角色推栏名片预设图与获赞数，URL 解密后返回。官方返回结构不固定的字典，
    /// 因此直接返回原始 JSON。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CardPresetQuery`]。
    ///
    /// # 注意
    ///
    /// 本接口需要推栏 `ticket`，请通过
    /// [`ClientBuilder::ticket`](crate::ClientBuilder::ticket) 或
    /// [`Client::set_ticket`] 预先配置。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    pub async fn preset(&self, query: &CardPresetQuery) -> Result<serde_json::Value> {
        self.inner.get("/card/preset", query).await
    }

    /// 随机名片（VIP2）。
    ///
    /// 按门派 / 体型随机取一张名片，两个筛选参数都可省略。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CardRandomQuery`]。
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
    /// let query = jx3_api::api::card::CardRandomQuery::default()
    ///     .server("长安城")
    ///     .force("七秀")
    ///     .body("成女");
    /// let card = client.card().random(&query).await?;
    /// println!("{}：{}", card.role_name, card.show_hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn random(&self, query: &CardRandomQuery) -> Result<CardRandom> {
        self.inner.get("/card/random", query).await
    }

    /// 角色名片（VIP2）。
    ///
    /// 转发远程取角色当前展示的名片。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CardRecordQuery`]。
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
    /// let query = jx3_api::api::card::CardRecordQuery::default()
    ///     .server("长安城")
    ///     .name("漓心");
    /// let card = client.card().record(&query).await?;
    /// println!("展示中：{}，点赞 {}", card.show_active, card.show_like);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record(&self, query: &CardRecordQuery) -> Result<CardRecord> {
        self.inner.get("/card/record", query).await
    }

    /// 名片历史（VIP2）。
    ///
    /// 转发远程取角色名片历史列表；`like = true` 时只返回已点赞的名片。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CardRecordsQuery`]。
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
    /// let query = jx3_api::api::card::CardRecordsQuery::default()
    ///     .server("长安城")
    ///     .name("漓心")
    ///     .like(true);
    /// for card in client.card().records(&query).await? {
    ///     println!("#{} {}", card.show_index, card.show_avatar);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &CardRecordsQuery) -> Result<Vec<CardHistory>> {
        self.inner.get("/card/records", query).await
    }
}

impl Client {
    /// 名片相关接口。
    pub fn card(&self) -> CardApi {
        CardApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_query_serializes_set_fields_only() {
        let query = CardCachedQuery::default().server("长安城").name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
        assert!(encoded.contains("name=%E6%BC%93%E5%BF%83"));
        assert!(!encoded.contains("like"));
    }

    #[test]
    fn random_query_omits_optional_filters() {
        let query = CardRandomQuery::default().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "server=%E9%95%BF%E5%AE%89%E5%9F%8E");
    }

    #[test]
    fn records_query_includes_like() {
        let query = CardRecordsQuery::default().like(true);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "like=true");
    }
}
