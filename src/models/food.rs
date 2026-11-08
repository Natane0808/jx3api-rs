//! 小吃小药数据模型。
//!
//! 对应接口：`/food/list`。

use serde::{Deserialize, Serialize};

/// 单个门派心法的小吃小药条目。
///
/// 对应接口：`GET /food/list?name=花间游`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchoolFood {
    /// 门派名称，例如 `万花`。
    pub school: String,
    /// 心法名称，例如 `花间游`。
    pub kungfu: String,
    /// 品质颜色，例如 `紫`、`蓝`。
    pub color: String,
    /// 分类，例如 `增强食品`、`辅助药品`。
    pub r#class: String,
    /// 物品名称，例如 `云州刀削面·精`。
    pub name: String,
    /// 增益属性，例如 `内功`、`元气`。
    pub boost: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_list_fixture() {
        let envelope: ApiResponse<Vec<SchoolFood>> =
            parse(include_str!("../../tests/fixtures/food_list.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 184);
        let first = &items[0];
        assert_eq!(first.school, "万花");
        assert_eq!(first.kungfu, "花间游");
        assert_eq!(first.r#class, "增强食品");
        assert!(!first.name.is_empty());
        assert_eq!(first.boost, "内功");
    }

    #[test]
    fn missing_fields_fall_back_to_default() {
        let food: SchoolFood = serde_json::from_str(r#"{"name":"羊肉烩面"}"#).unwrap();
        assert_eq!(food.name, "羊肉烩面");
        assert!(food.school.is_empty());
        assert!(food.r#class.is_empty());
    }
}
