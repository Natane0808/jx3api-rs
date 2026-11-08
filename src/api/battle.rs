//! 帮战记录接口。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::battle::BattleRecord;

/// 帮战记录接口。
///
/// 通过 [`Client::battle`] 获取。
#[derive(Debug, Clone)]
pub struct BattleApi {
    inner: Arc<Inner>,
}

impl BattleApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 帮战记录（免费）。
    ///
    /// 查询某服最近 30 场帮战，区分已结束 / 进行中，并算出时长与结束时间。
    ///
    /// # 参数
    ///
    /// * `server` —— 区服名称，例如 `长安城`。
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
    /// for record in client.battle().records("长安城").await? {
    ///     println!("{} vs {}", record.declaring_name, record.accepting_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, server: impl AsRef<str>) -> Result<Vec<BattleRecord>> {
        let query = vec![("server", server.as_ref().to_owned())];
        self.inner.get("/battle/records", &query).await
    }
}

impl Client {
    /// 帮战记录接口。
    pub fn battle(&self) -> BattleApi {
        BattleApi::new(self.inner())
    }
}
