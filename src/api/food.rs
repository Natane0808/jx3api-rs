//! 小吃小药接口。
//!
//! 对应 OpenAPI 中的 `/food/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::food::SchoolFood;

/// 小吃小药接口。
///
/// 通过 [`Client::food`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let foods = client.food().list(Some("花间游")).await?;
/// for food in foods {
///     println!("{} {}：{}", food.school, food.kungfu, food.name);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FoodApi {
    inner: Arc<Inner>,
}

impl FoodApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 小吃小药（免费）。
    ///
    /// 返回各门派心法对应的小吃小药（增强 / 辅助，食品 / 药品），按心法分组排列。
    ///
    /// # 参数
    ///
    /// * `name` —— 可选，心法名称，例如 `花间游`；不传则返回全部心法。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如心法名称不存在）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let foods = client.food().list(None).await?;
    /// println!("共 {} 条", foods.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, name: Option<&str>) -> Result<Vec<SchoolFood>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(name) = name {
            query.push(("name", name.to_owned()));
        }
        self.inner.get("/food/list", &query).await
    }
}

impl Client {
    /// 小吃小药接口。
    pub fn food(&self) -> FoodApi {
        FoodApi::new(self.inner())
    }
}
