//! 语音合成数据模型。
//!
//! 对应接口：`/sound/converter`。

use serde::{Deserialize, Serialize};

/// 语音合成结果。
///
/// 对应接口：`GET /sound/converter`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundConverter {
    /// 合成内容。
    pub text: String,
    /// 语音服务令牌。
    pub token: String,
    /// 音频地址（NLS TTS 流地址）。
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_converter_fixture_envelope() {
        // 夹具是错误响应（code = 400，data = {}），只断言包络能解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/sound_converter.json"));
        assert!(!envelope.is_success());
        assert_eq!(envelope.code, 400);
        assert_eq!(envelope.msg, "参数不正确");
    }

    #[test]
    fn parses_converter_model() {
        let raw = r#"{
            "text": "你好，剑网3",
            "token": "nls-token",
            "url": "https://nls-gateway.cn-shanghai.aliyuncs.com/stream/v1/tts?token=nls-token"
        }"#;
        let audio: SoundConverter = serde_json::from_str(raw).unwrap();
        assert_eq!(audio.text, "你好，剑网3");
        assert_eq!(audio.token, "nls-token");
        assert!(audio.url.contains("tts"));
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let audio: SoundConverter = serde_json::from_str(r#"{"text":"hi"}"#).unwrap();
        assert_eq!(audio.text, "hi");
        assert!(audio.token.is_empty());
        assert!(audio.url.is_empty());
    }
}
