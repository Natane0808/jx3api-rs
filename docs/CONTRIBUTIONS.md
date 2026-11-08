# 模块实现约定（新增 / 修改业务域时必读）

本文件面向**维护者**：说明这个 crate 的组织方式、每个业务域怎么落代码、怎么用本地夹具
离线验证，以及有哪些坑。参考实现：[`src/api/active.rs`](../src/api/active.rs) 与
[`src/models/active.rs`](../src/models/active.rs)。

## 0. 项目背景与关键事实

| 项目 | 结论 |
| --- | --- |
| 规范来源 | `https://www.jx3api.com/openapi`（HTTP）、`https://www.jx3api.com/openapi.socket.json`（WebSocket） |
| 规范快照 | [`openapi/openapi.json`](../openapi/openapi.json) 82 条 GET 路径、[`openapi/openapi.socket.json`](../openapi/openapi.socket.json) 39 个 action |
| 请求方式 | **全部 GET**，参数走 query string（不是 POST + JSON body） |
| 基础地址 | `https://www.jx3api.com`，路径形如 `/active/calendar`（**没有** `/data` 前缀） |
| 鉴权 | `token`（站点令牌）以 query 参数传递；部分接口另需 `ticket`（推栏 Ticket） |
| 响应包络 | `{ "code": 200, "msg": "success", "data": ..., "time": 1794106931 }` |
| 档位 | `x-level`：`0` = 免费 / `1` = VIP1 / `2` = VIP2，与 [`Level`](../src/endpoint.rs) 对应 |
| WebSocket | `wss://socket.nicemoe.cn`，消息 `{ "action": <整数>, "status": ..., "detail": {...} }` |
| 计费 | 每次 HTTP 请求消耗 1 次额度（余量查询不在本仓库的规范快照内） |

> 历史坑：旧版本仓库用的是 `https://www.jx3api.com/data` + POST body，该路径**已全部失效**
> （服务端返回 `404 请求的路径 [App.Data.Xxx] 不存在`）。当前实现严格按官方 OpenAPI 重写。

设计取舍：

* **不做同步阻塞版**（`reqwest::blocking`），避免与 tokio 运行时冲突。
* **不做自动重试 / 限流退避**：重试语义由调用方决定；只有 WebSocket 客户端内置重连。
* **不用 cargo feature 裁剪接口**：档位由服务端判定，本地裁剪只会让 feature 组合爆炸。
* **模型全面容错**：官方字段随时增删、整数与数字字符串混用，因此模型字段默认值回落 +
  [`src/serde_util.rs`](../src/serde_util.rs) 的宽松解析。

## 1. 文件与命名

每个业务域 **X**（对应 OpenAPI 路径前缀 `/x/`）产出两个文件：

| 文件 | 内容 |
| --- | --- |
| `src/api/x.rs` | `XApi` 句柄、查询参数结构体、方法实现、`impl Client` 访问器 |
| `src/models/x.rs` | 该域的响应数据模型 + 夹具单元测试 |

* 句柄结构体命名：`XApi`（`active` → `ActiveApi`，`home` → `HomeApi`）。
* 访问器命名：`Client::x()`，例如 `client.home().flower(...)`。
* 模型结构体命名：`<Domain><Thing>`，例如 `ActiveCelebs`、`HomeFurniture`，**不要**带
  `Query` 后缀（避免与请求结构体撞名）。
* 新增业务域时还要在 [`src/api/mod.rs`](../src/api/mod.rs) 与
  [`src/models/mod.rs`](../src/models/mod.rs) 里加 `pub mod x;`，并重新生成
  [`src/endpoint.rs`](../src/endpoint.rs)（`python scripts/gen_endpoints.py`）。

## 2. 请求参数

* **参数 ≤ 1 个**：方法直接收参数，必填用 `&str` / `i64`，可选用 `Option<T>`。
* **参数 ≥ 2 个**：定义 `XxxQuery` 结构体：

