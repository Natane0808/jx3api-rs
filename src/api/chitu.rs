//! 赤兔接口。
//!
//! 对应 OpenAPI 中的 `/chitu/records`（今日）与 `/chitu/week/records`（本周）路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::chitu::{ChituRecord, ChituWeekRecord};

/// 赤兔接口。
///
/// 通过 [`Client::chitu`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder().token("jx3api::xxxx").build()?;
/// let today = client.chitu().records().await?;
/// println!("今日 {} 条记录", today.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ChituApi {
    inner: Arc<Inner>,
}

impl ChituApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 今日赤兔（VIP2）。
    ///
    /// 返回今日赤兔预告及出世记录，数据源魔盒优先于腾讯文档。
    ///
    /// # 参数
    ///
    /// 无参数。
    ///
    /// # 注意
    ///
    /// 返回模型是 [`ChituRecord`]（含 `id` / `send`），与
    /// [`ChituApi::week_records`] 的 [`ChituWeekRecord`] 不同。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码
    /// （例如当天暂无数据时返回 `暂无数据`）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// for record in client.chitu().records().await? {
    ///     println!("{} {} {}", record.server, record.map_name, record.date);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self) -> Result<Vec<ChituRecord>> {
        self.inner
            .get("/chitu/records", &Vec::<(&str, String)>::new())
            .await
    }

    /// 本周赤兔（VIP2）。
    ///
    /// 返回本周期赤兔预告及出世记录，本周期周二 07:00 重置。
    ///
    /// # 参数
    ///
    /// 无参数。
    ///
    /// # 注意
    ///
    /// 返回模型是 [`ChituWeekRecord`]（无 `id` / `send`，但有
    /// `time` / `window` / `status`），与 [`ChituApi::records`] 的
    /// [`ChituRecord`] 不同。
    ///
    /// # Errors
    ///
    /// 网络异常、令牌档位不足（需 VIP2 及以上）或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().token("jx3api::xxxx").build()?;
    /// for record in client.chitu().week_records().await? {
    ///     println!("{} {} {} {}", record.server, record.map_name, record.date, record.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn week_records(&self) -> Result<Vec<ChituWeekRecord>> {
        self.inner
            .get("/chitu/week/records", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 赤兔接口。
    pub fn chitu(&self) -> ChituApi {
        ChituApi::new(self.inner())
    }
}
