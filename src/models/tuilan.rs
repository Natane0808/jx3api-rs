//! 资历分布相关数据模型。
//!
//! 对应接口：`/tuilan/achievement`。
//!
//! 官方返回结构不固定的裸字典：包含角色名片、按 子类 / 详情 / 地图 / 副本 汇总的
//! 资历完成度（各桶 `total`、排序结果与 `totalScore`）。SDK 不做改写，直接以
//! [`serde_json::Value`] 承载，避免官方调整分桶结构就解析失败。

/// 资历分布。
///
/// 官方返回裸字典，因此这里给出 [`serde_json::Value`] 的类型别名，便于在函数签名
/// 与文档里表达「这是资历分布的原始 JSON」。
pub type TuilanAchievement = serde_json::Value;

#[cfg(test)]
mod tests {
    use crate::response::ApiResponse;

    use super::*;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_achievement_fixture() {
        // 夹具是错误响应（缺 name 参数），只断言包络可解析。
        let envelope: ApiResponse<TuilanAchievement> =
            parse(include_str!("../../tests/fixtures/tuilan_achievement.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn achievement_alias_is_raw_json() {
        let raw = r#"{"data":{"total":100,"totalScore":12345},"name":"漓心"}"#;
        let value: TuilanAchievement = serde_json::from_str(raw).unwrap();
        assert_eq!(value["data"]["total"], serde_json::json!(100));
        assert_eq!(value["name"], serde_json::json!("漓心"));
    }
}
