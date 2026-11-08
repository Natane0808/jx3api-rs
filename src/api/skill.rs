//! 技能接口。
//!
//! 对应 OpenAPI 中的 `/skill/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::skill::{SkillCalculate, SkillRework};

/// 技能接口。
///
/// 通过 [`Client::skill`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxxxxxx")?;
/// for item in client.skill().calculate(None).await? {
///     println!("{:?} 急速 {:.2}% → {} 帧", item.level, item.haste_percent, item.now_frame);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SkillApi {
    inner: Arc<Inner>,
}

impl SkillApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 急速计算（VIP2）。
    ///
    /// 按技能 cd 推算各急速档位对应的读条帧数与实际时长；帧数相同的档位会被去重，
    /// 只保留最低急速档。响应与官方 Python SDK 的 `SkillCalculate` 一致，
    /// 保留 `u_now_frame` / `haste_percent_limit` 等 buff 版字段。
    ///
    /// # 参数
    ///
    /// * `cooldown` —— 可选，技能 cd（秒），默认 `1.5`。
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
    /// let client = Client::with_token("jx3api::xxxxxxxx")?;
    /// for item in client.skill().calculate(Some(2.5)).await? {
    ///     println!("{} 帧 / {:?} 秒", item.now_frame, item.duration);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn calculate(&self, cooldown: Option<f64>) -> Result<Vec<SkillCalculate>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(cooldown) = cooldown {
            query.push(("cooldown", cooldown.to_string()));
        }
        self.inner.get("/skill/calculate", &query).await
    }

    /// 技改记录（免费）。
    ///
    /// 返回官网公告中标题含「武学调整」的技改公告列表，最新的在前。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for item in client.skill().rework().await? {
    ///     println!("{} {} {}", item.time, item.title, item.url);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rework(&self) -> Result<Vec<SkillRework>> {
        self.inner
            .get("/skill/rework", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 技能接口。
    pub fn skill(&self) -> SkillApi {
        SkillApi::new(self.inner())
    }
}
