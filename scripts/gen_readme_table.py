#!/usr/bin/env python3
"""从 ``openapi/openapi.json`` 生成 README 的完整接口表（markdown）。

README 中的表格被一对标记包裹：

.. code-block:: markdown

    <!-- BEGIN:ENDPOINT_TABLE -->
    ...
    <!-- END:ENDPOINT_TABLE -->

用法::

    # 打印到标准输出
    python scripts/gen_readme_table.py

    # 就地刷新 README.md 中的表格
    python scripts/gen_readme_table.py --write README.md

    # CI 校验：README 中的表格是否与 OpenAPI 规范一致（不一致返回 1）
    python scripts/gen_readme_table.py --check README.md

数据来源只有一个：官方 OpenAPI 规范 ``openapi/openapi.json``（82 条路径）。
表格按 ``x-tagGroups`` → ``tag`` → 路径前缀（业务域）分组，档位取自 ``x-level``。
"""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from collections import OrderedDict

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "openapi" / "openapi.json"
DEFAULT_README = ROOT / "README.md"

BEGIN_MARKER = "<!-- BEGIN:ENDPOINT_TABLE -->"
END_MARKER = "<!-- END:ENDPOINT_TABLE -->"

# x-level 数值 → 展示文案；与 src/endpoint.rs 的 Level 枚举一致。
LEVEL_NAMES = {0: "免费", 1: "VIP1", 2: "VIP2"}
LEVEL_ORDER = (0, 1, 2)


def load_spec(path: pathlib.Path = SPEC) -> dict:
    """读取 OpenAPI 规范。"""
    return json.loads(path.read_text(encoding="utf-8"))


def domain_of(path: str) -> str:
    """路径的第一段即业务域：``/active/calendar`` → ``active``。"""
    return path.strip("/").split("/")[0]


def level_of(operation: dict) -> int:
    """取接口档位，缺失时按免费处理。"""
    return int(operation.get("x-level", 0))


def cell(text: str) -> str:
    """清理表格单元格内容，避免管道符与换行破坏表格。"""
    return (text or "").replace("|", "\\|").replace("\n", " ").strip()


def collect_endpoints(spec: dict) -> list[dict]:
    """展开规范中的全部 GET 接口，按路径排序。"""
    endpoints: list[dict] = []
    for path in sorted(spec.get("paths") or {}):
        operation = (spec["paths"][path] or {}).get("get")
        if not operation:
            continue
        tags = operation.get("tags") or [""]
        endpoints.append(
            {
                "path": path,
                "domain": domain_of(path),
                "tag": tags[0],
                "level": level_of(operation),
                "summary": operation.get("summary", ""),
            }
        )
    return endpoints


def collect_groups(spec: dict, endpoints: list[dict]) -> "OrderedDict[str, list[dict]]":
    """按官方 ``x-tagGroups`` 分组业务域，返回 ``{分组名: [{domain, tag, endpoints}]}``。

    分组顺序遵循 ``x-tagGroups``；同组内业务域按 OpenAPI ``tags`` 顺序排列，
    未出现在任何分组里的 tag 归入末尾的「其它」分组。
    """
    tag_groups: dict[str, str] = {}
    group_order: list[str] = []
    for group in spec.get("x-tagGroups") or []:
        name = group.get("name", "")
        group_order.append(name)
        for tag in group.get("tags") or []:
            tag_groups[tag] = name

    tag_order = [
        tag.get("name", "") for tag in spec.get("tags") or [] if tag.get("name")
    ]
    tag_rank = {tag: index for index, tag in enumerate(tag_order)}

    # 业务域 → 该域全部接口；tag 取该域接口出现次数最多的 tag。
    domains: "OrderedDict[str, dict]" = OrderedDict()
    for endpoint in endpoints:
        entry = domains.setdefault(
            endpoint["domain"],
            {"domain": endpoint["domain"], "tag": endpoint["tag"], "endpoints": []},
        )
        entry["endpoints"].append(endpoint)

    groups: "OrderedDict[str, list[dict]]" = OrderedDict(
        (name, []) for name in group_order
    )
    groups.setdefault("其它", [])

    for entry in domains.values():
        group = tag_groups.get(entry["tag"], "其它")
        groups.setdefault(group, []).append(entry)
        groups[group].sort(key=lambda item: tag_rank.get(item["tag"], 10**9))
        entry["endpoints"].sort(key=lambda item: item["path"])

    return OrderedDict(
        (name, entries) for name, entries in groups.items() if entries
    )


def total_paths(spec: dict) -> int:
    """规范中的 GET 接口总数。"""
    return sum(1 for op in (spec.get("paths") or {}).values() if op.get("get"))