```rust
/// 奇遇记录查询参数。
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventRecordsQuery {
    /// 目标区服。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// 角色名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否补全未触发奇遇。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<i64>,
}

impl EventRecordsQuery {
    /// 创建查询参数。
    pub fn new() -> Self { Self::default() }

    /// 设置区服。
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置角色名。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}
```

* `token` **不要**写进查询结构体，SDK 会自动注入；确需单次覆盖时才加 `ticket` 字段
  （SDK 检测到查询串里已有 `ticket=` 就不再注入全局值）。
* `ticket`（推栏 Ticket）同理：默认取 `Client` 上的全局配置，需要覆盖时再加字段。
* **必填性约定**：`XxxQuery` 的字段统一用 `Option<T>`，便于 `Default` 构造 + 链式 setter；
  参数名、默认值、取值范围以 OpenAPI 为准，必填参数在方法 rustdoc 里标注 **必填**，
  SDK 不做本地必填 / 范围校验（缺失时由服务端返回业务错误）。

## 3. 方法签名与错误

```rust
impl XApi {
    /// 一句话说明接口做什么。
    ///
    /// 参数说明、注意事项、档位要求。
    ///
    /// # Errors
    ///
    /// 网络异常或服务端返回错误码。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # use jx3_api::Client;
    /// # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new()?;
    /// let data = client.x().method(...).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn method(&self, query: &XxxQuery) -> Result<ReturnType> {
        self.inner.get("/x/method", query).await
    }
}
```

* 返回类型是 **响应体的 `data`**，不是整个包络；需要包络时用 `Client::request`。
* `data` 可能为 `null` 且业务上允许时，返回 `Option<T>`（`self.inner.get::<Option<T>>(...)`）。
* 列表型接口返回 `Vec<T>`；单对象接口返回结构体；官方未定型的接口返回
  `serde_json::Value` 或 `Vec<serde_json::Value>`。
* 文档示例统一放 `# use jx3_api::...` 隐藏行，保证 `cargo test --doc` 能编译。

## 4. 数据模型

```rust
/// 家园装饰。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HomeFurniture {
    /// 装饰 id。
    pub id: i64,
    /// 名称。
    pub name: String,
    /// 类型（官方有时返回字符串）。
    #[serde(deserialize_with = "crate::serde_util::flexible_i64")]
    pub r#type: i64,
    /// 图片地址。
    pub image: String,
    /// 产出地图，可能为空。
    pub produce: Option<String>,
}
```

规则：

1. 一律 `#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]`。
2. 一律在结构体上加 `#[serde(default)]`，缺失字段回落默认值。
3. 字段名与 JSON 完全一致（`snake_case`），Rust 关键字用 `r#type`；JSON 是驼峰时用
   `#[serde(rename = "camelName")]`，需要双向兼容时再加 `alias`。
4. 数值字段若官方可能返回字符串，加
   `#[serde(deserialize_with = "crate::serde_util::flexible_i64")]`；
   小数用 `flexible_f64`；字符串字段用 `flexible_string`；布尔用 `flexible_bool`。
5. 可能为 `null` 的字段用 `Option<T>`。
6. 结构体不确定的子结构用 `serde_json::Value` 兜底；超宽结构可用
   `#[serde(flatten)] extra: HashMap<String, serde_json::Value>`。
7. 每个公开字段都要有 `///` 文档（crate 开了 `#![warn(missing_docs)]`）。

## 5. 离线夹具测试

