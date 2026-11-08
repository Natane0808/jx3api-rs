# 更新日志

本项目的所有重要变更都记录在这里。

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] - 2026-11-08

基于官方 OpenAPI 的完整重写（与 0.1.0 不兼容）。

### Added

- 覆盖官方 OpenAPI 的 **82 个 HTTP 接口**，按 37 个业务域组织
  （免费 20 / VIP1 31 / VIP2 31），全部为当前 REST 路径的 GET 请求。
- 可选 **WebSocket 客户端**（`feature = "websocket"`）：`wss://socket.nicemoe.cn`，
  39 个 action，自动重连（指数退避）、30 秒心跳、`on` / `on_any` 事件派发。
- 强类型查询参数（`XxxQuery`）与响应模型；`Endpoint` / `Level` 接口元数据。
- 宽松反序列化工具（`flexible_i64` / `flexible_f64` / `flexible_string` /
  `flexible_bool` / `null_as_default`），容忍数字与字符串混用、`null` 与字段缺失。
- 83 份真实响应夹具与离线测试；7 个无成功夹具的域提供环境变量驱动的
  `#[ignore]` 线上测试（`tests/live.rs`），凭据只从 `.env` / 环境变量读取。
- 维护脚本：`gen_endpoints.py`、`gen_readme_table.py`、`probe_api.py`、
  `verify_coverage.py`。
- GitHub Actions CI：`cargo fmt --check`、`clippy -D warnings`、测试、文档、
  `cargo package`、覆盖率与 README 表校验、TOML lint、MSRV 1.85。
- `Cargo.lock` 纳入版本管理，构建可复现。

### Changed

- 统一入口为 `Client` / `ClientBuilder`，业务域通过 `client.<域>()` 获取句柄
  （例如 `client.active().calendar(None)`）。
- 错误模型统一为 `Error`（网络 / HTTP 状态码 / 解析 / 业务码）与 `ApiResponse<T>` 包络。
- 响应模型字段全面容错：缺失字段回落默认值，未知字段忽略。
- `README.md` 重写，新增 `examples/`（basic / role / websocket）与集成测试。

### Fixed

- 修复 `Client` URL 拼接缺少 `/` 的问题。
- 修复 `ApiResponse<T>` 反序列化对 `T: Default` 的额外约束。
- 修复 `/battle/records`（camelCase 字段）、`/duowan/statistics`（`logoUrl` /
  `campName`）、`/calcd/attribute`（面板 `percent`）等静默丢数据的问题。
- `/recruit/search` 补充 `campName` 字段。
- `/arena/recent` 的 `mode` 参数改为整数，与官方 SDK 一致。
- `/arena/recent` 的 `performance` 支持显式 `null`（降级为空战绩）。

### Removed

- 0.1.0 的旧版 API：`src/api.rs`、`src/api/free.rs`、`src/model.rs`、
  `src/model/free.rs`、`src/enums.rs`、`src/err.rs`。
- 旧版 `POST /data/...` 接口路径（服务端已下线）。
- 已废弃的 `[package] authors` 字段。

### Known limitations

- `/arena/*`、`/school/*` 需要推栏 ticket；`/sound/converter` 需要 TTS 凭据。
- `/fraud/detail` 服务端当前返回 `500`（服务未实现）。
- `arena` / `school` / `fraud` / `reward` / `sound` 暂无成功夹具；模型已与官方
  Python SDK（唯一对齐当前 API 的官方 SDK）逐字段核对一致，待凭据到位后补充
  真实响应验证。

## [0.1.0] - 2025-10-08

- 初始版本：基于旧版 `/data` 前缀 + `POST` 请求的接口封装。

[0.2.0]: https://github.com/Natane0808/jx3api-rs/releases/tag/v0.2.0
[0.1.0]: https://crates.io/crates/jx3-api/0.1.0