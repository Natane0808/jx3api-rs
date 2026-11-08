//! 交易行相关数据模型。
//!
//! 对应接口：`/trade/demon`、`/trade/item/records`、`/trade/item/search`、
//! `/trade/records`、`/trade/manufacture`、`/trade/wanbaolou`。
//!
//! `/trade/wanbaolou` 官方返回结构不固定的字典，API 层直接返回
//! [`serde_json::Value`]，本模块不为其建模。

use serde::{Deserialize, Serialize};

/// 外观条目。
///
/// 对应接口：`GET /trade/item/search`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeItem {
    /// 外观分类。
    pub category: String,
    /// 外观名称。
    pub name: String,
    /// 外观别名。
    pub alias: String,
    /// 官方定价。
    pub retail: i64,
    /// 描述。
    pub desc: String,
    /// 收录日期，例如 `2026-11-08`。
    pub date: String,
    /// 展示视角 / 视图（官方原样返回）。
    pub view: String,
}

/// 外观成交记录（`/trade/records` 与 `/trade/item/records` 共用）。
///
/// `/trade/records` 返回各区历史成交价（`search.sales`，各最近 10 条）；
/// `/trade/item/records` 在其基础上追加万宝楼两期挂单，因此两者字段一致，
/// 这里共用一个模型，`list` 中的每条记录既可能是成交记录也可能是挂单
/// （结构不固定，用 [`serde_json::Value`] 承载）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeRecords {
    /// 外观分类。
    pub category: String,
    /// 外观名称。
    pub name: String,
    /// 外观别名。
    pub alias: String,
    /// 官方定价。
    pub retail: i64,
    /// 描述。
    pub desc: String,
    /// 收录日期。
    pub date: String,
    /// 展示视角 / 视图。
    pub view: String,
    /// 按服务器分组的成交 / 挂单记录。
    pub list: Vec<TradeRecordGroup>,
}

/// 按服务器分组的记录。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeRecordGroup {
    /// 分组名称，通常是服务器名。
    pub name: String,
    /// 该服务器下的成交记录或万宝楼挂单（结构不固定，保留原始 JSON）。
    pub list: Vec<serde_json::Value>,
}

/// 金价行情。
///
/// 对应接口：`GET /trade/demon`。各渠道价格官方以字符串返回（例如 `800.00`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeDemon {
    /// 大区名称。
    pub zone: String,
    /// 服务器名称。
    pub server: String,
    /// 贴吧渠道金价。
    pub tieba: String,
    /// 万宝楼渠道金价。
    pub wanbaolou: String,
    /// dd373 渠道金价。
    pub dd373: String,
    /// uu898 渠道金价。
    pub uu898: String,
    /// 5173 渠道金价（官方字段名以数字开头，Rust 侧映射为 `site_5173`）。
    #[serde(rename = "5173")]
    pub site_5173: String,
    /// 7881 渠道金价（官方字段名以数字开头，Rust 侧映射为 `site_7881`）。
    #[serde(rename = "7881")]
    pub site_7881: String,
    /// 采集日期，例如 `2026-11-08`。
    pub date: String,
}

/// 制造成本计算结果。
///
/// 对应接口：`GET /trade/manufacture`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeManufacture {
    /// 成品名称。
    pub name: String,
    /// 成品等级。
    pub level: i64,
    /// 制造经验。
    pub exp: i64,
    /// 消耗精力。
    #[serde(rename = "costVigor")]
    pub cost_vigor: i64,
    /// 品质。
    pub quality: i64,
    /// 图标编号。
    #[serde(rename = "iconId")]
    pub icon_id: i64,
    /// 单次产出下限。
    #[serde(rename = "createItemMin")]
    pub create_item_min: i64,
    /// 单次产出上限。
    #[serde(rename = "createItemMax")]
    pub create_item_max: i64,
    /// 产出物品编号。
    #[serde(rename = "createItemIndex")]
    pub create_item_index: i64,
    /// 提示信息。
    #[serde(rename = "tipInfo")]
    pub tip_info: Vec<TradeTip>,
    /// 成品实时价格。
    pub price: TradePrice,
    /// 成本合计。
    pub total: TradeManufactureTotal,
    /// 材料列表。
    pub items: Vec<TradeMaterial>,
}

/// 提示信息。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeTip {
    /// 文本内容。
    pub text: String,
    /// 字号。
    pub font: i64,
    /// 颜色。
    pub color: String,
}

/// 价格统计。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradePrice {
    /// 最低价。
    #[serde(rename = "lowestPrice")]
    pub lowest_price: i64,
    /// 平均价。
    #[serde(rename = "avgPrice")]
    pub avg_price: i64,
    /// 最高价。
    #[serde(rename = "highestPrice")]
    pub highest_price: i64,
    /// 样本量。
    #[serde(rename = "sampleSize")]
    pub sample_size: i64,
    /// 更新时间戳（Unix 秒）。
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

/// 成本合计。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeManufactureTotal {
    /// 商店材料固定价合计。
    pub subtotal: i64,
    /// 最低成本合计。
    #[serde(rename = "lowestTotal")]
    pub lowest_total: i64,
    /// 平均成本合计。
    #[serde(rename = "avgTotal")]
    pub avg_total: i64,
    /// 最高成本合计。
    #[serde(rename = "highestTotal")]
    pub highest_total: i64,
}

