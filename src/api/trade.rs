//! 交易行相关接口。
//!
//! 对应 OpenAPI 中的 `/trade/*` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::trade::{TradeDemon, TradeItem, TradeManufacture, TradeRecords};

/// 金价行情查询参数。
///
/// 对应接口：`GET /trade/demon`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::trade::TradeDemonQuery::default().server("长安城");
/// for price in client.trade().demon(&query).await? {
///     println!("{} 万宝楼 {}", price.server, price.wanbaolou);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TradeDemonQuery {
    /// 服务器名称，可选；不指定时每服取最新一条。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 显示数量，可选，默认 `10`，取值范围 `1..=100`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TradeDemonQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置显示数量（`1..=100`）。
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// 外观价格查询参数。
///
/// 对应接口：`GET /trade/records`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TradeRecordsQuery {
    /// 物品名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 服务器名称，可选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

impl TradeRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置物品名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }
}

/// 外观价格与挂单查询参数。
///
/// 对应接口：`GET /trade/item/records`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TradeItemRecordsQuery {
    /// 物品名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 服务器名称，可选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

impl TradeItemRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置物品名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置服务器名称。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }
}

/// 成本计算查询参数。
///
/// 对应接口：`GET /trade/manufacture`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TradeManufactureQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 成品名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 价格来源，可选，默认 `0`：`0` = 交易行、`1` = jx3box。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i64>,
}

impl TradeManufactureQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置成品名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置价格来源（`0` 交易行 / `1` jx3box）。
    pub fn source(mut self, source: i64) -> Self {
        self.source = Some(source);
        self
    }
}

/// 交易行相关接口。
///
/// 通过 [`Client::trade`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::trade::TradeDemonQuery::default().limit(5);
/// for price in client.trade().demon(&query).await? {
///     println!("{}：{}", price.server, price.tieba);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TradeApi {
    inner: Arc<Inner>,
}

impl TradeApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 金价行情（VIP1）。
    ///
    /// 返回各渠道金币比价；指定服务器时返回该服最近 `limit` 条，否则每服最新一条。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TradeDemonQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::trade::TradeDemonQuery::default()
    ///     .server("长安城")
    ///     .limit(10);
    /// for price in client.trade().demon(&query).await? {
    ///     println!("{} 贴吧 {} / 万宝楼 {}", price.date, price.tieba, price.wanbaolou);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn demon(&self, query: &TradeDemonQuery) -> Result<Vec<TradeDemon>> {
        self.inner.get("/trade/demon", query).await
    }

    /// 外观价格与挂单（VIP1）。
    ///
    /// 精确定位外观后取各区 / 指定服历史成交，并附万宝楼两期挂单。与
    /// [`TradeApi::records`] 共用 [`TradeRecords`] 模型。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TradeItemRecordsQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、外观未收录、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    pub async fn item_records(&self, query: &TradeItemRecordsQuery) -> Result<TradeRecords> {
        self.inner.get("/trade/item/records", query).await
    }

    /// 外观搜索（VIP1）。
    ///
    /// 按名称查外观：先精确匹配 `name` / `alias` / `feiniu`，无结果再模糊匹配。
    ///
    /// # 参数
    ///
    /// * `name` —— 物品名称。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for item in client.trade().item_search("同人白发").await? {
    ///     println!("{}（{}）", item.name, item.category);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn item_search(&self, name: impl AsRef<str>) -> Result<Vec<TradeItem>> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/trade/item/search", &query).await
    }

    /// 外观价格（VIP1）。
    ///
    /// 定位外观后取各区 / 指定服历史成交价（各最近 10 条）。与
    /// [`TradeApi::item_records`] 共用 [`TradeRecords`] 模型。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TradeRecordsQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、外观未收录、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::trade::TradeRecordsQuery::default().name("同人白发");
    /// let records = client.trade().records(&query).await?;
    /// println!("{} 共 {} 个服务器有成交", records.name, records.list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &TradeRecordsQuery) -> Result<TradeRecords> {
        self.inner.get("/trade/records", query).await
    }

    /// 成本计算（VIP2）。
    ///
    /// 按成品配方算材料成本：商店材料取固定售价，交易行材料与成品取实时价。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TradeManufactureQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、配方不存在、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::trade::TradeManufactureQuery::default()
    ///     .server("长安城")
    ///     .name("菜团子");
    /// let cost = client.trade().manufacture(&query).await?;
    /// println!("{} 平均成本 {}", cost.name, cost.total.avg_total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn manufacture(&self, query: &TradeManufactureQuery) -> Result<TradeManufacture> {
        self.inner.get("/trade/manufacture", query).await
    }

    /// 编号搜索（VIP2）。
    ///
    /// 按角色编号查万宝楼账号详情。官方返回结构不固定的字典，因此直接返回原始 JSON。
    ///
    /// # 参数
    ///
    /// * `id` —— 万宝楼角色编号。
    ///
    /// # Errors
    ///
    /// 网络异常、编号未收录、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    pub async fn wanbaolou(&self, id: impl AsRef<str>) -> Result<serde_json::Value> {
        let query = vec![("id", id.as_ref().to_owned())];
        self.inner.get("/trade/wanbaolou", &query).await
    }
}

impl Client {
    /// 交易行相关接口。
    pub fn trade(&self) -> TradeApi {
        TradeApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demon_query_omits_optional_fields() {
        let encoded = serde_urlencoded::to_string(TradeDemonQuery::default()).unwrap();
        assert!(encoded.is_empty());
        let query = TradeDemonQuery::default().server("长安城").limit(5);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("limit=5"));
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
    }

    #[test]
    fn records_queries_share_shape() {
        let a = TradeRecordsQuery::default()
            .name("同人白发")
            .server("长安城");
        let b = TradeItemRecordsQuery::default()
            .name("同人白发")
            .server("长安城");
        assert_eq!(
            serde_urlencoded::to_string(&a).unwrap(),
            serde_urlencoded::to_string(&b).unwrap()
        );
    }

    #[test]
    fn manufacture_query_includes_source() {
        let query = TradeManufactureQuery::default()
            .server("长安城")
            .name("菜团子")
            .source(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("source=1"));
        assert!(encoded.contains("name=%E8%8F%9C%E5%9B%A2%E5%AD%90"));
    }
}
