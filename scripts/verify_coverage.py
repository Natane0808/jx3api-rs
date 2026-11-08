#!/usr/bin/env python3
"""覆盖率自检：把官方 OpenAPI 规范与实际 Rust 实现做对账。

用法（仓库根目录执行）：

    python scripts/verify_coverage.py
    python scripts/verify_coverage.py --strict   # 把 WARN 项也计入失败

检查项：

1. `openapi/openapi.json` 中每一条 GET 路径，是否在 `src/api/*.rs` 里被引用
   （出现字面量 `"<path>"`）。
2. 每个业务域是否同时存在 `src/api/<域>.rs` 与 `src/models/<域>.rs`。
3. 是否还残留「待实现」桩。
4. 每个业务域是否都有 `#[cfg(test)] mod tests`，以及测试里引用的夹具文件是否存在。
5. `tests/fixtures/` 里是否每个夹具都被某个模型文件引用。
6. 每个业务域是否有**成功**夹具（`code == 200`）；没有成功夹具的域默认只 WARN，
   加 `--strict` 时计入失败。
7. 每个模型测试是否有字段级断言（出现 `assert_eq!`，或对字段的 `assert!`）；
   默认只 WARN，加 `--strict` 时计入失败。

退出码：0 = 全部通过（默认模式允许 WARN）；1 = 有失败项（打印明细）。
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
API = SRC / "api"
MODELS = SRC / "models"
FIXTURES = ROOT / "tests" / "fixtures"
OPENAPI = ROOT / "openapi" / "openapi.json"

DOMAINS = sorted(
    p.stem for p in API.glob("*.rs") if p.stem not in {"mod"}
)


def load_openapi_paths() -> dict[str, dict]:
    with OPENAPI.open() as handle:
        spec = json.load(handle)
    result: dict[str, dict] = {}
    for path, methods in spec.get("paths", {}).items():
        get = methods.get("get")
        if get is not None:
            result[path] = get
    return result


def endpoint_table_paths() -> dict[str, str]:
    """从 src/endpoint.rs 的 `path()` 中解析出 `变体名 -> 路径`。"""
    text = (SRC / "endpoint.rs").read_text(encoding="utf-8")
    return dict(re.findall(r'Endpoint::(\w+) => "(/[^"]+)"', text))


def fixture_success_stats() -> tuple[dict[str, int], dict[str, int]]:
    """统计每个业务域的成功夹具（`code == 200`）数量与全部夹具数量。"""
    success = {domain: 0 for domain in DOMAINS}
    total = {domain: 0 for domain in DOMAINS}
    for path in sorted(FIXTURES.glob("*.json")):
        domain = path.stem.split("_")[0]
        if domain not in total:
            continue
        total[domain] += 1
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        if payload.get("code") == 200:
            success[domain] += 1
    return success, total


def has_field_assertions(test_text: str) -> bool:
    """启发式判断模型测试是否做了字段级断言。

    通过条件：测试体出现 `assert_eq!`，或出现针对字段访问的 `assert!`。
    单纯 `assert!(envelope.is_success())` 不算字段级断言。
    """
    if "assert_eq!" in test_text:
        return True
    for match in re.finditer(r"assert!\((.*?)\);", test_text, re.S):
        body = match.group(1)
        if ".is_success(" in body:
            continue
        if re.search(r"\.\w+", body):
            return True
    return False


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="覆盖率自检")
    parser.add_argument(
        "--strict",
        action="store_true",
        help="把 WARN 项（无成功夹具 / 测试无字段级断言）也计入失败",
    )
    args = parser.parse_args(argv)

    print(f"仓库根目录: {ROOT}")

    failures: list[str] = []

    # ---- 1. HTTP 路径覆盖 -------------------------------------------------
    paths = load_openapi_paths()
    all_api_src = "\n".join(p.read_text(encoding="utf-8") for p in API.glob("*.rs"))
    missing_paths = [
        path for path in sorted(paths) if f'"{path}"' not in all_api_src
    ]
    print(f"\n[1] OpenAPI GET 路径: {len(paths)} 个，已实现 {len(paths) - len(missing_paths)} 个")
    for path in missing_paths:
        failures.append(f"未实现接口: {path}")
        print(f"    ✗ 缺失 {path}")

    # ---- 1b. Endpoint 表与 OpenAPI 一致性 ---------------------------------
    table = endpoint_table_paths()
    table_paths = set(table.values())
    print(f"\n[1b] Endpoint 表: {len(table)} 个变体 / {len(table_paths)} 条唯一路径")
    for path in sorted(table_paths - set(paths)):
        failures.append(f"Endpoint 表多出官方规范之外的路径: {path}")
        print(f"    ✗ 表中有、规范无: {path}")
    for path in sorted(set(paths) - table_paths):
        failures.append(f"Endpoint 表缺少路径: {path}")
        print(f"    ✗ 规范有、表中无: {path}")

    # ---- 2. 域文件完整性 --------------------------------------------------
    print(f"\n[2] 业务域: {len(DOMAINS)} 个")
    for domain in DOMAINS:
        model = MODELS / f"{domain}.rs"
        if not model.exists():
            failures.append(f"缺少模型文件: src/models/{domain}.rs")
            print(f"    ✗ {domain}: 缺少 src/models/{domain}.rs")

    # ---- 3. 残留桩 --------------------------------------------------------
    stubs = [
        p.relative_to(ROOT).as_posix()
        for p in list(SRC.rglob("*.rs"))
        if "待实现" in p.read_text(encoding="utf-8")
    ]
    print(f"\n[3] 残留「待实现」文件: {len(stubs)} 个")
    for stub in stubs:
        failures.append(f"残留待实现: {stub}")
        print(f"    ✗ {stub}")

    # ---- 4. 模型测试与夹具 ------------------------------------------------
    # 允许 `cargo fmt` 把 include_str! 折成多行：
    #   parse(include_str!(
    #       "../../tests/fixtures/x.json"
    #   ));
    fixture_pattern = re.compile(
        r'include_str!\(\s*"\.\./\.\./tests/fixtures/([^"]+)"\s*\)', re.S
    )
    referenced: set[str] = set()
    domains_without_tests: list[str] = []
    print(f"\n[4] 模型夹具测试: {len(DOMAINS)} 个域")
    for domain in DOMAINS:
        model = MODELS / f"{domain}.rs"
        if not model.exists():
            continue
        text = model.read_text(encoding="utf-8")
        if "mod tests" not in text:
            domains_without_tests.append(domain)
            failures.append(f"缺少夹具测试: src/models/{domain}.rs")
            print(f"    ✗ {domain}: 没有 #[cfg(test)] mod tests")
            continue
        names = fixture_pattern.findall(text)
        if not names:
            domains_without_tests.append(domain)
            failures.append(f"测试未引用夹具: src/models/{domain}.rs")
            print(f"    ✗ {domain}: 测试里没有 include_str! 夹具")
            continue
        for name in names:
            referenced.add(name)
            if not (FIXTURES / name).exists():
                failures.append(f"夹具不存在: tests/fixtures/{name}")
                print(f"    ✗ {domain}: 引用了不存在的夹具 {name}")

    # ---- 4b. 每域成功夹具统计（默认 WARN，--strict 计入失败） --------------
    success, total = fixture_success_stats()
    no_success = [domain for domain in DOMAINS if total[domain] > 0 and success[domain] == 0]
    no_fixture = [domain for domain in DOMAINS if total[domain] == 0]
    success_files = sum(success.values())
    total_files = sum(total.values())
    print(
        f"\n[4b] 成功夹具覆盖: {success_files}/{total_files} 份夹具 code=200；"
        f"{len(DOMAINS) - len(no_success) - len(no_fixture)}/{len(DOMAINS)} 个域有成功夹具"
    )
    if no_success:
        mode = "计入失败" if args.strict else "WARN，未计入失败"
        print(f"    无成功夹具的域 {len(no_success)} 个（{mode}）")
        for domain in no_success:
            mark = "✗" if args.strict else "·"
            print(f"    {mark} {domain}: {success[domain]}/{total[domain]} 份成功")
            if args.strict:
                failures.append(f"无成功夹具: {domain}（{success[domain]}/{total[domain]}）")
    for domain in no_fixture:
        print(f"    ✗ {domain}: 没有任何夹具")
        failures.append(f"无任何夹具: {domain}")

    # ---- 4c. 模型测试字段级断言（启发式，默认 WARN） ----------------------
    thin: list[str] = []
    for domain in DOMAINS:
        model = MODELS / f"{domain}.rs"
        if not model.exists():
            continue
        text = model.read_text(encoding="utf-8")
        if "mod tests" not in text:
            continue
        if not has_field_assertions(text[text.index("mod tests"):]):
            thin.append(domain)
    print(
        f"\n[4c] 字段级断言（启发式）: {len(DOMAINS) - len(thin)}/{len(DOMAINS)} 个域通过"
    )
    for domain in thin:
        mark = "✗" if args.strict else "·"
        print(f"    {mark} {domain}: 测试未发现 assert_eq! 或字段级 assert!")
        if args.strict:
            failures.append(f"测试缺少字段级断言: {domain}")

    # ---- 5. 夹具使用率（仅提示，不算失败） --------------------------------
    unused = sorted(
        p.name for p in FIXTURES.glob("*.json") if p.name not in referenced
    )
    print(f"\n[5] 未被引用的夹具: {len(unused)} 个（仅供参考）")
    for name in unused:
        print(f"    · {name}")

    # ---- 汇总 --------------------------------------------------------------
    print("\n" + "=" * 60)
    if failures:
        print(f"结果：失败（{len(failures)} 项）")
        return 1

    warnings = len(no_success) + len(thin)
    if warnings:
        print(f"结果：全部通过 ✅（{warnings} 项 WARN 未计入退出码，可用 --strict 复查）")
    else:
        print("结果：全部通过 ✅")
    return 0


if __name__ == "__main__":
    sys.exit(main())