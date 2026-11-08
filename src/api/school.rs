//! 心法配装相关接口。
//!
//! 对应 OpenAPI 中的 `/school/*` 路径。
//!
//! 本域全部接口都需要推栏 `ticket`。`ticket` **不**写进查询结构体，SDK 会从
//! [`Client`] 的全局配置自动注入，见
//! [`ClientBuilder::ticket`](crate::ClientBuilder::ticket) 与
//! [`Client::set_ticket`]。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::school::{SchoolMatrix, SchoolSeniority, SchoolSkills, SchoolTalent};

/// 资历榜单查询参数。
///
/// 对应接口：`GET /school/seniority`。
///
/// 两个参数都可省略，服务端默认取 `ALL`（全服 / 全门派）。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::school::SchoolSeniorityQuery::default()
///     .server("长安城")
///     .school("七秀");
/// let top = client.school().seniority(&query).await?;
/// println!("前 3 名：{:?}", top.iter().take(3).map(|x| &x.role).collect::<Vec<_>>());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SchoolSeniorityQuery {
    /// 服务器名称，可选，默认 `ALL`（全服）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 门派名称，可选，默认 `ALL`（全门派）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
}

impl SchoolSeniorityQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务器名称（默认 `ALL`）。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置门派名称（默认 `ALL`）。
    pub fn school(mut self, school: impl Into<String>) -> Self {
        self.school = Some(school.into());
        self
    }
}

/// 技能信息查询参数。
///
/// 对应接口：`GET /school/skills`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SchoolSkillsQuery {
    /// 门派名称，**必填**，例如 `七秀`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否强制更新推栏缓存，`0` 不更新（默认），`1` 更新。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<i64>,
}

impl SchoolSkillsQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置门派名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否强制更新缓存（`0` / `1`）。
    pub fn update(mut self, update: i64) -> Self {
        self.update = Some(update);
        self
    }
}

/// 奇穴信息查询参数。
///
/// 对应接口：`GET /school/talent`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SchoolTalentQuery {
    /// 门派名称，**必填**，例如 `七秀`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否强制更新推栏缓存，`0` 不更新（默认），`1` 更新。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<i64>,
}

impl SchoolTalentQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置门派名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否强制更新缓存（`0` / `1`）。
    pub fn update(mut self, update: i64) -> Self {
        self.update = Some(update);
        self
    }
}

/// 配装搜索查询参数。
///
/// 对应接口：`GET /school/search`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SchoolSearchQuery {
    /// 门派名称，**必填**，例如 `七秀`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 玩法模式，可选，例如 `秘境挑战`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl SchoolSearchQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置门派名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置玩法模式。
    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }
}

/// 心法配装相关接口。
///
/// 通过 [`Client::school`] 获取。五个接口都需要先配置推栏 `ticket`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().ticket("推栏 ticket").build()?;
/// let matrix = client.school().matrix("七秀").await?;
/// println!("{} 的阵眼：{}", matrix.name, matrix.skill_name);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SchoolApi {
    inner: Arc<Inner>,
}

