//! WebSocket 客户端的错误类型。
//!
//! 与 HTTP 侧的 [`crate::error::Error`] 分开定义：本模块只在 `websocket`
//! feature 下编译，错误语义（连接、协议、关闭、超时）也与 HTTP 请求不同。

use std::time::Duration;

/// WebSocket 客户端统一使用的 `Result` 别名。
pub type SocketResult<T, E = SocketError> = std::result::Result<T, E>;

/// WebSocket 客户端错误。
///
/// 覆盖四类失败场景：
///
/// * [`SocketError::Connect`] / [`SocketError::Protocol`] —— 网络层与协议层异常；
/// * [`SocketError::Encode`] / [`SocketError::Decode`] —— 消息序列化、反序列化失败；
/// * [`SocketError::Closed`] —— 连接已关闭；
/// * [`SocketError::Timeout`] —— 操作超时。
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SocketError {
    /// 建立连接（WebSocket 握手）失败。
    #[error("WebSocket 连接失败: {0}")]
    Connect(String),

    /// 连接建立后的读写或协议错误（底层 I/O、TLS、帧解析等）。
    #[error("WebSocket 协议错误: {0}")]
    Protocol(String),

    /// 序列化待发送的消息失败。
    #[error("WebSocket 消息序列化失败: {0}")]
    Encode(String),

    /// 反序列化收到的消息失败。
    #[error("WebSocket 消息反序列化失败: {0}")]
    Decode(#[from] serde_json::Error),

    /// 连接已关闭，无法继续操作。
    #[error("WebSocket 连接已关闭: {0}")]
    Closed(String),

    /// 操作超时。
    #[error("WebSocket 操作超时（{0:?}）")]
    Timeout(Duration),

    /// 无法识别的 action 数值。
    #[error("无法识别的 WebSocket action: {0}")]
    UnknownAction(u32),

    /// action 字符串不是合法的十进制整数。
    #[error("无法解析 WebSocket action: {0}")]
    ActionParse(String),

    /// 地址不是合法的 `ws://` 或 `wss://` URL。
    #[error("无效的 WebSocket 地址: {0}")]
    InvalidUrl(String),
}

impl SocketError {
    /// 是否为「连接已关闭」类错误。
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed(_))
    }

    /// 是否为超时错误。
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_and_closed_are_recognised() {
        let timeout = SocketError::Timeout(Duration::from_secs(1));
        assert!(timeout.is_timeout());
        assert!(!timeout.is_closed());

        let closed = SocketError::Closed("已关闭".into());
        assert!(closed.is_closed());
        assert!(!closed.is_timeout());
    }

    #[test]
    fn decode_error_keeps_serde_source() {
        let err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        let err = SocketError::from(err);
        assert!(matches!(err, SocketError::Decode(_)));
        assert!(err.to_string().contains("反序列化"));
    }
}
