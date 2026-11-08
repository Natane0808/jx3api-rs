//! 家园接口。
//!
//! 对应 OpenAPI 中的 `/home/*` 路径。

use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::home::{HomeFlowerItem, HomeFurniture, HomeTravel};

/// 家园鲜花查询参数。
///
/// 对应接口：`GET /home/flower`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::home::HomeFlowerQuery;
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = HomeFlowerQuery::default().server("长安城").map("广陵邑");
/// let flowers = client.home().flower(&query).await?;
/// println!("{} 个地图有行情", flowers.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct HomeFlowerQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 鲜花名称，可选，例如 `一级玫瑰`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地图名称，可选，例如 `广陵邑`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
}

impl HomeFlowerQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置鲜花名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置地图名称。
    pub fn map(mut self, map: impl Into<String>) -> Self {
        self.map = Some(map.into());
        self
    }
}

/// 家园接口。
///
/// 通过 [`Client::home`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// for item in client.home().furniture("龙门香梦").await? {
///     println!("{} 品质 {}", item.name, item.quality);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct HomeApi {
    inner: Arc<Inner>,
}

impl HomeApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 家园鲜花行情（免费）。
    ///
    /// 查询指定区服的鲜花价格，按地图（例如 `广陵邑`、`枫叶泊·天苑`）归组，键为地图名称。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，`server` 必填，`name` / `map` 可选，见 [`HomeFlowerQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如缺少 `server`）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::api::home::HomeFlowerQuery;
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = HomeFlowerQuery::default().server("长安城");
    /// for (map, items) in client.home().flower(&query).await? {
    ///     println!("{map}：{} 种鲜花", items.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn flower(
        &self,
        query: &HomeFlowerQuery,
    ) -> Result<HashMap<String, Vec<HomeFlowerItem>>> {
        self.inner.get("/home/flower", query).await
    }

    /// 家园装饰（免费）。
    ///
    /// 按装饰名称查询家园装饰的属性（品质、风水、趣味等）与图标。
    ///
    /// # 参数
    ///
    /// * `name` —— 装饰名称，例如 `龙门香梦`。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如装饰名称不存在）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for item in client.home().furniture("龙门香梦").await? {
    ///     println!("{} 类型 {}", item.name, item.r#type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn furniture(&self, name: impl AsRef<str>) -> Result<Vec<HomeFurniture>> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/home/furniture", &query).await
    }

    /// 器物图谱（免费）。
    ///
    /// 按器物名称查询家园器物（`produce` 列）的属性与图标，结构同
    /// [`HomeFurniture`]。
    ///
    /// # 参数
    ///
    /// * `name` —— 器物名称。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如器物名称不存在）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for item in client.home().travel("龙门香梦").await? {
    ///     println!("{} 产出 {:?}", item.name, item.produce);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn travel(&self, name: impl AsRef<str>) -> Result<Vec<HomeTravel>> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/home/travel", &query).await
    }
}

impl Client {
    /// 家园接口。
    pub fn home(&self) -> HomeApi {
        HomeApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flower_query_serializes_only_set_fields() {
        let query = HomeFlowerQuery::default().server("长安城");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "server=%E9%95%BF%E5%AE%89%E5%9F%8E");
    }

    #[test]
    fn flower_query_serializes_all_fields() {
        let query = HomeFlowerQuery::default()
            .server("长安城")
            .name("一级玫瑰")
            .map("广陵邑");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server="));
        assert!(encoded.contains("name="));
        assert!(encoded.contains("map="));
    }
}
