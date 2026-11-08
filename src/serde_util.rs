//! 宽松的反序列化辅助函数。
//!
//! JX3API 的部分字段会在「数字」与「数字字符串」之间摇摆（例如家园装饰接口把
//! `type`、`color`、`quality` 全部序列化成字符串）。这里的辅助函数允许一个字段
//! 同时接受两种形态，从而避免因为一次线上改动就整个响应解析失败。

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 宽松解析整数：接受 JSON 数字、数字字符串、`null`（按 `0` 处理）。
///
/// ```rust
/// # use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(deserialize_with = "jx3_api::serde_util::flexible_i64")]
///     pub count: i64,
/// }
/// ```
pub fn flexible_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(FlexibleI64Visitor)
}

/// 宽松解析浮点数：接受 JSON 数字、数字字符串、`null`（按 `0.0` 处理）。
pub fn flexible_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(FlexibleF64Visitor)
}

/// 宽松解析字符串：接受 JSON 字符串、数字、布尔值、`null`（按空串处理）。
pub fn flexible_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(FlexibleStringVisitor)
}

/// 宽松解析布尔：接受 JSON 布尔、数字（`0` 为假）、字符串（`"true"` / `"false"` / `"1"` / `"0"`）。
pub fn flexible_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(FlexibleBoolVisitor)
}

struct FlexibleI64Visitor;

impl<'de> Visitor<'de> for FlexibleI64Visitor {
    type Value = i64;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("整数或整数字符串")
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(i64::try_from(v).unwrap_or(i64::MAX))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v as i64)
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(i64::from(v))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(0);
        }
        trimmed
            .parse::<i64>()
            .or_else(|_| trimmed.parse::<f64>().map(|f| f as i64))
            .map_err(|_| E::custom(format!("无法把 `{v}` 解析为整数")))
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(0)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(0)
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

struct FlexibleF64Visitor;

impl<'de> Visitor<'de> for FlexibleF64Visitor {
    type Value = f64;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("浮点数或浮点数字符串")
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v as f64)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v as f64)
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(if v { 1.0 } else { 0.0 })
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(0.0);
        }
        trimmed
            .parse::<f64>()
            .map_err(|_| E::custom(format!("无法把 `{v}` 解析为浮点数")))
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(0.0)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(0.0)
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

struct FlexibleStringVisitor;

impl<'de> Visitor<'de> for FlexibleStringVisitor {
    type Value = String;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("字符串")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.to_owned())
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(String::new())
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(String::new())
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

struct FlexibleBoolVisitor;

impl<'de> Visitor<'de> for FlexibleBoolVisitor {
    type Value = bool;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("布尔值")
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v != 0)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v != 0)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v.trim().to_ascii_lowercase().as_str() {
            "" | "0" | "false" | "null" | "no" => Ok(false),
            _ => Ok(true),
        }
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(false)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(false)
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

/// 把 JSON `null` 当作类型的默认值（[`Default::default`]）。
///
/// 用于「官方规范标注必有、但线上偶尔返回 `null`」的嵌套对象字段。配合
/// `#[serde(default)]` 使用时：
///
/// * 字段**缺失** → 走 `Default`；
/// * 字段为 `null` → 也走 `Default`（而不是整个响应解析失败）；
/// * 字段是对象 → 正常反序列化。
///
/// ```rust
/// # use serde::Deserialize;
/// #[derive(Deserialize, Default)]
/// # struct Detail { value: i64 }
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(default, deserialize_with = "jx3_api::serde_util::null_as_default")]
///     pub detail: Detail,
/// }
///
/// let missing: Foo = serde_json::from_str(r#"{}"#).unwrap();
/// let null: Foo = serde_json::from_str(r#"{"detail":null}"#).unwrap();
/// let object: Foo = serde_json::from_str(r#"{"detail":{"value":7}}"#).unwrap();
/// assert_eq!(missing.detail.value, 0);
/// assert_eq!(null.detail.value, 0);
/// assert_eq!(object.detail.value, 7);
/// ```
pub fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(deserializer).map(Option::unwrap_or_default)
}

/// 数字 / 字符串双形态的字段类型。
///
/// 序列化为 JSON 时始终输出数字；反序列化时接受 JSON 数字、数字字符串与
/// `null`（按 `0` 处理），与 [`flexible_i64`] 的宽松规则一致。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize)]
#[serde(transparent)]
pub struct FlexNumber(pub i64);

impl<'de> Deserialize<'de> for FlexNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        flexible_i64(deserializer).map(FlexNumber)
    }
}

impl From<i64> for FlexNumber {
    fn from(value: i64) -> Self {
        FlexNumber(value)
    }
}

impl From<FlexNumber> for i64 {
    fn from(value: FlexNumber) -> Self {
        value.0
    }
}

impl std::fmt::Display for FlexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Sample {
        #[serde(deserialize_with = "flexible_i64")]
        count: i64,
        #[serde(deserialize_with = "flexible_f64")]
        price: f64,
        #[serde(deserialize_with = "flexible_string")]
        label: String,
        #[serde(deserialize_with = "flexible_bool")]
        active: bool,
    }

    #[test]
    fn accepts_number_and_string_forms() {
        let raw = r#"{"count":"42","price":1.5,"label":7,"active":"true"}"#;
        let s: Sample = serde_json::from_str(raw).unwrap();
        assert_eq!(s.count, 42);
        assert_eq!(s.price, 1.5);
        assert_eq!(s.label, "7");
        assert!(s.active);
    }

    #[test]
    fn accepts_null_as_zero() {
        let raw = r#"{"count":null,"price":null,"label":null,"active":null}"#;
        let s: Sample = serde_json::from_str(raw).unwrap();
        assert_eq!(s.count, 0);
        assert_eq!(s.price, 0.0);
        assert_eq!(s.label, "");
        assert!(!s.active);
    }

    #[test]
    fn flex_number_accepts_number_and_numeric_string() {
        assert_eq!(
            serde_json::from_str::<FlexNumber>("42").unwrap(),
            FlexNumber(42)
        );
        assert_eq!(
            serde_json::from_str::<FlexNumber>("\"42\"").unwrap(),
            FlexNumber(42)
        );
        assert_eq!(
            serde_json::from_str::<FlexNumber>("null").unwrap(),
            FlexNumber(0)
        );
        assert_eq!(serde_json::to_string(&FlexNumber(42)).unwrap(), "42");
    }

    #[test]
    fn null_as_default_handles_missing_null_and_object() {
        #[derive(Debug, Default, PartialEq, Deserialize)]
        struct Detail {
            value: i64,
        }

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            #[serde(default, deserialize_with = "null_as_default")]
            detail: Detail,
        }

        let missing: Wrapper = serde_json::from_str(r#"{}"#).unwrap();
        let null: Wrapper = serde_json::from_str(r#"{"detail":null}"#).unwrap();
        let object: Wrapper = serde_json::from_str(r#"{"detail":{"value":7}}"#).unwrap();

        assert_eq!(missing.detail, Detail::default());
        assert_eq!(null.detail, Detail::default());
        assert_eq!(object.detail.value, 7);
    }
}
