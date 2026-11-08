//! 装备属性与配装记录接口。
//!
//! 对应 OpenAPI 中的 `/calcd/*` 路径，全部为 VIP2 接口，官方 Python SDK 尚未封装。
//!
//! 模型依据 `tests/fixtures/calcd_*.json` 的真实响应推断，详见
//! [`crate::models::calcd`]。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::calcd::{CalcdAttribute, CalcdRecords};

/// 装备属性查询参数。
///
/// 对应接口：`GET /calcd/attribute`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::calcd::CalcdAttributeQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let attribute = client.calcd().attribute(&query).await?;
/// println!("{} 装分 {}", attribute.kungfu_name, attribute.equip_score);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CalcdAttributeQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 返回格式，可选，默认 `calcd`：`calcd` 返回计算后的装备详情与属性面板，
    /// `client` 返回客户端原始格式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl CalcdAttributeQuery {
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

    /// 设置返回格式（`calcd` / `client`）。
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// 装备详情查询参数。
///
/// 对应接口：`GET /calcd/detail`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CalcdDetailQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 心法名称，**必填**，例如 `冰心诀`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kungfu: Option<String>,
    /// 装备类型，**必填**，取值 `秘境挑战` / `竞技对抗` / `休闲` / `通用`。
    ///
    /// 字段名与 JSON 一致（`type`），Rust 侧用 `r#type` 表达，链式 setter 为
    /// [`CalcdDetailQuery::equip_type`]。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 返回格式，可选，默认 `calcd`：`calcd` 返回计算后的装备详情与属性面板，
    /// `client` 返回客户端原始格式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl CalcdDetailQuery {
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

    /// 设置心法名称（必填）。
    pub fn kungfu(mut self, kungfu: impl Into<String>) -> Self {
        self.kungfu = Some(kungfu.into());
        self
    }

    /// 设置装备类型（必填），取值 `秘境挑战` / `竞技对抗` / `休闲` / `通用`。
    pub fn equip_type(mut self, equip_type: impl Into<String>) -> Self {
        self.r#type = Some(equip_type.into());
        self
    }

    /// 设置返回格式（`calcd` / `client`）。
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// 装备记录查询参数。
///
/// 对应接口：`GET /calcd/records`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CalcdRecordsQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CalcdRecordsQuery {
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

/// 装备属性与配装记录接口。
///
/// 通过 [`Client::calcd`] 获取。
///
/// 三个接口都需要站点令牌 LV.2 及以上。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::calcd::CalcdRecordsQuery::default()
///     .server("长安城")
///     .name("漓心");
/// for record in client.calcd().records(&query).await?.equip_list {
///     println!("{} {} {}", record.season_name, record.kungfu_name, record.equip_type);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CalcdApi {
    inner: Arc<Inner>,
}

impl CalcdApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 装备属性（VIP2）。
    ///
    /// 转发远程查询角色装备与属性面板，默认读取缓存。返回结构极深：装备、属性面板、
    /// 套装与附魔均有强类型字段，镶嵌 / 彩石等深层子结构保留原始 JSON。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CalcdAttributeQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码（例如角色未收录）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::calcd::CalcdAttributeQuery::default()
    ///     .server("长安城")
    ///     .name("漓心")
    ///     .format("calcd");
    /// let attribute = client.calcd().attribute(&query).await?;
    /// println!("{}：{} 件装备，装分 {}", attribute.role_name, attribute.equip_list.len(), attribute.equip_score);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn attribute(&self, query: &CalcdAttributeQuery) -> Result<CalcdAttribute> {
        self.inner.get("/calcd/attribute", query).await
    }

    /// 装备详情（VIP2）。
    ///
    /// 按服务器、角色名、心法与装备类型定位历史配装。官方未给出该接口的响应
    /// schema（采样夹具是错误响应），因此直接返回原始 JSON；`format=calcd` 时的
    /// 内容与 [`CalcdApi::attribute`] 的装备部分接近。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CalcdDetailQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    pub async fn detail(&self, query: &CalcdDetailQuery) -> Result<serde_json::Value> {
        self.inner.get("/calcd/detail", query).await
    }

    /// 装备记录（VIP2）。
    ///
    /// 转发远程查询角色所有历史装备快照，按时间降序返回心法名、配装类型与时间戳。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`CalcdRecordsQuery`]。
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
    /// let query = jx3_api::api::calcd::CalcdRecordsQuery::default()
    ///     .server("长安城")
    ///     .name("漓心");
    /// let records = client.calcd().records(&query).await?;
    /// println!("{} 共 {} 套历史配装", records.role_name, records.equip_list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, query: &CalcdRecordsQuery) -> Result<CalcdRecords> {
        self.inner.get("/calcd/records", query).await
    }
}

impl Client {
    /// 装备属性与配装记录接口。
    pub fn calcd(&self) -> CalcdApi {
        CalcdApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_query_serializes_optional_format() {
        let query = CalcdAttributeQuery::default().server("长安城").name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("format"));
        let query = query.format("client");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("format=client"));
    }

    #[test]
    fn detail_query_serializes_type_keyword() {
        let query = CalcdDetailQuery::default()
            .server("长安城")
            .name("漓心")
            .kungfu("冰心诀")
            .equip_type("秘境挑战");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("type=%E7%A7%98%E5%A2%83%E6%8C%91%E6%88%98"));
        assert!(encoded.contains("kungfu=%E5%86%B0%E5%BF%83%E8%AF%80"));
        assert!(!encoded.contains("format"));
    }

    #[test]
    fn records_query_is_empty_by_default() {
        let encoded = serde_urlencoded::to_string(CalcdRecordsQuery::default()).unwrap();
        assert!(encoded.is_empty());
    }
}
