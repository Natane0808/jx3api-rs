//! 语音合成接口。
//!
//! 对应 OpenAPI 中的 `/sound/*` 路径。

use std::sync::Arc;

use serde::Serialize;

use crate::client::{Client, Inner};
use crate::error::Result;
use crate::models::sound::SoundConverter;

/// 语音合成查询参数。
///
/// 对应接口：`GET /sound/converter`。
///
/// `appkey` / `access` / `secret` / `text` 为必填项，其余字段留空时使用服务端默认值。
///
/// # 示例
///
/// ```no_run
/// # use jx3_api::api::sound::SoundConverterQuery;
/// # use jx3_api::Client;
/// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new()?;
/// let query = SoundConverterQuery::default()
///     .appkey("your-appkey")
///     .access("your-access-key-id")
///     .secret("your-access-key-secret")
///     .text("你好，剑网3");
/// let audio = client.sound().converter(&query).await?;
/// println!("{}", audio.url);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SoundConverterQuery {
    /// 阿里云语音合成 appkey，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appkey: Option<String>,
    /// 阿里云 AccessKeyId，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// 阿里云 AccessKeySecret，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// 发音人，默认 `Aitong`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// 音频编码，默认 `mp3`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 音频采样率，默认 `16000`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// 音量，默认 `50`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    /// 语速，默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<String>,
    /// 语调，默认 `0`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_rate: Option<String>,
    /// 语音内容，**必填**。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SoundConverterQuery {
    /// 创建查询参数。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置阿里云 appkey（必填）。
    pub fn appkey(mut self, appkey: impl Into<String>) -> Self {
        self.appkey = Some(appkey.into());
        self
    }

    /// 设置阿里云 AccessKeyId（必填）。
    pub fn access(mut self, access: impl Into<String>) -> Self {
        self.access = Some(access.into());
        self
    }

    /// 设置阿里云 AccessKeySecret（必填）。
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// 设置发音人（默认 `Aitong`）。
    pub fn voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// 设置音频编码（默认 `mp3`）。
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置音频采样率（默认 `16000`）。
    pub fn sample_rate(mut self, sample_rate: i64) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// 设置音量（默认 `50`）。
    pub fn volume(mut self, volume: impl Into<String>) -> Self {
        self.volume = Some(volume.into());
        self
    }

    /// 设置语速（默认 `0`）。
    pub fn speech_rate(mut self, speech_rate: impl Into<String>) -> Self {
        self.speech_rate = Some(speech_rate.into());
        self
    }

    /// 设置语调（默认 `0`）。
    pub fn pitch_rate(mut self, pitch_rate: impl Into<String>) -> Self {
        self.pitch_rate = Some(pitch_rate.into());
        self
    }

    /// 设置语音内容（必填）。
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

/// 语音合成接口。
///
/// 通过 [`Client::sound`] 获取。
#[derive(Debug, Clone)]
pub struct SoundApi {
    inner: Arc<Inner>,
}

impl SoundApi {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    /// 语音合成（免费）。
    ///
    /// 用调用方提供的阿里云密钥签名换取语音服务 Token，并拼出 NLS TTS 流地址。
    ///
    /// # 参数
    ///
    /// * `query` —— 查询参数，`appkey` / `access` / `secret` / `text` 必填，
    ///   见 [`SoundConverterQuery`]。
    ///
    /// # Errors
    ///
    /// 网络异常、服务端返回错误码（例如阿里云密钥无效、缺少必填参数）。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::api::sound::SoundConverterQuery;
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let query = SoundConverterQuery::default()
    ///     .appkey("your-appkey")
    ///     .access("your-access-key-id")
    ///     .secret("your-access-key-secret")
    ///     .text("你好，剑网3");
    /// let audio = client.sound().converter(&query).await?;
    /// println!("{} → {}", audio.text, audio.url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn converter(&self, query: &SoundConverterQuery) -> Result<SoundConverter> {
        self.inner.get("/sound/converter", query).await
    }
}

impl Client {
    /// 语音合成接口。
    pub fn sound(&self) -> SoundApi {
        SoundApi::new(self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_required_fields_only() {
        let query = SoundConverterQuery::default()
            .appkey("app")
            .access("ak")
            .secret("sk")
            .text("你好");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(
            encoded,
            "appkey=app&access=ak&secret=sk&text=%E4%BD%A0%E5%A5%BD"
        );
        // token 由 Client 自动注入，不应出现在查询结构体里。
        assert!(!encoded.contains("token="));
    }

    #[test]
    fn serializes_optional_fields() {
        let query = SoundConverterQuery::default()
            .appkey("app")
            .access("ak")
            .secret("sk")
            .text("你好")
            .voice("Aitong")
            .format("mp3")
            .sample_rate(16000)
            .volume("50")
            .speech_rate("0")
            .pitch_rate("0");
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("voice=Aitong"));
        assert!(encoded.contains("format=mp3"));
        assert!(encoded.contains("sample_rate=16000"));
        assert!(encoded.contains("volume=50"));
        assert!(encoded.contains("speech_rate=0"));
        assert!(encoded.contains("pitch_rate=0"));
    }
}
