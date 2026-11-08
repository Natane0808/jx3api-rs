//! 骗子查询相关数据模型。
//!
//! 对应接口：`/fraud/detail`。

use serde::{Deserialize, Serialize};

/// 骗子记录（贴吧风控条目）。
///
/// 对应接口：`GET /fraud/detail`。
///
/// 注意：该接口依赖远程风控服务，当前夹具是 `500` 错误响应，
/// 因此单元测试只断言包络可解析。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FraudRecord {
    /// 贴吧主题 id。
    pub tid: i64,
    /// 发帖人。
    pub author: String,
    /// 是否已评论。
    pub commented: bool,
    /// 帖子内容。
    pub content: String,
    /// 创建时间戳（秒）。
    pub created: i64,
    /// 楼层 id。
    pub pid: i64,
    /// 记录状态。
    pub status: i64,
    /// 贴吧名称。
    pub tieba: String,
    /// 帖子标题。
    pub title: String,
    /// 角色 UID。
    pub uid: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_error_fixture() {
        // 夹具是远程风控服务未实现的错误响应，只断言能解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/fraud_detail.json"));
        assert!(!envelope.is_success());
        assert!(envelope.msg.contains("未实现"));
    }

    #[test]
    fn parses_success_payload() {
        let raw = r#"{"code":200,"msg":"success","data":[
            {"tid":123,"author":"张三","commented":true,"content":"跑单",
             "created":1794000000,"pid":456,"status":1,"tieba":"剑网3",
             "title":"避雷","uid":100001}
        ]}"#;
        let envelope: ApiResponse<Vec<FraudRecord>> = parse(raw);
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].uid, 100001);
        assert_eq!(items[0].author, "张三");
        assert!(items[0].commented);
        assert_eq!(items[0].tieba, "剑网3");
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let record: FraudRecord = serde_json::from_str(r#"{"uid":1}"#).unwrap();
        assert_eq!(record.uid, 1);
        assert!(record.author.is_empty());
        assert!(!record.commented);
    }
}
