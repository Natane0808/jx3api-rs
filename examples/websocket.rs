//! WebSocket 实时事件示例。
//!
//! 本示例连接 `wss://socket.nicemoe.cn`，订阅「奇遇触发」以及全部事件，
//! 按 Ctrl+C 退出并优雅关闭连接。
//!
//! 需要启用 `websocket` feature：
//!
//! ```text
//! cargo run --example websocket --features websocket
//! ```
//!
//! 未启用该 feature 时，示例仍可编译，只会打印一条提示。
//! 令牌只从环境变量 `JX3API_TOKEN` 读取；未设置时以匿名方式连接。
//! 该示例会访问真实网络，请勿在自动化测试中运行。

#[cfg(feature = "websocket")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use jx3_api::socket::{SocketAction, SocketClient, SocketEvent};

    let mut builder = SocketClient::builder();

    // 令牌可选：设置 JX3API_TOKEN 时会附加到连接地址的 query 中。
    if let Ok(token) = std::env::var("JX3API_TOKEN") {
        if !token.trim().is_empty() {
            builder = builder.token(token.trim());
        }
    }

    let client = builder
        .reconnect(true) // 断线自动重连（默认开启）
        .build()?;

    // 只订阅「奇遇触发」（action = 1001），拿到强类型 detail。
    client.on(SocketAction::EventTrigger, |event| {
        if let SocketEvent::EventTrigger(detail) = event {
            println!(
                "[奇遇触发] {} {}：{} 于 {} 触发",
                detail.zone, detail.server, detail.name, detail.event
            );
        }
    });

    // 订阅全部事件；`event.name()` 返回事件中文名，`event.action()` 返回枚举。
    client.on_any(|event| {
        println!("[{}] {:?}", event.name(), event.action());
    });

    println!("正在连接 {} ……（按 Ctrl+C 退出）", client.url());
    let handle = client.connect().await?;

    tokio::signal::ctrl_c().await?;
    println!("收到 Ctrl+C，正在关闭连接……");
    handle.close().await?;

    Ok(())
}

#[cfg(not(feature = "websocket"))]
fn main() {
    eprintln!("本示例需要启用 websocket feature：");
    eprintln!("  cargo run --example websocket --features websocket");
}
