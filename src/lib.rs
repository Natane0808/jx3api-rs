//! # jx3-api
//!
//! [JX3API](https://www.jx3api.com)（剑网3 官方数据接口）的 Rust SDK。
//!
//! 覆盖官方 OpenAPI 规范中的全部 **82 个 HTTP 接口**，并提供可选的
//! WebSocket 实时事件推送客户端。
//!
//! 此外单独封装了快照之外的 **`POST /token/stats`**：通过 [`Client::quota`]
//! 查询令牌档位与剩余次数（该查询不消耗接口次数）。
//!
//! ## 快速开始
//!
//! ```no_run
//! use jx3_api::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 免费接口无需令牌；会员接口请填入自己的 token。
//!     let client = Client::builder()
//!         .token("jx3api::xxxxxxxx")
//!         .build()?;
//!
//!     // 活动日历
//!     let calendar = client.active().calendar(None).await?;
//!     println!("{} 大战：{}", calendar.date, calendar.war);
//!
//!     // 区服状态
//!     let status = client.server().status_check("长安城", None).await?;
//!     println!("{status:?}");
//!
//!     // 令牌剩余次数
//!     let quota = client.quota().stats().await?;
//!     println!("剩余 {} 次", quota.remaining);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 设计约定
//!
//! * **按业务域拆分**：每个业务域一个模块，通过 `client.<域>()` 获取句柄。
//! * **查询参数强类型**：参数超过一个的接口使用 `XxxQuery` 结构体，配合链式
//!   setter 与 `Default`，例如 `EventQuery::default().server("长安城")`。
//! * **模型容错**：所有模型字段缺失时回落到默认值，数字字段同时接受
//!   JSON 数字与数字字符串。
//! * **档位可见**：[`Endpoint`] 枚举保存每个接口的路径与所需档位（免费 / VIP1 / VIP2）。
//!
//! ## 功能开关
//!
//! * `websocket` —— 启用实时事件推送客户端 [`socket`]。
//!
//! ## 许可
//!
//! MIT

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod api;
pub mod client;
pub mod endpoint;
pub mod error;
pub mod models;
pub mod response;
pub mod serde_util;

#[cfg(feature = "websocket")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket")))]
pub mod socket;

pub use crate::client::{
    Client, ClientBuilder, DEFAULT_BASE_URL, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT,
};
pub use crate::endpoint::{Endpoint, Level};
pub use crate::error::{Error, Result};
pub use crate::response::ApiResponse;

/// 常用类型的一站式导入。
///
/// ```
/// use jx3_api::prelude::*;
/// ```
pub mod prelude {
    pub use crate::api;
    pub use crate::client::{Client, ClientBuilder};
    pub use crate::endpoint::{Endpoint, Level};
    pub use crate::error::{Error, Result};
    pub use crate::models;
    pub use crate::response::ApiResponse;
}
