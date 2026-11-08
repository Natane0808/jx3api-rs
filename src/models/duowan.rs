//! 统战歪歪数据模型。

use serde::{Deserialize, Serialize};

/// 某服的 YY 语音频道统计。
///
/// 对应接口：`GET /duowan/statistics`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DuowanStatistics {
    /// 服务器名称。
    pub server: String,
    /// 阵营频道列表。
    pub data: Vec<DuowanChannel>,
}

/// 单个 YY 频道。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DuowanChannel {
    /// 频道 sid。
    pub sid: i64,
    /// 在线人数。
    pub users: i64,
    /// 人数上限。
    pub limit: i64,
    /// 频道图片编号。
    pub logo: i64,
    /// 关联 sid。
    pub asid: i64,
    /// 频道图片地址。
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    /// 阵营名称。
    #[serde(rename = "campName")]
    pub camp_name: String,
    /// 频道昵称。
    pub snick: String,
    /// 扩展 sid，可能缺失。
    pub esid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    #[test]
    fn parses_fixture() {
        let envelope: ApiResponse<Vec<DuowanStatistics>> =
            serde_json::from_str(include_str!("../../tests/fixtures/duowan_statistics.json"))
                .expect("夹具应当能解析");
        assert!(envelope.is_success());
        let stats = envelope.data.expect("data 不应为空");
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].server, "长安城");
        let channels = &stats[0].data;
        assert_eq!(channels.len(), 2);

        let first = &channels[0];
        assert!(first.logo_url.starts_with("http"), "logoUrl 应能解析");
        assert_eq!(first.camp_name, "浩气盟");
        assert_eq!(first.esid, None);

        let second = &channels[1];
        assert!(second.logo_url.starts_with("http"));
        assert_eq!(second.camp_name, "恶人谷");
        assert_eq!(second.esid.as_deref(), Some("76007"));
    }
}
