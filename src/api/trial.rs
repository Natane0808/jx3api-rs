//! 试炼之地相关接口。
//!
//! 对应 OpenAPI 中的 `/trial/*` 路径，两个接口均为 VIP1，官方 Python SDK
//! 尚未封装。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::trial::{TrialBosses, TrialSeason};

/// 秒伤计算查询参数。
///
/// 对应接口：`GET /trial/bosses`。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = jx3_api::api::trial::TrialBossesQuery::default()
///     .season("暗影千机")
///     .floor(1);
/// let bosses = client.trial().bosses(&query).await?;
/// println!("{} 层共 {} 个 BOSS", bosses.floor, bosses.bosses.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct TrialBossesQuery {
    /// 赛季名称，**必填**，取值 `暗影千机` / `山海源流` / `太极秘录` / `丝路风语`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    /// 层数，**必填**，取值范围 `1..=120`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<i64>,
}

impl TrialBossesQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置赛季名称（必填）。
    pub fn season(mut self, season: impl Into<String>) -> Self {
        self.season = Some(season.into());
        self
    }

    /// 设置层数（`1..=120`）。
    pub fn floor(mut self, floor: i64) -> Self {
        self.floor = Some(floor);
        self
    }
}

/// 试炼之地相关接口。
///
/// 通过 [`Client::trial`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// for season in client.trial().seasons().await? {
///     println!("{} 最高 {} 层", season.season, season.max_floor);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TrialApi {
    inner: Arc<Inner>,
}

impl TrialApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 秒伤计算（VIP1）。
    ///
    /// 查某赛季某层各 BOSS 的血量与秒伤（秒伤现算：main 组 180 / 300 完美 / 通关秒，
    /// short 组 120 / 180，分身无秒伤）。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，见 [`TrialBossesQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（VIP1 及以上）或服务端返回错误码（例如层数越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = jx3_api::api::trial::TrialBossesQuery::default()
    ///     .season("暗影千机")
    ///     .floor(1);
    /// for boss in client.trial().bosses(&query).await?.bosses {
    ///     println!("{} {} 血量 {}", boss.boss, boss.group, boss.hp);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bosses(&self, query: &TrialBossesQuery) -> Result<TrialBosses> {
        self.inner.get("/trial/bosses", query).await
    }

    /// 赛季列表（VIP1）。
    ///
    /// 返回全部赛季及各自 BOSS 顺序，可直接用于做菜单 / 参数提示。
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
    /// for season in client.trial().seasons().await? {
    ///     println!("{}：{}", season.season, season.bosses.join(" / "));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn seasons(&self) -> Result<Vec<TrialSeason>> {
        self.inner
            .get("/trial/seasons", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 试炼之地相关接口。
    pub fn trial(&self) -> TrialApi {
        TrialApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bosses_query_serializes_both_fields() {
        let query = TrialBossesQuery::default().season("暗影千机").floor(3);
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("floor=3"));
        assert!(encoded.contains("season=%E6%9A%97%E5%BD%B1%E5%8D%83%E6%9C%BA"));
    }

    #[test]
    fn bosses_query_is_empty_by_default() {
        let encoded = serde_urlencoded::to_string(TrialBossesQuery::default()).unwrap();
        assert!(encoded.is_empty());
    }
}
