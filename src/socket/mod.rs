//! WebSocket 实时事件推送客户端（需要 `websocket` feature）。
//!
//! 服务地址为 `wss://socket.nicemoe.cn`，服务端主动推送游戏世界事件。每条消息
//! 都是一个 JSON 包络：
//!
//! ```json
//! {
//!   "action": 1001,
//!   "status": "success",
//!   "detail": {
//!     "zone": "电信一区",
//!     "server": "长安城",
//!     "name": "叶英",
//!     "event": "黑白路",
//!     "time": 1794106931
//!   }
//! }
//! ```
//!
//! 本模块把官方规范中的 **39 个 action** 全部建模为 [`SocketAction`]，并把
//! `detail` 解析成 [`SocketEvent`] 的强类型变体。
//!
//! ## 快速开始
//!
//! ```no_run
//! use jx3_api::socket::{SocketAction, SocketClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SocketClient::builder()
//!         .token("jx3api::xxxxxxxx")
//!         .build()?;
//!
//!     client.on(SocketAction::EventTrigger, |event| {
//!         println!("奇遇触发: {event:?}");
//!     });
//!     client.on_any(|event| println!("{}: {:?}", event.name(), event));
//!
//!     let handle = client.connect().await?;
//!     // ……业务处理……
//!     handle.close().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## 模块结构
//!
//! * [`actions`] —— 每个 action 对应的 `detail` 结构体；
//! * [`events`] —— [`SocketAction`] / [`SocketEnvelope`] / [`SocketEvent`]；
//! * [`client`] —— [`SocketClient`] / [`SocketBuilder`] / [`SocketHandle`]；
//! * [`error`] —— [`SocketError`]。
//!
//! ## 重连与心跳
//!
//! * 默认开启自动重连：断线后按指数退避重试（1s、2s、4s……最多 60s），
//!   连续失败 [`DEFAULT_MAX_RECONNECTS`] 次后停止；可用
//!   [`SocketBuilder::reconnect`] / [`SocketBuilder::max_reconnects`] 调整。
//!   重连次数耗尽后客户端进入终止状态（[`SocketClient::is_closed`] 返回 `true`），
//!   [`SocketClient::run`] 会在后台循环结束后返回 `Ok(())`；如需继续使用请重新构建客户端。
//! * 默认每 30 秒发送一次 WebSocket `Ping` 帧；可用
//!   [`SocketBuilder::heartbeat`] 修改间隔，传入 `None` 关闭。
//! * 首次连接失败由 [`SocketClient::connect`] 直接返回错误；之后的重连失败
//!   在后台静默重试，不会产生额外回调。

pub mod actions;
pub mod client;
pub mod error;
pub mod events;

pub use actions::*;
pub use client::{
    DEFAULT_CLOSE_TIMEOUT, DEFAULT_HEARTBEAT_INTERVAL, DEFAULT_MAX_RECONNECT_BACKOFF,
    DEFAULT_MAX_RECONNECTS, DEFAULT_RECONNECT_BACKOFF, DEFAULT_SOCKET_URL, SocketBuilder,
    SocketClient, SocketHandle, SocketHandler,
};
pub use error::{SocketError, SocketResult};
pub use events::{SocketAction, SocketEnvelope, SocketEvent};
