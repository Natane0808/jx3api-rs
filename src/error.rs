//! 错误类型定义。

/// 本 crate 统一使用的 `Result` 别名。
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// SDK 返回的错误。
///
/// 分为三类：
///
/// * [`Error::Http`] / [`Error::HttpStatus`] —— 网络层或 HTTP 状态码异常；
/// * [`Error::Decode`] / [`Error::Encode`] —— 序列化、反序列化失败；
/// * [`Error::Api`] —— 服务端正常返回但业务码不为 `200`（例如「未收录」「参数不正确」）。
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// HTTP 请求失败（连接、超时等）。
    #[error("HTTP 请求失败: {0}")]
    Http(#[from] reqwest::Error),

    /// HTTP 状态码不是 2xx，且响应体不是标准业务包络。
    #[error("HTTP 状态码异常 {status}: {body}")]
    HttpStatus {
        /// HTTP 状态码。
        status: u16,
        /// 原始响应体。
        body: String,
    },

    /// 响应体不是合法 JSON，或 `data` 字段与期望的类型不匹配。
    #[error("响应解析失败: {0}")]
    Decode(#[from] serde_json::Error),

    /// 查询参数序列化失败。
    #[error("查询参数序列化失败: {0}")]
    Encode(String),

    /// 服务端返回了业务错误码。
    #[error("接口 [{path}] 返回错误 [{code}]: {msg}")]
    Api {
        /// 业务错误码，非 200。
        code: i32,
        /// 服务端错误描述。
        msg: String,
        /// 出错的接口路径。
        path: String,
    },

    /// 接口返回了 `null` 数据，而调用方期望的是非空类型。
    #[error("接口 [{path}] 未返回数据")]
    EmptyData {
        /// 接口路径。
        path: String,
    },

    /// 调用前本地校验失败：该接口必须提供站点令牌，但 [`Client`](crate::Client)
    /// 上未配置，或显式传入的令牌为空。
    #[error("接口 [{path}] 需要站点令牌，请先通过 Client::builder().token(..) 配置")]
    MissingToken {
        /// 接口路径。
        path: String,
    },
}

impl Error {
    /// 取得业务错误码；网络层错误没有业务码时返回 `None`。
    pub fn code(&self) -> Option<i32> {
        match self {
            Error::Api { code, .. } => Some(*code),
            Error::HttpStatus { status, .. } => Some(i32::from(*status)),
            _ => None,
        }
    }

    /// 取得服务端返回的错误描述；非服务端错误时返回错误自身的描述。
    pub fn message(&self) -> String {
        match self {
            Error::Api { msg, .. } => msg.clone(),
            Error::HttpStatus { body, .. } => body.clone(),
            other => other.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_exposes_code_and_message() {
        let err = Error::Api {
            code: 400,
            msg: "参数不正确".into(),
            path: "/active/calendar".into(),
        };
        assert_eq!(err.code(), Some(400));
        assert_eq!(err.message(), "参数不正确");
        assert!(err.to_string().contains("/active/calendar"));
    }

    #[test]
    fn http_error_has_no_business_code() {
        assert_eq!(Error::Encode("boom".into()).code(), None);
    }
}
