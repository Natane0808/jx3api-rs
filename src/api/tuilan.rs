//! 资历分布接口。
//!
//! 对应 OpenAPI 中的 `/tuilan/achievement` 路径（VIP2）。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::tuilan::TuilanAchievement;

/// 资历分布查询参数。
///
/// 对应接口：`GET /tuilan/achievement`。
///
/// 推栏 `ticket` 为可选项，同样由 [`Client`] 全局注入，不写进本结构体。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::tuilan::TuilanAchievementQuery::default()
///     .server("长安城")
///     .name("漓心")
///     .class(1);
/// let achievement = client.tuilan().achievement(&query).await?;
/// println!("{achievement}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TuilanAchievementQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 资历分类，可选，默认 `1`，取值范围 `1..=3`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<i64>,
    /// 资历子类，可选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
}

impl TuilanAchievementQuery {
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

    /// 设置资历分类（`1..=3`）。
    pub fn class(mut self, class: i64) -> Self {
        self.class = Some(class);
        self
    }

    /// 设置资历子类。
    pub fn subclass(mut self, subclass: impl Into<String>) -> Self {
        self.subclass = Some(subclass.into());
        self
    }
}

/// 资历分布接口。
///
/// 通过 [`Client::tuilan`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::tuilan::TuilanAchievementQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let achievement = client.tuilan().achievement(&query).await?;
/// assert!(achievement.is_object());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TuilanApi {
    inner: Arc<Inner>,
}

impl TuilanApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 资历分布（VIP2）。
    ///
    /// 返回角色名片与按 子类 / 详情 / 地图 / 副本 汇总的资历完成度骨架：各桶 `total`
    /// 与排序结果按 `class` 缓存，每次请求只叠加该角色已完成的子集与 `speed`。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TuilanAchievementQuery`]。
    ///
    /// # 注意
    ///
    /// 官方返回裸字典，因此返回值为原始 JSON（[`TuilanAchievement`]）。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    pub async fn achievement(&self, query: &TuilanAchievementQuery) -> Result<TuilanAchievement> {
        self.inner.get("/tuilan/achievement", query).await
    }
}

impl Client {
    /// 资历分布接口。
    pub fn tuilan(&self) -> TuilanApi {
        TuilanApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serializes_optional_class_and_subclass() {
        let query = TuilanAchievementQuery::default()
            .server("长安城")
            .name("漓心");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(!encoded.contains("class="));
        assert!(!encoded.contains("subclass"));

        let query = query.class(2).subclass("秘境");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("class=2"));
        assert!(encoded.contains("subclass=%E7%A7%98%E5%A2%83"));
        assert!(!encoded.contains("ticket"));
    }
}
