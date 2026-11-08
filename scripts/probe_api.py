#!/usr/bin/env python3
"""按 OpenAPI 规范探测线上接口，保存原始响应作为离线测试夹具。

用法::

    JX3API_TOKEN=xxx python scripts/probe_api.py [--out tests/fixtures] [--only /a/b]

脚本只依赖标准库。默认读取 ``openapi/openapi.json``，对每个 GET 接口发起一次
请求，把服务端返回的原始 JSON 写进 ``<out>/<path>.json``，同时在 stdout 打印
``code`` / ``msg`` 摘要。已被 ``--only`` 过滤掉的接口不会被请求。
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import sys
import urllib.error
import urllib.parse
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "openapi" / "openapi.json"

# 通用样例值：按参数名给出，缺省时用该表。
SAMPLES: dict[str, object] = {
    "server": "长安城",
    "name": "",
    "subject": "古琴",
    "uid": 123456,
    "label": 1,
    "limit": 20,
    "num": 0,
    "mode": "",
    "ticket": "",
    "class": 1,
    "subclass": "",
    "camp": 1,
    "source": 0,
    "expired": 0,
    "full": 0,
    "history": 0,
    "like": "false",
    "page": 1,
    "update": 0,
    "floor": 1,
    "season": "",
    "kungfu": "纯阳",
    "type": "",
    "format": "calcd",
    "role": "",
    "id": "1",
    "cooldown": 1.5,
    "appkey": "test-appkey",
    "access": "test-access",
    "secret": "test-secret",
    "text": "剑网三",
    "tags": "剑网3",
    "force": "",
    "body": "",
    "voice": "Aitong",
    "sample_rate": 16000,
    "volume": "50",
    "speech_rate": "0",
    "pitch_rate": "0",
}

# 覆盖默认值（OpenAPI 里带默认值的参数）。
OVERRIDES: dict[str, object] = {
    "mode": "33",
    "limit": "20",
}

# 这些参数必须显式出现，即使 OpenAPI 标成可选。
FORCE = {"token", "ticket"}

# 按路径覆盖样例值：同名参数在不同接口下取值范围不同（mode / name 尤其明显）。
PATH_OVERRIDES: dict[str, dict[str, object]] = {
    "/arena/awesome": {"mode": "33", "limit": "20"},
    "/arena/schools": {"mode": "33"},
    "/rank/arena": {"mode": "0"},
    "/rank/wanted": {"mode": "1"},
    "/event/collect": {"num": "7"},
    "/event/statistics": {"name": "白日梦"},
    "/event/strategy": {"name": "白日梦"},
    "/home/flower": {"name": "", "map": ""},
    "/home/furniture": {"name": "龙门香梦"},
    "/home/travel": {"name": "日晷"},
    "/calcd/detail": {"type": "武器"},
    "/role/achievement": {"role": "29450102"},
    "/saohua/context": {"name": "疯狂星期四"},
    "/trial/bosses": {"season": "暗影千机", "floor": "1"},
    "/tieba/item/records": {"name": "白发玉冠"},
    "/tieba/random": {"tags": "剑网3"},
    "/trade/item/search": {"name": "白发玉冠"},
    "/trade/item/records": {"name": "白发玉冠"},
    "/trade/records": {"name": "白发玉冠"},
    "/trade/manufacture": {"name": "菜团子"},
    "/trade/wanbaolou": {"id": "29450102"},
    "/school/search": {"mode": ""},
}


def load_spec() -> dict:
    return json.loads(SPEC.read_text(encoding="utf-8"))


def sample_for(name: str, required: bool) -> object | None:
    if name in OVERRIDES:
        return OVERRIDES[name]
    if name in SAMPLES:
        value = SAMPLES[name]
        if value == "" and not required:
            return None
        return value
    return None


def build_params(op: dict, role_name: str, path: str = "") -> dict[str, object]:
    params: dict[str, object] = {}
    overrides = PATH_OVERRIDES.get(path, {})
    for param in op.get("parameters", []):
        name = param["name"]
        required = bool(param.get("required"))
        if name in overrides:
            value = overrides[name]
            if value == "":
                continue
            params[name] = value
            continue
        if name == "token":
            params[name] = os.environ.get("JX3API_TOKEN", "")
            continue
        if name == "name" and role_name:
            params[name] = role_name
            continue
        value = sample_for(name, required)
        if value is None:
            if required:
                params[name] = ""
            continue
        params[name] = value
    return params


def request(base_url: str, path: str, params: dict[str, object]) -> tuple[int, str]:
    query = urllib.parse.urlencode({k: v for k, v in params.items() if v != ""})
    url = f"{base_url}{path}"
    if query:
        url = f"{url}?{query}"
    req = urllib.request.Request(url, method="GET", headers={"accept": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=40) as resp:
            return resp.status, resp.read().decode("utf-8", "replace")
    except urllib.error.HTTPError as exc:  # 服务端用非 2xx 返回业务错误
        return exc.code, exc.read().decode("utf-8", "replace")
    except Exception as exc:  # 网络异常
        return 0, json.dumps({"code": 0, "msg": str(exc), "data": None, "time": 0}, ensure_ascii=False)


def slug(path: str) -> str:
    return path.strip("/").replace("/", "_") or "root"


def harvest_role(paths: list[str], spec: dict, base_url: str, token: str) -> str:
    """从 /event/recent 等接口里抓一个真实角色名，供依赖 name 的接口使用。"""
    for candidate in ("/event/recent", "/battle/records", "/recruit/search"):
        op = spec["paths"].get(candidate, {}).get("get")
        if not op:
            continue
        params = build_params(op, "", candidate)
        params["server"] = "长安城"
        params["token"] = token
        params["limit"] = 20
        _, body = request(base_url, candidate, params)
        try:
            data = json.loads(body).get("data") or []
        except json.JSONDecodeError:
            continue
        for item in data if isinstance(data, list) else []:
            if isinstance(item, dict):
                for key in ("name", "roleName", "leader"):
                    value = item.get(key)
                    if isinstance(value, str) and value:
                        print(f"[harvest] role name from {candidate}: {value}", file=sys.stderr)
                        return value
    return ""


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", default=str(ROOT / "tests" / "fixtures"))
    parser.add_argument("--base-url", default="https://www.jx3api.com")
    parser.add_argument("--only", action="append", default=[], help="只探测指定路径（可重复）")
    parser.add_argument("--role", default="", help="用于 name 参数的角色名")
    parser.add_argument("--no-role-harvest", action="store_true")
    args = parser.parse_args()

    token = os.environ.get("JX3API_TOKEN", "")
    if not token:
        print("缺少 JX3API_TOKEN 环境变量", file=sys.stderr)
        return 2

    spec = load_spec()
    out_dir = pathlib.Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)

    role_name = args.role
    if not role_name and not args.no_role_harvest:
        role_name = harvest_role(args.only or list(spec["paths"]), spec, args.base_url, token)

    failures = 0
    for path, item in spec["paths"].items():
        op = item.get("get")
        if not op:
            continue
        if args.only and path not in args.only:
            continue
        params = build_params(op, role_name, path)
        status, body = request(args.base_url, path, params)
        (out_dir / f"{slug(path)}.json").write_text(body, encoding="utf-8")
        try:
            parsed = json.loads(body)
            code, msg = parsed.get("code"), parsed.get("msg")
        except json.JSONDecodeError:
            code, msg = status, body[:120]
        flag = "ok " if code == 200 else "ERR"
        if code != 200:
            failures += 1
        print(f"{flag} {path:<26} http={status} code={code} msg={msg}")

    print(f"\n完成，失败 {failures} 个 -> {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
