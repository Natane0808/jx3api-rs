//! 角色相关接口。
//!
//! 对应 OpenAPI 中的 `/role/*` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::role::{RoleAchievement, RoleDetailed};

/// 角色详情查询参数。
///
/// 对应接口：`GET /role/detail`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::role::RoleDetailQuery::default()
///     .server("长安城")
///     .name("漓心")
///     .history(1);
/// let detail = client.role().detail(&query).await?;
/// println!("{} {} {}", detail.role_name, detail.force_name, detail.camp_name);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct RoleDetailQuery {
    /// 服务器大区，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否返回改名 / 帮会历史，`0` 不返回（默认），`1` 返回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<i64>,
}

impl RoleDetailQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器大区（必填）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否返回改名 / 帮会历史（`0` / `1`）。
    pub fn history(mut self, history: i64) -> Self {
        self.history = Some(history);
        self
    }
}

/// 角色成就查询参数。
///
/// 对应接口：`GET /role/achievement`。
///
/// 注意参数名：角色名对应的字段是 `role`（不是 `name`），`name` 是成就名称。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct RoleAchievementQuery {
    /// 服务器名称，**必填**，例如 `长安城`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名称，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 成就名称（支持模糊匹配），**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RoleAchievementQuery {
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
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    /// 设置成就名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 角色相关接口。
///
/// 通过 [`Client::role`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::role::RoleDetailQuery::default()
///     .server("长安城")
///     .name("漓心");
/// let detail = client.role().detail(&query).await?;
/// println!("{}", detail.role_name);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RoleApi {
    inner: Arc<Inner>,
}

impl RoleApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 角色详情（VIP1）。
    ///
    /// 返回角色名片（roles 合集）。`history = 1` 时同步采集改名 / 帮会历史后再返回；
    /// `history = 0`（默认）时先返回最新结果，采集异步执行。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RoleDetailQuery`]。
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
    /// let query = jx3_api::api::role::RoleDetailQuery::default()
    ///     .server("长安城")
    ///     .name("漓心")
    ///     .history(1);
    /// let detail = client.role().detail(&query).await?;
    /// println!("{} 门派 {}", detail.role_name, detail.force_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn detail(&self, query: &RoleDetailQuery) -> Result<RoleDetailed> {
        self.inner.get("/role/detail", query).await
    }

    /// 角色成就（VIP2）。
    ///
    /// 在成就库中按 名称 / 分类 / 子类 / 详情 模糊匹配，并标记该角色是否已完成。
    /// 返回结构是 `total=False` 的超宽字典：核心字段为强类型，未知字段保留在
    /// [`RoleAchievement::extra`] 中。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`RoleAchievementQuery`]。
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
    /// let query = jx3_api::api::role::RoleAchievementQuery::default()
    ///     .server("长安城")
    ///     .role("漓心")
    ///     .name("江湖");
    /// for item in client.role().achievement(&query).await?.data {
    ///     println!("{} 已完成：{}", item.name, item.is_finished);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn achievement(&self, query: &RoleAchievementQuery) -> Result<RoleAchievement> {
        self.inner.get("/role/achievement", query).await
    }
}

impl Client {
    /// 角色相关接口。
    pub fn role(&self) -> RoleApi {
        RoleApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_query_includes_history() {
        let query = RoleDetailQuery::default()
            .server("长安城")
            .name("漓心")
            .history(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("history=1"));
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
    }

    #[test]
    fn achievement_query_uses_role_field() {
        let query = RoleAchievementQuery::default()
            .server("长安城")
            .role("漓心")
            .name("江湖");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("role=%E6%BC%93%E5%BF%83"));
        assert!(encoded.contains("name=%E6%B1%9F%E6%B9%96"));
        assert!(!encoded.contains("history"));
    }
}
