//! 按业务域划分的接口集合。
//!
//! 每个模块对应一组 OpenAPI 路径，并通过 [`Client`](crate::Client) 上的同名方法获取：
//!
//! ```no_run
//! # use jx3_api::Client;
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new()?;
//! let events = client.active().celebs("楚天社").await?;
//! # Ok(())
//! # }
//! ```

pub mod active;
pub mod arena;
pub mod auction;
pub mod battle;
pub mod calcd;
pub mod card;
pub mod castle;
pub mod chat;
pub mod chitu;
pub mod duowan;
pub mod event;
pub mod exam;
pub mod firework;
pub mod food;
pub mod fraud;
pub mod home;
pub mod mech;
pub mod mentor;
pub mod monster;
pub mod news;
pub mod ranch;
pub mod rank;
pub mod recruit;
pub mod reward;
pub mod role;
pub mod sand;
pub mod saohua;
pub mod school;
pub mod server;
pub mod skill;
pub mod sound;
pub mod steed;
pub mod tieba;
pub mod token;
pub mod trade;
pub mod trial;
pub mod tuilan;
pub mod wicked;