def render(spec: dict) -> str:
    """生成完整接口表的 markdown（不含包裹标记）。"""
    endpoints = collect_endpoints(spec)
    groups = collect_groups(spec, endpoints)

    level_counts = {level: 0 for level in LEVEL_ORDER}
    for endpoint in endpoints:
        level_counts[endpoint["level"]] = level_counts.get(endpoint["level"], 0) + 1
    domain_count = sum(len(entries) for entries in groups.values())

    lines: list[str] = []
    lines.append(
        "> 本表由 [`scripts/gen_readme_table.py`](scripts/gen_readme_table.py) 依据 "
        "[`openapi/openapi.json`](openapi/openapi.json) 自动生成，请勿手工修改。"
    )
    lines.append(">")
    lines.append(
        f"> 共 **{len(endpoints)}** 个 HTTP 接口"
        f"（免费 **{level_counts.get(0, 0)}** / VIP1 **{level_counts.get(1, 0)}** / "
        f"VIP2 **{level_counts.get(2, 0)}**），按 **{domain_count}** 个业务域分组。"
        "每个业务域通过 `Client` 上的同名访问器获取句柄，例如 `client.active()`；"
        "档位含义见上文「档位说明」。"
    )
    lines.append("")

    # 总览：分组 → 业务域 → 调用入口 → 各档位接口数。
    lines.append("| 分组 | 业务域 | 调用入口 | 接口数 | 免费 | VIP1 | VIP2 |")
    lines.append("| --- | --- | --- | ---: | ---: | ---: | ---: |")
    for group_name, entries in groups.items():
        for entry in entries:
            counts = {level: 0 for level in LEVEL_ORDER}
            for endpoint in entry["endpoints"]:
                counts[endpoint["level"]] = counts.get(endpoint["level"], 0) + 1
            lines.append(
                f"| {group_name} | `{entry['domain']}` | `client.{entry['domain']}()` | "
                f"{len(entry['endpoints'])} | {counts.get(0, 0)} | {counts.get(1, 0)} | "
                f"{counts.get(2, 0)} |"
            )
    lines.append("")

    # 明细：分组标题 → 业务域小节 → 路径/档位/说明。
    for group_name, entries in groups.items():
        lines.append(f"### {group_name}")
        lines.append("")
        for entry in entries:
            lines.append(
                f"#### `{entry['domain']}` · {entry['tag']} —— "
                f"调用入口 `client.{entry['domain']}()`"
            )
            lines.append("")
            lines.append("| 路径 | 档位 | 说明 |")
            lines.append("| --- | --- | --- |")
            for endpoint in entry["endpoints"]:
                level = LEVEL_NAMES.get(endpoint["level"], str(endpoint["level"]))
                lines.append(
                    f"| `{cell(endpoint['path'])}` | {level} | {cell(endpoint['summary'])} |"
                )
            lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def replace_block(readme: str, table: str) -> str:
    """把 README 中标记之间的内容替换为 ``table``。"""
    start = readme.find(BEGIN_MARKER)
    end = readme.find(END_MARKER)
    if start == -1 or end == -1 or end < start:
        raise SystemExit(
            f"README 中未找到 {BEGIN_MARKER} / {END_MARKER} 标记，无法刷新接口表。"
        )
    head = readme[: start + len(BEGIN_MARKER)]
    tail = readme[end:]
    return f"{head}\n\n{table}\n{tail}"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="生成 README 的完整接口表")
    parser.add_argument(
        "--write",
        nargs="?",
        const=str(DEFAULT_README),
        default=None,
        metavar="README",
        help="就地刷新 README 的接口表（默认 README.md）",
    )
    parser.add_argument(
        "--check",
        nargs="?",
        const=str(DEFAULT_README),
        default=None,
        metavar="README",
        help="校验 README 的接口表是否为最新（不一致返回 1）",
    )
    args = parser.parse_args(argv)

    spec = load_spec()
    table = render(spec)
    path_count = total_paths(spec)
    domain_count = sum(
        len(entries) for entries in collect_groups(spec, collect_endpoints(spec)).values()
    )

    if args.check is not None:
        readme_path = pathlib.Path(args.check)
        readme = readme_path.read_text(encoding="utf-8")
        expected = replace_block(readme, table)
        if readme != expected:
            print(f"[FAIL] {readme_path} 的接口表已过期，请运行：", file=sys.stderr)
            print(f"       python {pathlib.Path(__file__).as_posix()} --write", file=sys.stderr)
            return 1
        print(f"[OK] {readme_path} 的接口表与 OpenAPI 规范一致（{path_count} 个接口）")
        return 0

    if args.write is not None:
        readme_path = pathlib.Path(args.write)
        readme = readme_path.read_text(encoding="utf-8")
        readme_path.write_text(replace_block(readme, table), encoding="utf-8")
        print(
            f"已刷新 {readme_path}：{path_count} 个接口 / {domain_count} 个业务域",
            file=sys.stderr,
        )
        return 0

    sys.stdout.write(table)
    print(f"（{path_count} 个接口 / {domain_count} 个业务域）", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())