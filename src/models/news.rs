//! 新闻公告数据模型。
//!
//! 对应接口：`/news/announce`、`/news/records`，两个接口共用 [`NewsRecord`]。

use serde::{Deserialize, Serialize};

/// 一条新闻 / 公告。
///
/// 对应接口：`GET /news/announce?limit=10` 与 `GET /news/records?limit=10`。
///
/// 列表里是「栏目 + 日期 + 详情」的扁平结构，正文与属性都在 [`NewsDesc`] 中。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewsRecord {
    /// 条目编号（公告为公告 id，资讯为资讯 id）。
    pub catid: String,
    /// 发布日期，例如 `2026/11/05`。
    pub date: String,
    /// 标题。
    pub title: String,
    /// 新闻类型，例如 `官方公告`、`官方新闻`。
    pub r#type: String,
    /// 原文链接。
    pub url: String,
    /// 详情内容。
    pub desc: NewsDesc,
}

/// 新闻 / 公告的详情。
///
/// `/news/announce` 与 `/news/records` 的详情字段并不完全一致：
///
/// * 两者都有的通用字段：`title`、`content`、`asktime`、`top`、`inputtime`、`ext_data`；
/// * 仅资讯（`/news/records`）出现的字段统一建模为 `Option<String>`，
///   取不到时为 `None`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewsDesc {
    /// 详情标题。
    pub title: String,
    /// 正文 HTML。
    pub content: String,
    /// 提问 / 发布时间，例如 `2026-11-05 09:00:00`。
    pub asktime: String,
    /// 是否置顶，官方以字符串返回。
    pub top: String,
    /// 录入时间戳；公告返回数字、资讯返回数字或字符串，因此统一转成字符串。
    #[serde(deserialize_with = "crate::serde_util::flexible_string")]
    pub inputtime: String,
    /// 扩展数据，实际响应中为 `null`。
    pub ext_data: Option<serde_json::Value>,
    /// 资讯正文 id。
    pub id: Option<String>,
    /// 资讯栏目 id。
    pub catid: Option<String>,
    /// 自定义样式。
    pub style: Option<String>,
    /// 缩略图地址。
    pub thumb: Option<String>,
    /// SEO 关键词。
    pub keywords: Option<String>,
    /// SEO 摘要。
    pub description: Option<String>,
    /// 详情页地址。
    pub url: Option<String>,
    /// 排序值。
    pub listorder: Option<String>,
    /// 是否外链。
    pub islink: Option<String>,
    /// 更新时间戳。
    pub updatetime: Option<String>,
    /// 原站 id。
    pub oid: Option<String>,
    /// 直播作者。
    pub author_live: Option<String>,
    /// 直播作者头像。
    pub author_live_icon: Option<String>,
    /// 直播校验状态。
    pub check_is_live: Option<String>,
    /// 直播状态。
    pub live_status: Option<String>,
    /// App 推荐标记。
    pub app_is_recommended: Option<String>,
    /// App 允许评论标记。
    pub app_allow_comment: Option<String>,
    /// 直播作者名。
    pub live_author: Option<String>,
    /// 直播作者头像地址。
    pub live_author_icon: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::ApiResponse;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_announce_fixture() {
        let envelope: ApiResponse<Vec<NewsRecord>> =
            parse(include_str!("../../tests/fixtures/news_announce.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 20);
        let first = &items[0];
        assert_eq!(first.catid, "1335908");
        assert_eq!(first.date, "2026/11/05");
        assert_eq!(first.r#type, "官方公告");
        assert!(first.title.contains("版本更新公告"));
        assert!(!first.url.is_empty());
        assert!(!first.desc.title.is_empty());
        assert!(!first.desc.content.is_empty());
        assert!(!first.desc.inputtime.is_empty());
        assert!(first.desc.ext_data.is_none());
        // 公告没有资讯专属字段。
        assert!(first.desc.id.is_none());
    }

    #[test]
    fn parses_records_fixture() {
        let envelope: ApiResponse<Vec<NewsRecord>> =
            parse(include_str!("../../tests/fixtures/news_records.json"));
        assert!(envelope.is_success());
        let items = envelope.data.expect("data 不应为空");
        assert_eq!(items.len(), 20);
        let first = &items[0];
        assert_eq!(first.catid, "24587596");
        assert_eq!(first.r#type, "官方新闻");
        assert_eq!(first.desc.id.as_deref(), Some("7596"));
        assert_eq!(first.desc.catid.as_deref(), Some("2458"));
        assert!(
            first
                .desc
                .description
                .as_deref()
                .is_some_and(|text| !text.is_empty())
        );
        // 资讯同样带有正文与录入时间。
        assert!(!first.desc.content.is_empty());
        assert_eq!(first.desc.inputtime, "1794024000");
    }

    #[test]
    fn inputtime_accepts_number_and_string() {
        let numeric: NewsDesc = serde_json::from_str(r#"{"inputtime":1793959578}"#).unwrap();
        assert_eq!(numeric.inputtime, "1793959578");
        let textual: NewsDesc = serde_json::from_str(r#"{"inputtime":"1793959578"}"#).unwrap();
        assert_eq!(textual.inputtime, "1793959578");
    }
}
