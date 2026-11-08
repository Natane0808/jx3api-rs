//! 秘境方位相关数据模型。
//!
//! 对应接口：`/mech/decrypt`。

use serde::{Deserialize, Serialize};

/// 秘境方位推算结果。
///
/// 对应接口：`GET /mech/decrypt`。服务端按 15 分钟档推算当前方位、
/// 下次变化时刻与倒计时。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MechDecrypt {
    /// 当前服务器时间，例如 `11:10:18`。
    #[serde(rename = "nowTime")]
    pub now_time: String,
    /// 当前档位的起始时刻，例如 `11:00:00`。
    #[serde(rename = "nowNode")]
    pub now_node: String,
    /// 当前档位的方位描述，例如 `北 4，南 1，西 1，东 9`。
    #[serde(rename = "nowResult")]
    pub now_result: String,
    /// 下一档位的起始时刻，例如 `11:15:00`。
    #[serde(rename = "nextNode")]
    pub next_node: String,
    /// 下一档位的方位描述。
    #[serde(rename = "nextResult")]
    pub next_result: String,
    /// 距离下一档位的倒计时，例如 `00:04:42`。
    #[serde(rename = "intervalTime")]
    pub interval_time: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_fixture() {
        let envelope: ApiResponse<MechDecrypt> =
            parse(include_str!("../../tests/fixtures/mech_decrypt.json"));
        assert!(envelope.is_success());
        let decrypt = envelope.data.expect("data 不应为空");
        assert_eq!(decrypt.now_time, "11:10:18");
        assert_eq!(decrypt.now_node, "11:00:00");
        assert!(decrypt.now_result.contains("北"));
        assert_eq!(decrypt.next_node, "11:15:00");
        assert!(!decrypt.next_result.is_empty());
        assert_eq!(decrypt.interval_time, "00:04:42");
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let decrypt: MechDecrypt = serde_json::from_str(r#"{"nowNode":"11:00:00"}"#).unwrap();
        assert_eq!(decrypt.now_node, "11:00:00");
        assert!(decrypt.now_result.is_empty());
        assert!(decrypt.interval_time.is_empty());
    }
}
