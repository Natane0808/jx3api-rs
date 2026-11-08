//! 统战歪歪接口。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::duowan::DuowanStatistics;

/// 统战歪歪接口。
///
/// 通过 [`Client::duowan`] 获取。
#[derive(Debug, Clone)]
pub struct DuowanApi {
    inner: Arc<Inner>,
}

impl DuowanApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 统战歪歪（免费）。
    ///
    /// 查询各服 YY 语音频道的在线主播。指定区服时查该服两营，
    /// 不指定时每服每营取一个代表。
    ///
    /// # 参数
    ///
    /// * `server` —— 可选，区服名称。
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
    /// for item in client.duowan().statistics(None).await? {
    ///     println!("{} 共 {} 个阵营频道", item.server, item.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statistics(&self, server: Option<&str>) -> Result<Vec<DuowanStatistics>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(server) = server {
            query.push(("server", server.to_owned()));
        }
        self.inner.get("/duowan/statistics", &query).await
    }
}

impl Client {
    /// 统战歪歪接口。
    pub fn duowan(&self) -> DuowanApi {
        DuowanApi::new(self.inner())
    }
}
