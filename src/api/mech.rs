//! 秘境方位相关接口。
//!
//! 对应 OpenAPI 中的 `/mech/decrypt` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::mech::MechDecrypt;

/// 秘境方位接口。
///
/// 通过 [`Client::mech`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::with_token("jx3api::xxxx")?;
/// let decrypt = client.mech().decrypt().await?;
/// println!("{}：{}", decrypt.now_time, decrypt.now_result);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MechApi {
    inner: Arc<Inner>,
}

impl MechApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 秘境方位（VIP1）。
    ///
    /// 按 15 分钟档推算当前方位、下次变化时刻与倒计时。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP1）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_token("jx3api::xxxx")?;
    /// let decrypt = client.mech().decrypt().await?;
    /// println!("当前方位：{}", decrypt.now_result);
    /// println!("下次变化：{}（{} 后）", decrypt.next_node, decrypt.interval_time);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn decrypt(&self) -> Result<MechDecrypt> {
        self.inner
            .get("/mech/decrypt", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 秘境方位接口。
    pub fn mech(&self) -> MechApi {
        MechApi::new(self.inner())
    }
}
