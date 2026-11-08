#!/usr/bin/env python3
"""根据 openapi/openapi.json 生成 src/endpoint.rs。

接口元数据（路径 / 等级 / 中文名）直接来自官方 OpenAPI 规范，避免手写漂移。

用法::

    python scripts/gen_endpoints.py
"""

from __future__ import annotations

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "openapi" / "openapi.json"
OUT = ROOT / "src" / "endpoint.rs"

HEADER = '''//! 接口元数据。
//!
//! 本文件由 `scripts/gen_endpoints.py` 依据官方 OpenAPI 规范自动生成，请勿手工修改。
//!
//! [`Endpoint`] 为每个 HTTP 接口提供了路径、所需档位与中文名，方便运行期自检、
//! 生成文档或做权限判断。

use serde::{Deserialize, Serialize};

/// 接口所需档位。
///
/// 与官方 OpenAPI 的 `x-level` 字段一致，也与站点令牌档位对应：
///
/// * [`Level::Free`] —— 免费接口，无需令牌；
/// * [`Level::Vip1`] —— 需要 VIP1 令牌；
/// * [`Level::Vip2`] —— 需要 VIP2 令牌。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// 免费接口。
    Free,
    /// 需要 VIP1 令牌。
    Vip1,
    /// 需要 VIP2 令牌。
    Vip2,
}

impl Level {
    /// 档位的中文描述。
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Free => "免费",
            Level::Vip1 => "VIP1",
            Level::Vip2 => "VIP2",
        }
    }

    /// 档位数值，与 OpenAPI 的 `x-level` 对应。
    pub fn as_u8(self) -> u8 {
        match self {
            Level::Free => 0,
            Level::Vip1 => 1,
            Level::Vip2 => 2,
        }
    }

    /// 是否为免费接口。
    pub fn is_free(self) -> bool {
        matches!(self, Level::Free)
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
'''


def variant_name(path: str) -> str:
    parts = [p for p in path.strip("/").split("/") if p]
    name = "".join(p[:1].upper() + p[1:] for p in parts)
    name = re.sub(r"[^0-9A-Za-z_]", "", name)
    if name and name[0].isdigit():
        name = "E" + name
    return name


def main() -> int:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))
    rows = []
    for path, item in spec["paths"].items():
        op = item.get("get")
        if not op:
            continue
        rows.append((variant_name(path), path, int(op.get("x-level", 0)), op.get("summary", "")))

    rows.sort()
    body = [HEADER, "\n/// 单个 HTTP 接口的元数据。\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum Endpoint {"]
    for variant, path, level, summary in rows:
        level_name = {0: "Free", 1: "Vip1", 2: "Vip2"}[level]
        body.append(f"    /// {summary}（`{path}`）")
        body.append(f"    {variant},")
    body.append("}\n")

    body.append("impl Endpoint {")
    body.append("    /// 全部接口。")
    body.append("    pub const ALL: &'static [Endpoint] = &[")
    for variant, _, _, _ in rows:
        body.append(f"        Endpoint::{variant},")
    body.append("    ];\n")

    body.append("    /// 接口路径，形如 `/active/calendar`。")
    body.append("    pub fn path(self) -> &'static str {")
    body.append("        match self {")
    for variant, path, _, _ in rows:
        body.append(f'            Endpoint::{variant} => "{path}",')
    body.append("        }\n    }\n")

    body.append("    /// 所需档位。")
    body.append("    pub fn level(self) -> Level {")
    body.append("        match self {")
    for variant, _, level, _ in rows:
        level_name = {0: "Free", 1: "Vip1", 2: "Vip2"}[level]
        body.append(f"            Endpoint::{variant} => Level::{level_name},")
    body.append("        }\n    }\n")

    body.append("    /// 接口中文名。")
    body.append("    pub fn summary(self) -> &'static str {")
    body.append("        match self {")
    for variant, _, _, summary in rows:
        summary = summary.replace('"', '\\"')
        body.append(f'            Endpoint::{variant} => "{summary}",')
    body.append("        }\n    }\n")

    body.append("    /// 按路径查找接口。")
    body.append("    pub fn from_path(path: &str) -> Option<Endpoint> {")
    body.append("        let path = path.trim_start_matches('/');")
    body.append("        Endpoint::ALL")
    body.append("            .iter()")
    body.append("            .copied()")
    body.append("            .find(|endpoint| endpoint.path().trim_start_matches('/') == path)")
    body.append("    }\n")

    body.append("    /// 是否为免费接口。")
    body.append("    pub fn is_free(self) -> bool {")
    body.append("        self.level().is_free()")
    body.append("    }")
    body.append("}\n")

    body.append("""#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_paths_are_unique() {
        assert!(!Endpoint::ALL.is_empty());
        let unique = Endpoint::ALL
            .iter()
            .map(|endpoint| endpoint.path())
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(
            Endpoint::ALL.len(),
            unique.len(),
            "接口路径不应重复"
        );
    }

    #[test]
    fn lookup_by_path() {
        let endpoint = Endpoint::from_path("/active/calendar").expect("应存在该接口");
        assert_eq!(endpoint.path(), "/active/calendar");
        assert!(endpoint.is_free());
    }
}
""")

    OUT.write_text("\n".join(body), encoding="utf-8")
    print(f"已生成 {OUT}（{len(rows)} 个接口）")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