impl SchoolApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 阵眼效果（VIP1）。
    ///
    /// 按心法取推栏阵眼（阵法）效果与描述。
    ///
    /// # 参数
    ///
    /// * `name` —— 职业（心法）名称，例如 `冰心诀`。
    ///
    /// # 注意
    ///
    /// 需要推栏 `ticket`，由 [`Client`] 全局注入。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().ticket("推栏 ticket").build()?;
    /// let matrix = client.school().matrix("冰心诀").await?;
    /// for desc in &matrix.descs {
    ///     println!("Lv{} {}：{}", desc.level, desc.name, desc.desc);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn matrix(&self, name: impl AsRef<str>) -> Result<SchoolMatrix> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/school/matrix", &query).await
    }

    /// 资历榜单（VIP1）。
    ///
    /// 返回指定服 / 门派的资历前 100 名，`server` 与 `school` 都可省略（默认全服全门派）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`SchoolSeniorityQuery`]。
    ///
    /// # 注意
    ///
    /// 需要推栏 `ticket`，由 [`Client`] 全局注入。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().ticket("推栏 ticket").build()?;
    /// let query = jx3_api::api::school::SchoolSeniorityQuery::default().school("七秀");
    /// for item in client.school().seniority(&query).await?.iter().take(10) {
    ///     println!("{} {} {}", item.role, item.server, item.value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn seniority(&self, query: &SchoolSeniorityQuery) -> Result<Vec<SchoolSeniority>> {
        self.inner.get("/school/seniority", query).await
    }

    /// 技能信息（VIP1）。
    ///
    /// 按心法取推栏各分类（remark）下的招式技能。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`SchoolSkillsQuery`]。
    ///
    /// # 注意
    ///
    /// 需要推栏 `ticket`，由 [`Client`] 全局注入。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().ticket("推栏 ticket").build()?;
    /// let query = jx3_api::api::school::SchoolSkillsQuery::default().name("七秀");
    /// for group in client.school().skills(&query).await? {
    ///     println!("{} 共 {} 个技能", group.class, group.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn skills(&self, query: &SchoolSkillsQuery) -> Result<Vec<SchoolSkills>> {
        self.inner.get("/school/skills", query).await
    }

    /// 奇穴信息（VIP1）。
    ///
    /// 按心法取推栏各等级的招式（`class = 1`）/ 奇穴（`class = 0`）技能，按等级升序。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`SchoolTalentQuery`]。
    ///
    /// # 注意
    ///
    /// 需要推栏 `ticket`，由 [`Client`] 全局注入。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP1 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().ticket("推栏 ticket").build()?;
    /// let query = jx3_api::api::school::SchoolTalentQuery::default().name("七秀");
    /// for level in client.school().talent(&query).await? {
    ///     println!("Lv{} 有 {} 条", level.level, level.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn talent(&self, query: &SchoolTalentQuery) -> Result<Vec<SchoolTalent>> {
        self.inner.get("/school/talent", query).await
    }

    /// 配装搜索（VIP2）。
    ///
    /// 按心法 + 玩法模式取推栏推荐配装；赛季与配装各缓存 7 天。官方返回结构不固定的
    /// 字典，因此直接返回原始 JSON。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`SchoolSearchQuery`]。
    ///
    /// # 注意
    ///
    /// 需要推栏 `ticket`，由 [`Client`] 全局注入。
    ///
    /// # Errors
    ///
    /// 网络异常、缺少推栏 `ticket`、令牌档位不足（VIP2 及以上）或服务端返回错误码。
    pub async fn search(&self, query: &SchoolSearchQuery) -> Result<serde_json::Value> {
        self.inner.get("/school/search", query).await
    }
}

impl Client {
    /// 心法配装相关接口。
    pub fn school(&self) -> SchoolApi {
        SchoolApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seniority_query_omits_defaults() {
        let encoded = serde_urlencoded::to_string(SchoolSeniorityQuery::default()).unwrap();
        assert!(encoded.is_empty());
        let query = SchoolSeniorityQuery::default()
            .server("长安城")
            .school("七秀");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("server=%E9%95%BF%E5%AE%89%E5%9F%8E"));
        assert!(encoded.contains("school=%E4%B8%83%E7%A7%80"));
    }

    #[test]
    fn skills_query_includes_update() {
        let query = SchoolSkillsQuery::default().name("七秀").update(1);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("update=1"));
        assert!(encoded.contains("name=%E4%B8%83%E7%A7%80"));
    }

    #[test]
    fn search_query_omits_mode_when_unset() {
        let query = SchoolSearchQuery::default().name("七秀");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(encoded, "name=%E4%B8%83%E7%A7%80");
        let query = query.mode("秘境挑战");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("mode=%E7%A7%98%E5%A2%83%E6%8C%91%E6%88%98"));
    }

    #[test]
    fn ticket_is_not_part_of_query() {
        // ticket 由 Client 全局注入，查询结构体里不应出现该字段。
        let encoded =
            serde_urlencoded::to_string(SchoolTalentQuery::default().name("七秀")).unwrap();
        assert!(!encoded.contains("ticket"));
    }
}
