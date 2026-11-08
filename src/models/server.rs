//! 服务器状态数据模型。
//!
//! 对应接口：`/server/status/check`。

use serde::{Deserialize, Serialize};

/// 单个服务器（大区）的状态。
///
/// 对应接口：`GET /server/status/check?server=长安城&type=1`。
///
/// `status` 官方既可能返回数字也可能返回字符串，这里统一宽松解析为字符串。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerCheck {
    /// 大区名称，例如 `电信区`。
    pub zone: String,
    /// 服务器名称，例如 `长安城`。
    pub server: String,
    /// 在线状态（`1` 在线 / `0` 离线），数字与字符串都接受。
    #[serde(deserialize_with = "crate::serde_util::flexible_string")]
    pub status: String,
    /// 最后一次探测到在线的时间戳（`type=1` 时有）。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub lasttime: i64,
    /// 服务器关闭时间戳（`type=1` 时有）。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub shuttime: i64,
}

/// `/server/status/check` 的返回结果。
///
/// 该接口的 `data` 类型随 `type` 参数变化（见官方文档）：
///
/// * `type=1`（默认）→ 数组；
/// * 其它 `type` → 单对象（指定区服）或数组（未指定区服）。
///
/// 因此用 `#[serde(untagged)]` 枚举表达「一条」或「多条」两种形态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCheckResult {
    /// 单条结果。
    One(ServerCheck),
    /// 多条结果。
    Many(Vec<ServerCheck>),
}

impl ServerCheckResult {
    /// 是否为多条结果。
    pub fn is_many(&self) -> bool {
        matches!(self, ServerCheckResult::Many(_))
    }

    /// 以切片形式查看结果，单条结果也会返回长度为 1 的切片。
    pub fn as_slice(&self) -> &[ServerCheck] {
        match self {
            ServerCheckResult::One(item) => std::slice::from_ref(item),
            ServerCheckResult::Many(items) => items,
        }
    }

    /// 把结果展开成 `Vec`，单条结果会包装成长度为 1 的向量。
    pub fn into_vec(self) -> Vec<ServerCheck> {
        match self {
            ServerCheckResult::One(item) => vec![item],
            ServerCheckResult::Many(items) => items,
        }
    }
}

impl From<ServerCheck> for ServerCheckResult {
    fn from(value: ServerCheck) -> Self {
        ServerCheckResult::One(value)
    }
}

impl From<Vec<ServerCheck>> for ServerCheckResult {
    fn from(value: Vec<ServerCheck>) -> Self {
        ServerCheckResult::Many(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_fixture_as_many() {
        let envelope: ApiResponse<ServerCheckResult> = parse(include_str!(
            "../../tests/fixtures/server_status_check.json"
        ));
        assert!(envelope.is_success());
        let result = envelope.data.expect("data 不应为空");
        assert!(result.is_many());
        let items = result.as_slice();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].server, "长安城");
        assert_eq!(items[0].zone, "电信区");
        // 官方返回数字 1，宽松解析成字符串 "1"。
        assert_eq!(items[0].status, "1");
        assert_eq!(items[0].lasttime, 1793850059);
        assert_eq!(items[0].shuttime, 1793831879);
    }

    #[test]
    fn parses_single_object_as_one() {
        let raw = r#"{
            "code": 200,
            "msg": "success",
            "data": { "zone": "电信区", "server": "长安城", "status": "1" },
            "time": 1
        }"#;
        let envelope: ApiResponse<ServerCheckResult> = parse(raw);
        assert!(envelope.is_success());
        let result = envelope.data.expect("data 不应为空");
        assert!(!result.is_many());
        assert_eq!(result.as_slice()[0].server, "长安城");
        assert_eq!(result.clone().into_vec().len(), 1);
    }

    #[test]
    fn status_accepts_string_form() {
        let item: ServerCheck =
            serde_json::from_str(r#"{"server":"长安城","status":"在线"}"#).unwrap();
        assert_eq!(item.status, "在线");
        assert_eq!(item.lasttime, 0);
    }
}
