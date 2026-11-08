//! 家园数据模型。
//!
//! 对应接口：`/home/flower`、`/home/furniture`、`/home/travel`。

use serde::{Deserialize, Serialize};

/// 家园鲜花行情条目。
///
/// 对应接口：`GET /home/flower?server=长安城`，按地图归组后作为
/// `HashMap<String, Vec<HomeFlowerItem>>` 的值。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HomeFlowerItem {
    /// 鲜花名称，例如 `一级牵牛花`。
    pub name: String,
    /// 可染色颜色，以中文逗号分隔；部分作物（如 `芜菁`）为 `null`。
    pub color: Option<String>,
    /// 单价。
    #[serde(deserialize_with = "crate::serde_util::flexible_f64")]
    pub price: f64,
    /// 产出的家园线路编号。
    pub line: Vec<String>,
}

/// 家园装饰。
///
/// 对应接口：`GET /home/furniture?name=龙门香梦`。
///
/// 官方把 `type` / `color` / `limit` / `quality` / `view` 等数值字段序列化成
/// 字符串，这里统一用 [`flexible_i64`](crate::serde_util::flexible_i64) 宽松解析。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HomeFurniture {
    /// 装饰编号，官方可能省略。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub id: i64,
    /// 装饰名称，例如 `龙门香梦`。
    pub name: String,
    /// 装饰类型编号。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub r#type: i64,
    /// 偏色编号。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub color: i64,
    /// 获取来源，例如 `大水南方令`。
    pub source: String,
    /// 建筑度。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub architecture: i64,
    /// 摆放数量上限。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub limit: i64,
    /// 品质。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub quality: i64,
    /// 观赏值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub view: i64,
    /// 实用值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub practical: i64,
    /// 坚固值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub hard: i64,
    /// 风水值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub geomantic: i64,
    /// 趣味值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub interesting: i64,
    /// 产出地图，可能为 `null`。
    pub produce: Option<String>,
    /// 图标地址。
    pub image: String,
    /// 道具描述。
    pub tip: String,
}

/// 家园器物图谱条目。
///
/// 对应接口：`GET /home/travel?name=...`，字段与 [`HomeFurniture`] 一致。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HomeTravel {
    /// 器物编号，官方可能省略。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub id: i64,
    /// 器物名称。
    pub name: String,
    /// 器物类型编号。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub r#type: i64,
    /// 偏色编号。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub color: i64,
    /// 获取来源。
    pub source: String,
    /// 建筑度。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub architecture: i64,
    /// 摆放数量上限。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub limit: i64,
    /// 品质。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub quality: i64,
    /// 观赏值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub view: i64,
    /// 实用值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub practical: i64,
    /// 坚固值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub hard: i64,
    /// 风水值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub geomantic: i64,
    /// 趣味值。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub interesting: i64,
    /// 产出地图，可能为 `null`。
    pub produce: Option<String>,
    /// 图标地址。
    pub image: String,
    /// 道具描述。
    pub tip: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_flower_fixture() {
        let envelope: ApiResponse<HashMap<String, Vec<HomeFlowerItem>>> =
            parse(include_str!("../../tests/fixtures/home_flower.json"));
        assert!(envelope.is_success());
        let maps = envelope.data.expect("data 不应为空");
        assert_eq!(maps.len(), 5);
        let guangdong = maps.get("广陵邑").expect("应包含广陵邑地图");
        assert_eq!(guangdong.len(), 23);
        let first = &guangdong[0];
        assert_eq!(first.name, "一级牵牛花");
        assert_eq!(first.color.as_deref(), Some("红，粉，紫"));
        assert_eq!(first.price, 1.5);
        assert_eq!(first.line.len(), 3);
        // 部分作物的 color 为 null。
        assert!(guangdong.iter().any(|item| item.color.is_none()));
    }

    #[test]
    fn parses_furniture_fixture() {
        let envelope: ApiResponse<Vec<HomeFurniture>> =
            parse(include_str!("../../tests/fixtures/home_furniture.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 1);
        let item = &items[0];
        assert_eq!(item.name, "龙门香梦");
        // 官方返回的是字符串 "1"，宽松解析后应得到数字 1。
        assert_eq!(item.r#type, 1);
        assert_eq!(item.quality, 1000);
        assert_eq!(item.limit, 10);
        assert!(item.produce.is_none());
        assert!(!item.image.is_empty());
    }

    #[test]
    fn parses_travel_fixture_envelope() {
        // 夹具是错误响应（code = 400，data = {}），只断言包络能解析。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/home_travel.json"));
        assert!(!envelope.is_success());
        assert_eq!(envelope.code, 400);
        assert_eq!(envelope.msg, "参数不正确");
    }

    #[test]
    fn parses_travel_model_from_string_fields() {
        let raw = r#"{
            "id": 1,
            "name": "器物",
            "type": "2",
            "color": "3",
            "source": "大水南方令",
            "architecture": "0",
            "limit": "10",
            "quality": "1000",
            "view": "7931",
            "practical": "3525",
            "hard": "3525",
            "geomantic": "3525",
            "interesting": "3525",
            "produce": null,
            "image": "https://example.com/a.png",
            "tip": "说明"
        }"#;
        let item: HomeTravel = serde_json::from_str(raw).unwrap();
        assert_eq!(item.name, "器物");
        assert_eq!(item.r#type, 2);
        assert_eq!(item.view, 7931);
        assert_eq!(item.interesting, 3525);
        assert!(item.produce.is_none());
    }
}
