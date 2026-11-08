//! 统一响应包络。

use serde::Serialize;
use serde::de::{Deserialize, Deserializer};

/// JX3API 的统一响应结构。
///
/// 服务端所有 HTTP 接口都返回该结构：
///
/// ```json
/// { "code": 200, "msg": "success", "data": { "...": "..." }, "time": 1794106931 }
/// ```
///
/// SDK 的业务方法默认直接返回 `data`，需要完整包络时使用
/// [`Client::request`](crate::Client::request)。
///
/// 反序列化由手写实现完成，因此 `T` 只需实现 [`Deserialize`]，**不需要**
/// `Default`，便于在测试里用任意模型解析夹具。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ApiResponse<T> {
    /// 业务状态码，`200` 表示成功。
    pub code: i32,
    /// 服务端描述信息，成功时为 `"success"`。
    pub msg: String,
    /// 业务数据，可能为 `null`。
    #[serde(default)]
    pub data: Option<T>,
    /// 服务端 Unix 时间戳（秒）。
    #[serde(default)]
    pub time: i64,
}

impl<'de, T> Deserialize<'de> for ApiResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// 与 [`ApiResponse`] 字段一致的中间结构体。
        ///
        /// 分开定义是为了避免 `#[derive(Deserialize)]` 给 `T` 强加 `Default` 约束。
        #[derive(serde::Deserialize)]
        #[serde(bound(deserialize = "T: Deserialize<'de>"))]
        struct Helper<T> {
            #[serde(default)]
            code: i32,
            #[serde(default)]
            msg: String,
            #[serde(default)]
            data: Option<T>,
            #[serde(default)]
            time: i64,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(ApiResponse {
            code: helper.code,
            msg: helper.msg,
            data: helper.data,
            time: helper.time,
        })
    }
}

impl<T> ApiResponse<T> {
    /// 是否为成功响应。
    pub fn is_success(&self) -> bool {
        self.code == 200
    }

    /// 取出 `data`，不存在时返回 [`Error::EmptyData`](crate::Error::EmptyData)。
    pub fn into_data(self, path: &str) -> crate::Result<T> {
        self.data.ok_or_else(|| crate::Error::EmptyData {
            path: path.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_envelope() {
        let raw = r#"{"code":200,"msg":"success","data":{"date":"2026-11-08"},"time":1794106931}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(raw).unwrap();
        assert!(resp.is_success());
        assert_eq!(resp.time, 1794106931);
        assert_eq!(resp.data.unwrap()["date"], "2026-11-08");
    }

    #[test]
    fn data_may_be_missing() {
        let raw = r#"{"code":404,"msg":"未收录","data":{},"time":1}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(raw).unwrap();
        assert!(!resp.is_success());
        assert_eq!(resp.msg, "未收录");
    }

    /// `T` 未实现 `Default` 时也能反序列化（手写实现的关键收益）。
    #[test]
    fn data_type_does_not_need_default() {
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct NoDefault {
            value: String,
        }

        let raw = r#"{"code":200,"msg":"success","data":{"value":"ok"},"time":1}"#;
        let resp: ApiResponse<NoDefault> = serde_json::from_str(raw).unwrap();
        assert_eq!(resp.data, Some(NoDefault { value: "ok".into() }));
    }
}