`tests/fixtures/<path_underscored>.json` 保存了真实响应（`scripts/probe_api.py` 抓取）。
在 `src/models/x.rs` 末尾写单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> crate::response::ApiResponse<T> {
        serde_json::from_str(raw).expect("夹具应当能解析")
    }

    #[test]
    fn parses_fixture() {
        let envelope: crate::response::ApiResponse<Vec<HomeFurniture>> =
            parse(include_str!("../../tests/fixtures/home_furniture.json"));
        assert!(envelope.is_success());
        assert!(!envelope.data.unwrap().is_empty());
    }
}
```

* 夹具文件名 = 接口路径把 `/` 换成 `_`，例如 `/home/furniture` → `home_furniture.json`。
* **夹具可能是错误响应**（`code != 200`、`data` 为 `{}`）：此时只断言能解析并对业务码做断言，
  另外用内联 JSON 覆盖字段映射（驼峰重命名、`class` / `type` 关键字、`flatten` 等）。
* [`ApiResponse<T>`](../src/response.rs) 的 `Deserialize` 是手写实现，`T` **不需要**实现 `Default`。

## 6. WebSocket 模块（`feature = "websocket"`）

* 入口：[`src/socket/client.rs`](../src/socket/client.rs) 的 `SocketClient` / `SocketBuilder`。
* 协议：`wss://socket.nicemoe.cn`，包络 `{ action: <整数>, status, detail }`；action 既接受
  整数也接受十进制字符串。
* 39 个 action 集中在 [`src/socket/events.rs`](../src/socket/events.rs) 的 `SocketAction`，
  对应 detail 结构体在 [`src/socket/actions.rs`](../src/socket/actions.rs)。
* 默认行为：30 秒 WebSocket `Ping` 心跳；断线指数退避重连（1s → 60s，最多 10 次）；
  首次连接失败直接返回错误，之后的重连失败在后台静默重试。
* 新增 action 时：从 `openapi/openapi.socket.json` 取 `x-action` 与 `parameters`
  （= detail 字段），同步更新 `SocketAction` / `*Detail` / `SocketEvent` 三处。
* 验证：`cargo build --lib --features websocket`、`cargo test --all-features`。

## 7. 接口元数据与档位

[`src/endpoint.rs`](../src/endpoint.rs) 由 `scripts/gen_endpoints.py` 依据 OpenAPI 生成，
每个 [`Endpoint`](../src/endpoint.rs) 变体提供 `path()` / `level()` / `summary()`，
`Endpoint::ALL` 覆盖全部 82 条路径，可用于运行期自检与文档生成：

```rust
use jx3_api::{Endpoint, Level};

assert_eq!(Endpoint::ActiveCalendar.level(), Level::Free);
assert_eq!(Endpoint::from_path("/role/detail"), Some(Endpoint::RoleDetail));
```

不要手工修改 `src/endpoint.rs`，改规范快照后重新运行生成脚本。

## 8. 维护脚本与自检

| 脚本 | 用途 |
| --- | --- |
| `scripts/gen_readme_table.py` | 从 `openapi/openapi.json` 生成 README 接口表；`--check README.md` 校验是否过期 |
| `scripts/gen_endpoints.py` | 重新生成 `src/endpoint.rs` |
| `scripts/probe_api.py` | 探测线上接口并刷新 `tests/fixtures/`（**消耗额度**，需 `JX3API_TOKEN`） |
| `scripts/verify_coverage.py` | 覆盖率自检：82 条路径 / 37 个域 / 夹具测试 / 残留桩 / Endpoint 表一致性；新增「每域成功夹具」与「字段级断言」启发式检查（默认 WARN，`--strict` 计入失败） |

提交前跑一遍：

```bash
python scripts/verify_coverage.py
python scripts/gen_readme_table.py --check README.md
cargo test --all-features
cargo clippy --all-targets --all-features
```

## 9. 校验清单

* [ ] `cargo build --lib` 与 `cargo build --lib --all-features` 通过，无 warning
* [ ] `cargo test --all-features` 通过（单元 + 集成 + 文档示例）
* [ ] `cargo clippy --all-targets --all-features` 无 warning
* [ ] 每个方法有 rustdoc（含 `# Errors` 与 `no_run` 示例）
* [ ] 参数名、默认值、取值范围与 OpenAPI 的 `parameters` 一致（Query 字段统一 `Option`，见 §2 必填性约定）
* [ ] 路径与方法名一一对应 OpenAPI 的 `paths`
* [ ] `python scripts/gen_readme_table.py --check README.md` 通过
* [ ] `python scripts/verify_coverage.py` 全部通过
* [ ] 没有把令牌写进仓库（`.env` 已在 `.gitignore` 中忽略）