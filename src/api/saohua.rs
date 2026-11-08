//! 骚话接口。
//!
//! 对应 OpenAPI 中的 `/saohua/*` 路径。

use std::sync::Arc;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::saohua::{
    SaohuaAnswer, SaohuaContent, SaohuaContext, SaohuaRandom, SaohuaZhanan,
};

/// 骚话接口。
///
/// 通过 [`Client::saohua`] 获取，全部接口都是一次返回一条（或若干条）随机文案。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let answer = client.saohua().answer().await?;
/// println!("{} {}", answer.answer, answer.hearten);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SaohuaApi {
    inner: Arc<Inner>,
}

impl SaohuaApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 答案之书（免费）。
    ///
    /// 随机返回一条答案，外加一条鼓励语。
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
    /// let answer = client.saohua().answer().await?;
    /// println!("答案：{}，鼓励：{}", answer.answer, answer.hearten);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn answer(&self) -> Result<SaohuaAnswer> {
        self.inner
            .get("/saohua/answer", &Vec::<(&str, String)>::new())
            .await
    }

    /// 舔狗日志（免费）。
    ///
    /// 从舔狗日志库随机取一条。
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
    /// println!("{}", client.saohua().content().await?.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn content(&self) -> Result<SaohuaContent> {
        self.inner
            .get("/saohua/content", &Vec::<(&str, String)>::new())
            .await
    }

    /// 分类语录（VIP2）。
    ///
    /// 按分类随机取一条语录。
    ///
    /// # 参数
    ///
    /// * `name` —— 语录分类，可选值：`疯狂星期四` / `彩虹屁` / `毒鸡汤` / `朋友圈`。
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
    /// println!("{}", client.saohua().context("疯狂星期四").await?.content);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn context(&self, name: impl AsRef<str>) -> Result<SaohuaContext> {
        let query = vec![("name", name.as_ref().to_owned())];
        self.inner.get("/saohua/context", &query).await
    }

    /// 喝什么（免费）。
    ///
    /// 从饮品库随机返回 2～4 条推荐。
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
    /// for drink in client.saohua().drink().await? {
    ///     println!("{drink}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn drink(&self) -> Result<Vec<String>> {
        self.inner
            .get("/saohua/drink", &Vec::<(&str, String)>::new())
            .await
    }

    /// 吃什么（免费）。
    ///
    /// 从食物库随机返回 2～4 条推荐。
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
    /// let foods = client.saohua().eat().await?;
    /// println!("今天吃：{}", foods.join(" / "));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn eat(&self) -> Result<Vec<String>> {
        self.inner
            .get("/saohua/eat", &Vec::<(&str, String)>::new())
            .await
    }

    /// 随机骚话（免费）。
    ///
    /// 从骚话库随机取一条。
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
    /// println!("{}", client.saohua().random().await?.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn random(&self) -> Result<SaohuaRandom> {
        self.inner
            .get("/saohua/random", &Vec::<(&str, String)>::new())
            .await
    }

    /// 渣男语录（免费）。
    ///
    /// 从渣男语录库随机取一条。
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
    /// println!("{}", client.saohua().zhanan().await?.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn zhanan(&self) -> Result<SaohuaZhanan> {
        self.inner
            .get("/saohua/zhanan", &Vec::<(&str, String)>::new())
            .await
    }
}

impl Client {
    /// 骚话接口。
    pub fn saohua(&self) -> SaohuaApi {
        SaohuaApi::new(self.inner())
    }
}
