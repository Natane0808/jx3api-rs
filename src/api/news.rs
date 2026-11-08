//! 新闻公告接口。
//!
//! 对应 OpenAPI 中的 `/news/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::news::NewsRecord;

/// 新闻公告接口。
///
/// 通过 [`Client::news`] 获取。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// for record in client.news().announce(Some(3)).await? {
///     println!("{} {} {}", record.date, record.title, record.r#type);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct NewsApi {
    inner: Arc<Inner>,
}

impl NewsApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 维护公告（免费）。
    ///
    /// 返回标题含「版本更新」的官方公告，最新的在前。
    ///
    /// # 参数
    ///
    /// * `limit` —— 可选，返回条数，默认 10，取值范围 `1..=50`。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如 `limit` 越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for record in client.news().announce(Some(5)).await? {
    ///     println!("{}", record.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn announce(&self, limit: Option<u32>) -> Result<Vec<NewsRecord>> {
        self.list("/news/announce", limit).await
    }

    /// 新闻资讯（免费）。
    ///
    /// 返回官网最新资讯列表，最新的在前。
    ///
    /// # 参数
    ///
    /// * `limit` —— 可选，返回条数，默认 10，取值范围 `1..=50`。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码（例如 `limit` 越界）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// for record in client.news().records(None).await? {
    ///     println!("{} {}", record.date, record.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn records(&self, limit: Option<u32>) -> Result<Vec<NewsRecord>> {
        self.list("/news/records", limit).await
    }

    /// 两个新闻接口共用的请求实现。
    async fn list(&self, path: &str, limit: Option<u32>) -> Result<Vec<NewsRecord>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        self.inner.get(path, &query).await
    }
}

impl Client {
    /// 新闻公告接口。
    pub fn news(&self) -> NewsApi {
        NewsApi::new(self.inner())
    }
}