/// 制造材料。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeMaterial {
    /// 材料来源类型（对应官方字段 `type`），例如 `商店` / `交易行`。
    #[serde(rename = "type")]
    pub kind: String,
    /// 材料名称。
    pub name: String,
    /// 品质。
    pub quality: i64,
    /// 材料分类编号，可能为 `null`。
    pub genre: Option<i64>,
    /// 图标编号。
    #[serde(rename = "iconId")]
    pub icon_id: i64,
    /// 需求数量。
    pub quantity: i64,
    /// 单价：商店材料是数字，交易行材料是价格对象（保留原始 JSON）。
    pub price: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_demon_fixture() {
        let envelope: ApiResponse<Vec<TradeDemon>> =
            parse(include_str!("../../tests/fixtures/trade_demon.json"));
        assert!(envelope.is_success());
        let list = envelope.data.expect("data 不应为空");
        assert!(!list.is_empty());
        let first = &list[0];
        assert_eq!(first.server, "长安城");
        assert_eq!(first.tieba, "800.00");
        assert_eq!(first.site_5173, "000.00");
        assert_eq!(first.site_7881, "917.43");
        assert_eq!(first.date, "2026-11-08");
    }

    #[test]
    fn parses_records_fixtures() {
        // 两个夹具都是错误响应（`/trade/records` 参数不正确、`/trade/item/records` 未找到外观），
        // 共用模型只要保证包络可解析。
        let envelope: ApiResponse<TradeRecords> =
            parse(include_str!("../../tests/fixtures/trade_records.json"));
        assert_eq!(envelope.code, 400);
        let envelope: ApiResponse<TradeRecords> =
            parse(include_str!("../../tests/fixtures/trade_item_records.json"));
        assert_eq!(envelope.code, 404);
    }

    #[test]
    fn parses_item_search_fixture() {
        // 错误响应的 `data` 是 `{}`（不是数组），按原始 JSON 断言包络。
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/trade_item_search.json"));
        assert_eq!(envelope.code, 404);
    }

    #[test]
    fn parses_manufacture_fixture() {
        let envelope: ApiResponse<TradeManufacture> =
            parse(include_str!("../../tests/fixtures/trade_manufacture.json"));
        assert_eq!(envelope.code, 400);
    }

    #[test]
    fn parses_wanbaolou_fixture() {
        let envelope: ApiResponse<serde_json::Value> =
            parse(include_str!("../../tests/fixtures/trade_wanbaolou.json"));
        assert_eq!(envelope.code, 404);
    }

    #[test]
    fn records_model_accepts_both_group_shapes() {
        let raw = r#"{
            "category": "外观",
            "name": "同人白发",
            "alias": "trbf",
            "retail": 100,
            "desc": "描述",
            "date": "2026-11-08",
            "view": "正面",
            "list": [{
                "name": "长安城",
                "list": [
                    {"date": "2026-11-01", "sale": 1, "server": "长安城", "value": 100, "zone": "电信区"},
                    {"id": "84", "index": "1", "zone": "电信区", "server": "长安城", "value": 120, "sale": 0,
                     "remainingTime": 3600, "token": "tk", "date": "2026-11-02", "source": 1, "status": 1}
                ]
            }]
        }"#;
        let records: TradeRecords = serde_json::from_str(raw).unwrap();
        assert_eq!(records.name, "同人白发");
        assert_eq!(records.list.len(), 1);
        assert_eq!(records.list[0].name, "长安城");
        assert_eq!(records.list[0].list.len(), 2);
        assert_eq!(records.list[0].list[0]["value"], serde_json::json!(100));
        assert_eq!(
            records.list[0].list[1]["remainingTime"],
            serde_json::json!(3600)
        );
    }

    #[test]
    fn manufacture_model_parses_price_and_materials() {
        let raw = r##"{
            "name": "菜团子",
            "level": 90,
            "exp": 10,
            "costVigor": 12,
            "quality": 3,
            "iconId": 1234,
            "createItemMin": 1,
            "createItemMax": 2,
            "createItemIndex": 5678,
            "tipInfo": [{"text": "需要烹饪", "font": 18, "color": "#ffffff"}],
            "price": {"lowestPrice": 10, "avgPrice": 20, "highestPrice": 30, "sampleSize": 4, "updatedAt": 5},
            "total": {"subtotal": 11, "lowestTotal": 12, "avgTotal": 13, "highestTotal": 14},
            "items": [
                {"type": "商店", "name": "面粉", "quality": 1, "genre": null, "iconId": 2, "quantity": 3, "price": 10},
                {"type": "交易行", "name": "鸡蛋", "quality": 1, "genre": 5, "iconId": 3, "quantity": 1,
                 "price": {"lowestPrice": 5, "avgPrice": 6, "highestPrice": 7, "sampleSize": 8, "updatedAt": 9}}
            ]
        }"##;
        let manufacture: TradeManufacture = serde_json::from_str(raw).unwrap();
        assert_eq!(manufacture.name, "菜团子");
        assert_eq!(manufacture.cost_vigor, 12);
        assert_eq!(manufacture.tip_info[0].color, "#ffffff");
        assert_eq!(manufacture.price.avg_price, 20);
        assert_eq!(manufacture.total.highest_total, 14);
        assert_eq!(manufacture.items.len(), 2);
        assert!(manufacture.items[0].genre.is_none());
        assert_eq!(manufacture.items[0].price, serde_json::json!(10));
        assert_eq!(manufacture.items[1].kind, "交易行");
        assert_eq!(manufacture.items[1].genre, Some(5));
        assert_eq!(
            manufacture.items[1].price["highestPrice"],
            serde_json::json!(7)
        );
    }
}
