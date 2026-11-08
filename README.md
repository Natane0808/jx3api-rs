# jx3-api

[![CI](https://github.com/Natane0808/jx3api-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Natane0808/jx3api-rs/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/jx3-api)](https://docs.rs/jx3-api)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

[JX3API](https://www.jx3api.com)（剑网3 官方数据接口）的 Rust SDK：**异步**、**类型安全**，
覆盖官方 OpenAPI 中的全部 **82 个 HTTP 接口**，并提供可选的 **WebSocket 实时事件**客户端。

```rust
use jx3_api::Client;

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    // 免费接口无需令牌
    let client = Client::new()?;

    // 今天的活动日历
    let calendar = client.active().calendar(None).await?;
    println!("{} 大战：{}", calendar.date, calendar.war);

    Ok(())
}
```

## 特性

- **全量覆盖**：按官方 OpenAPI 规范实现全部 82 条 GET 路径，路径与档位与规范一致，
  参数名、默认值与取值范围也逐项对齐。
- **按业务域拆分**：通过 `client.<域>()` 获取句柄（如 `client.active()`、`client.role()`），
  每个域一个 `XxxApi`，方法名与接口语义一致。
- **强类型 + 容错**：参数 ≥ 2 的接口提供 `XxxQuery` 查询结构体，结构稳定的响应提供强类型模型
  （官方结构未定型的接口返回 `serde_json::Value`）；模型字段默认值容错，
  数字字段同时接受 JSON 数字与数字字符串，官方字段增删不会导致整体解析失败。
- **统一错误模型**：网络错误、HTTP 状态码、JSON 解析、业务错误码（`code != 200`）统一为
  [`Error`](https://docs.rs/jx3-api/latest/jx3_api/enum.Error.html)，并提供 `code()` / `message()` 辅助方法。
- **档位可见**：`Endpoint` / `Level` 枚举保存每个接口的路径、中文名与所需档位
  （免费 / VIP1 / VIP2），便于运行期自检。
- **可选 WebSocket**：`websocket` feature 提供实时事件推送客户端（39 个事件 action），
  默认不引入相关依赖。
- **可离线测试**：`tests/fixtures/` 固化了真实响应快照，默认 `cargo test` 不消耗 API 调用次数。

## 安装

```bash
cargo add jx3-api
```

启用 WebSocket 实时事件：

```bash
cargo add jx3-api --features websocket
```

在 `Cargo.toml` 中手动声明时：

```toml
[dependencies]
jx3-api = "0.2"
# 需要实时事件时启用：
# jx3-api = { version = "0.2", features = ["websocket"] }
```

本 crate 只提供异步 API，运行时需由调用方提供 Tokio。要求 Rust 1.85+（edition 2024）：

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

更多示例见 [`examples/`](examples/)：免费接口 [`basic.rs`](examples/basic.rs)、
令牌查询 [`role.rs`](examples/role.rs)、实时事件 [`websocket.rs`](examples/websocket.rs)。

## 快速开始

下面的例子调用**免费接口** `/active/calendar`，无需任何令牌即可运行：

```rust
use jx3_api::Client;

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    let client = Client::new()?;
    let calendar = client.active().calendar(None).await?;

    println!("{} 星期{}", calendar.date, calendar.week);
    println!("大战：{}", calendar.war);
    println!("战场：{}", calendar.battle);
    println!("矿车：{}", calendar.orecar);

    Ok(())
}
```

完整可运行版本见 [`examples/basic.rs`](examples/basic.rs)：

```bash
cargo run --example basic
```

会员接口（VIP1 / VIP2）需要站点令牌，从环境变量读取，**不要把令牌写进代码或提交到仓库**：

```rust
use jx3_api::api::role::RoleDetailQuery;
use jx3_api::Client;

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    let token = std::env::var("JX3API_TOKEN").expect("请先设置 JX3API_TOKEN");
    let client = Client::with_token(token)?;

    // /role/detail 需要 VIP1 及以上档位
    let query = RoleDetailQuery::default()
        .server("长安城")
        .name("漓心")
        .history(1);
    let role = client.role().detail(&query).await?;
    println!("{} 门派 {}", role.role_name, role.force_name);

    Ok(())
}
```

## 客户端配置

`Client::new()` / `Client::with_token(..)` 是常用便捷入口；需要更多控制时使用 `ClientBuilder`：

```rust
use std::time::Duration;

use jx3_api::Client;

fn build_client() -> Result<Client, Box<dyn std::error::Error>> {
    let token = std::env::var("JX3API_TOKEN")?;

    let mut builder = Client::builder()
        .token(token)                       // 站点令牌，缺省时只能访问免费接口
        .timeout(Duration::from_secs(15))   // 请求超时，默认 30 秒
        .base_url("https://www.jx3api.com") // API 根地址，默认即官方地址
        .user_agent("my-app/1.0");          // 自定义 User-Agent

    // 可选：心法 / 名剑等接口需要推栏 Ticket
    if let Ok(ticket) = std::env::var("JX3API_TICKET") {
        builder = builder.ticket(ticket);
    }

    Ok(builder.build()?)
}
```

> **本地开发提示**：SDK **不会**自动加载 `.env` 文件。请显式导出环境变量，例如
> PowerShell `$env:JX3API_TOKEN = "jx3api::xxxxxxxx"`、Bash `export JX3API_TOKEN=...`。
> 示例代码与线上测试读取环境变量 `JX3API_TOKEN`，同时兼容常用的 `API_TOKEN` 变量名。
> 仓库根目录的 `.env` 已在 `.gitignore` 中忽略，可用 shell / dotenv 工具加载后再导出。

| 构造器方法 | 默认值 | 说明 |
| --- | --- | --- |
| `token(impl Into<String>)` | 无 | 站点令牌，以 query 参数 `token` 自动注入每个请求 |
| `ticket(impl Into<String>)` | 无 | 推栏 Ticket，以 query 参数 `ticket` 自动注入 |
| `timeout(Duration)` | 30 秒 | 请求超时；通过 `http_client` 注入自定义客户端时由该客户端决定 |
| `base_url(impl Into<String>)` | `https://www.jx3api.com` | API 根地址（自建代理 / 测试服务器时使用） |
| `user_agent(impl Into<String>)` | `jx3-api/<版本>` | 请求头 `User-Agent` |
| `http_client(reqwest::Client)` | 内部构造 | 复用自定义 `reqwest` 客户端（代理、连接池、证书等） |

使用自定义 `reqwest` 客户端（需要与你项目中的 `reqwest` 版本一致，当前为 `0.12`）：

```rust
use jx3_api::Client;

fn build_client_with_proxy() -> Result<Client, Box<dyn std::error::Error>> {
    let http = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7890")?)
        .build()?;

    Ok(Client::builder().http_client(http).build()?)
}
```

`Client` 可克隆、可跨线程共享（内部共享同一个连接池），配置的令牌与 Ticket 也可以在运行时替换：

```rust
fn rotate_credentials(client: &jx3_api::Client) -> Result<(), Box<dyn std::error::Error>> {
    // 运行时替换站点令牌
    client.set_token(Some(std::env::var("JX3API_TOKEN")?));

    // 清空推栏 Ticket
    client.set_ticket(None);

    Ok(())
}
```

## 档位说明

每个接口所需的档位与官方 OpenAPI 的 `x-level` 字段一致，对应
[`Level`](https://docs.rs/jx3-api/latest/jx3_api/enum.Level.html) 枚举与
[`Endpoint::level`](https://docs.rs/jx3-api/latest/jx3_api/enum.Endpoint.html)：

| 档位 | `Level` | `x-level` | 需要的令牌 | 接口数 | 说明 |
| --- | --- | ---: | --- | ---: | --- |
| 免费 | `Level::Free` | `0` | 不需要 | 20 | 无需登录即可调用 |
| VIP1 | `Level::Vip1` | `1` | VIP1 及以上 | 31 | 令牌档位由 [jx3api.com](https://www.jx3api.com) 账户决定 |
| VIP2 | `Level::Vip2` | `2` | VIP2 | 31 | 最高档位 |

- 档位是接口的**最低要求**，高等级令牌通常可以调用低等级接口（以站点套餐说明为准）。
- 档位不足时服务端仍会返回 HTTP 200，但业务码不是 `200`，SDK 会映射为
  [`Error::Api`](https://docs.rs/jx3-api/latest/jx3_api/enum.Error.html)（含 `code` / `msg` / `path`）。
- 运行期判断接口档位：

```rust
use jx3_api::{Endpoint, Level};

fn level_demo() {
    assert_eq!(Endpoint::ActiveCalendar.level(), Level::Free);
    assert_eq!(Endpoint::RoleDetail.level(), Level::Vip1);
    assert!(!Endpoint::TradeDemon.is_free());
}
```

## 查询参数与必填性

`XxxQuery` 的字段统一为 `Option<T>`，便于 `Default` 构造与链式 setter 按需覆盖；字段名、默认值、
取值范围与官方 OpenAPI 规范一致，必填参数会在各方法的 rustdoc 中标注 **必填**。SDK 不做本地
必填 / 范围校验，漏传必填参数时由服务端返回业务错误（映射为 `Error::Api`）。

## 完整接口表

<!-- BEGIN:ENDPOINT_TABLE -->

> 本表由 [`scripts/gen_readme_table.py`](scripts/gen_readme_table.py) 依据 [`openapi/openapi.json`](openapi/openapi.json) 自动生成，请勿手工修改。
>
> 共 **82** 个 HTTP 接口（免费 **20** / VIP1 **31** / VIP2 **31**），按 **37** 个业务域分组。每个业务域通过 `Client` 上的同名访问器获取句柄，例如 `client.active()`；档位含义见上文「档位说明」。

| 分组 | 业务域 | 调用入口 | 接口数 | 免费 | VIP1 | VIP2 |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| 活动情报 | `active` | `client.active()` | 2 | 2 | 0 | 0 |
| 活动情报 | `castle` | `client.castle()` | 1 | 0 | 0 | 1 |
| 活动情报 | `chitu` | `client.chitu()` | 2 | 0 | 0 | 2 |
| 活动情报 | `firework` | `client.firework()` | 1 | 0 | 0 | 1 |
| 活动情报 | `ranch` | `client.ranch()` | 2 | 0 | 1 | 1 |
| 排行榜单 | `arena` | `client.arena()` | 3 | 0 | 3 | 0 |
| 排行榜单 | `rank` | `client.rank()` | 7 | 0 | 2 | 5 |
| 排行榜单 | `tuilan` | `client.tuilan()` | 1 | 0 | 0 | 1 |
| 物价交易 | `auction` | `client.auction()` | 1 | 0 | 0 | 1 |
| 物价交易 | `steed` | `client.steed()` | 1 | 0 | 0 | 1 |
| 物价交易 | `trade` | `client.trade()` | 6 | 0 | 4 | 2 |
| 阵营战场 | `battle` | `client.battle()` | 1 | 1 | 0 | 0 |
| 阵营战场 | `sand` | `client.sand()` | 1 | 0 | 1 | 0 |
| 阵营战场 | `wicked` | `client.wicked()` | 1 | 0 | 0 | 1 |
| 角色查询 | `calcd` | `client.calcd()` | 3 | 0 | 0 | 3 |
| 角色查询 | `card` | `client.card()` | 5 | 0 | 0 | 5 |
| 角色查询 | `event` | `client.event()` | 6 | 0 | 6 | 0 |
| 角色查询 | `monster` | `client.monster()` | 2 | 0 | 0 | 2 |
| 角色查询 | `role` | `client.role()` | 2 | 0 | 1 | 1 |
| 角色查询 | `school` | `client.school()` | 5 | 0 | 4 | 1 |
| 游戏社区 | `chat` | `client.chat()` | 1 | 0 | 0 | 1 |
| 游戏社区 | `duowan` | `client.duowan()` | 1 | 1 | 0 | 0 |
| 游戏社区 | `food` | `client.food()` | 1 | 1 | 0 | 0 |
| 游戏社区 | `fraud` | `client.fraud()` | 1 | 0 | 1 | 0 |
| 游戏社区 | `home` | `client.home()` | 3 | 3 | 0 | 0 |
| 游戏社区 | `mentor` | `client.mentor()` | 1 | 0 | 1 | 0 |
| 游戏社区 | `news` | `client.news()` | 2 | 2 | 0 | 0 |
| 游戏社区 | `recruit` | `client.recruit()` | 1 | 0 | 1 | 0 |
| 游戏社区 | `saohua` | `client.saohua()` | 7 | 6 | 0 | 1 |
| 游戏社区 | `tieba` | `client.tieba()` | 2 | 0 | 2 | 0 |
| 系统工具 | `exam` | `client.exam()` | 1 | 1 | 0 | 0 |
| 系统工具 | `server` | `client.server()` | 1 | 1 | 0 | 0 |
| 系统工具 | `skill` | `client.skill()` | 2 | 1 | 0 | 1 |
| 系统工具 | `sound` | `client.sound()` | 1 | 1 | 0 | 0 |
| 副本秘境 | `mech` | `client.mech()` | 1 | 0 | 1 | 0 |
| 副本秘境 | `reward` | `client.reward()` | 1 | 0 | 1 | 0 |
| 试炼之地 | `trial` | `client.trial()` | 2 | 0 | 2 | 0 |

### 活动情报

#### `active` · 活动日历 —— 调用入口 `client.active()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/active/calendar` | 免费 | 活动日历 |
| `/active/celebs` | 免费 | 地图活动 |

#### `castle` · 世界首领 —— 调用入口 `client.castle()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/castle/status` | VIP2 | 关隘首领 |

#### `chitu` · 赤兔查询 —— 调用入口 `client.chitu()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/chitu/records` | VIP2 | 今日赤兔 |
| `/chitu/week/records` | VIP2 | 本周赤兔 |

#### `firework` · 烟花事件 —— 调用入口 `client.firework()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/firework/records` | VIP2 | 烟花记录 |

#### `ranch` · 马场查询 —— 调用入口 `client.ranch()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/ranch/chat` | VIP2 | 马场预告 |
| `/ranch/records` | VIP1 | 马场事件 |

### 排行榜单

#### `arena` · 名剑相关 —— 调用入口 `client.arena()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/arena/awesome` | VIP1 | 名剑排行 |
| `/arena/recent` | VIP1 | 名剑战绩 |
| `/arena/schools` | VIP1 | 名剑统计 |

#### `rank` · 排行榜单 —— 调用入口 `client.rank()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/rank/arena` | VIP2 | 跨服名剑 |
| `/rank/championship` | VIP2 | 武林争霸 |
| `/rank/constable` | VIP2 | 捕快荣誉 |
| `/rank/outlaw` | VIP2 | 江湖浪客 |
| `/rank/statistics` | VIP1 | 排行统计 |
| `/rank/trials` | VIP1 | 试炼之地 |
| `/rank/wanted` | VIP2 | 决斗挑战 |

#### `tuilan` · 资历分布 —— 调用入口 `client.tuilan()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/tuilan/achievement` | VIP2 | 资历分布 |

### 物价交易

#### `auction` · 阵营竞拍 —— 调用入口 `client.auction()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/auction/records` | VIP2 | 阵营拍卖 |

#### `steed` · 的卢拍卖 —— 调用入口 `client.steed()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/steed/records` | VIP2 | 的卢拍卖 |

#### `trade` · 物价交易 —— 调用入口 `client.trade()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/trade/demon` | VIP1 | 金价行情 |
| `/trade/item/records` | VIP1 | 外观价格 |
| `/trade/item/search` | VIP1 | 外观搜索 |
| `/trade/manufacture` | VIP2 | 成本计算 |
| `/trade/records` | VIP1 | 物品价格 |
| `/trade/wanbaolou` | VIP2 | 编号搜索 |

### 阵营战场

#### `battle` · 帮战记录 —— 调用入口 `client.battle()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/battle/records` | 免费 | 帮战记录 |

#### `sand` · 阵营沙盘 —— 调用入口 `client.sand()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/sand/records` | VIP1 | 沙盘据点 |

#### `wicked` · 诛恶事件 —— 调用入口 `client.wicked()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/wicked/records` | VIP2 | 诛恶事件 |

### 角色查询

#### `calcd` · 角色属性 —— 调用入口 `client.calcd()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/calcd/attribute` | VIP2 | 装备属性 |
| `/calcd/detail` | VIP2 | 装备详情 |
| `/calcd/records` | VIP2 | 装备记录 |

#### `card` · 角色名片 —— 调用入口 `client.card()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/card/cached` | VIP2 | 名片缓存 |
| `/card/preset` | VIP2 | 名片预设 |
| `/card/random` | VIP2 | 随机名片 |
| `/card/record` | VIP2 | 名片记录 |
| `/card/records` | VIP2 | 名片历史 |

#### `event` · 奇遇相关 —— 调用入口 `client.event()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/event/collect` | VIP1 | 奇遇汇总 |
| `/event/missing` | VIP1 | 未触发的 |
| `/event/recent` | VIP1 | 近期奇遇 |
| `/event/records` | VIP1 | 奇遇记录 |
| `/event/statistics` | VIP1 | 奇遇统计 |
| `/event/strategy` | VIP1 | 奇遇攻略 |

#### `monster` · 百战记录 —— 调用入口 `client.monster()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/monster/records` | VIP2 | 角色百战 |
| `/monster/weekly` | VIP2 | 百战首领 |

#### `role` · 角色信息 —— 调用入口 `client.role()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/role/achievement` | VIP2 | 成就查询 |
| `/role/detail` | VIP1 | 角色详情 |

#### `school` · 心法配装 —— 调用入口 `client.school()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/school/matrix` | VIP1 | 阵眼效果 |
| `/school/search` | VIP2 | 配装搜索 |
| `/school/seniority` | VIP1 | 资历榜单 |
| `/school/skills` | VIP1 | 技能信息 |
| `/school/talent` | VIP1 | 奇穴信息 |

### 游戏社区

#### `chat` · 聊天记录 —— 调用入口 `client.chat()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/chat/records` | VIP2 | 角色聊天 |

#### `duowan` · 多玩语音 —— 调用入口 `client.duowan()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/duowan/statistics` | 免费 | 统战歪歪 |

#### `food` · 小吃小药 —— 调用入口 `client.food()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/food/list` | 免费 | 小吃小药 |

#### `fraud` · 骗子查询 —— 调用入口 `client.fraud()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/fraud/detail` | VIP1 | 骗子查询 |

#### `home` · 家园相关 —— 调用入口 `client.home()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/home/flower` | 免费 | 家园鲜花 |
| `/home/furniture` | 免费 | 家园装饰 |
| `/home/travel` | 免费 | 器物图谱 |

#### `mentor` · 师徒相关 —— 调用入口 `client.mentor()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/mentor/search` | VIP1 | 师徒列表 |

#### `news` · 新闻公告 —— 调用入口 `client.news()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/news/announce` | 免费 | 维护公告 |
| `/news/records` | 免费 | 新闻资讯 |

#### `recruit` · 团队招募 —— 调用入口 `client.recruit()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/recruit/search` | VIP1 | 团队招募 |

#### `saohua` · 骚话语录 —— 调用入口 `client.saohua()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/saohua/answer` | 免费 | 答案之书 |
| `/saohua/content` | 免费 | 舔狗日志 |
| `/saohua/context` | VIP2 | 分类语录 |
| `/saohua/drink` | 免费 | 喝什么 |
| `/saohua/eat` | 免费 | 吃什么 |
| `/saohua/random` | 免费 | 随机骚话 |
| `/saohua/zhanan` | 免费 | 渣男语录 |

#### `tieba` · 百度贴吧 —— 调用入口 `client.tieba()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/tieba/item/records` | VIP1 | 外观记录 |
| `/tieba/random` | VIP1 | 百度贴吧 |

### 系统工具

#### `exam` · 科举搜索 —— 调用入口 `client.exam()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/exam/search` | 免费 | 科举答案 |

#### `server` · 大区状态 —— 调用入口 `client.server()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/server/status/check` | 免费 | 大区状态 |

#### `skill` · 急速计算 —— 调用入口 `client.skill()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/skill/calculate` | VIP2 | 急速计算 |
| `/skill/rework` | 免费 | 技改记录 |

#### `sound` · 语音合成 —— 调用入口 `client.sound()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/sound/converter` | 免费 | 语音合成 |

### 副本秘境

#### `mech` · 秘境方位 —— 调用入口 `client.mech()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/mech/decrypt` | VIP1 | 秘境方位 |

#### `reward` · 掉落统计 —— 调用入口 `client.reward()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/reward/statistics` | VIP1 | 掉落统计 |

### 试炼之地

#### `trial` · 试炼之地 —— 调用入口 `client.trial()`

| 路径 | 档位 | 说明 |
| --- | --- | --- |
| `/trial/bosses` | VIP1 | 秒伤计算 |
| `/trial/seasons` | VIP1 | 赛季列表 |

<!-- END:ENDPOINT_TABLE -->

> `/active/calendar` 在 SDK 中拆成 `calendar`（`mode=day`）与 `list_calendar`（`mode=list`）两个方法，
> 因此公开方法数（83）比路径数（82）多 1。

## WebSocket 实时事件

官方 WebSocket（`wss://socket.nicemoe.cn`）会主动推送游戏世界事件。
该功能由 `websocket` feature 提供，**默认关闭**，不引入额外依赖：

```bash
cargo add jx3-api --features websocket
```

连接并订阅事件：

```rust
use std::time::Duration;

use jx3_api::socket::{SocketAction, SocketClient, SocketEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = SocketClient::builder();

    // 令牌可选：设置后附加到连接地址的 query 中
    if let Ok(token) = std::env::var("JX3API_TOKEN") {
        builder = builder.token(token);
    }

    let client = builder
        .reconnect(true)                    // 断线自动重连（默认开启）
        .heartbeat(Some(Duration::from_secs(30))) // 心跳间隔，None 关闭
        .build()?;

    // 订阅单个 action，回调里拿到强类型 detail
    client.on(SocketAction::EventTrigger, |event| {
        if let SocketEvent::EventTrigger(detail) = event {
            println!("{} {} 触发 {}", detail.server, detail.name, detail.event);
        }
    });

    // 订阅全部 action
    client.on_any(|event| println!("[{}] {:?}", event.name(), event.action()));

    // 建立连接，后台任务自动收包并按 action 派发
    let handle = client.connect().await?;

    // ……业务处理，直到收到 Ctrl+C……
    tokio::signal::ctrl_c().await?;
    handle.close().await?;

    Ok(())
}
```

每条推送消息的包络为：

```json
{
  "action": 1001,
  "status": "success",
  "detail": { "zone": "电信一区", "server": "长安城", "name": "叶英", "event": "黑白路", "time": 1794106931 }
}
```

SDK 会把 `detail` 按 `action` 解析为 `SocketEvent` 的强类型变体
（例如 `SocketEvent::EventTrigger(EventTriggerDetail)`），字段缺失时取默认值。
全部 39 个事件定义在 `jx3_api::socket::events`，对应的 `detail` 结构体在
`jx3_api::socket::actions`（客户端实现见 [`src/socket/`](src/socket/)）：

| 分类 | action |
| --- | --- |
| 奇遇事件 | `1001` |
| 马场事件 | `1002`、`1003` |
| 扶摇事件 | `1005`、`1006` |
| 的卢事件 | `1008`–`1011` |
| 掉落交易 | `1012`、`1013` |
| 江湖事件 | `1014`、`1015` |
| 阵营祭祀 | `1017` |
| 关隘首领 | `1018` |
| 宣战约战 | `1101`–`1105` |
| 据点攻防 | `1111`–`1122` |
| 社交动态 | `1201` |
| 系统通知 | `2001`–`2006` |

其他约定：

- 用 `SocketAction::ALL` 遍历全部 action，`SocketAction::as_u32()` / `as_str()` 取回数值；
- 默认自动重连：断线后按指数退避（1s、2s、4s……最多 60s）重试，连续失败 10 次后停止，
  可用 `.reconnect(false)` / `.max_reconnects(n)` 调整；
- 重连次数耗尽后客户端进入**终止状态**（`is_closed()` 返回 `true`），`run()` 会在后台循环结束后
  返回 `Ok(())`；如需继续使用请重新构建客户端；
- 默认每 30 秒发送一次 Ping 心跳，`.heartbeat(..)` 可修改间隔或传 `None` 关闭；
- `SocketClient::run()` 会连接并阻塞到运行循环结束；`SocketHandle::close()` / `join()` /
  `abort()` 提供细粒度控制。

完整可运行版本见 [`examples/websocket.rs`](examples/websocket.rs)，该示例用 `tokio::signal`
监听 Ctrl+C，因此 Tokio 需要启用 `signal` feature（或直接使用 `features = ["full"]`）：

```bash
cargo run --example websocket --features websocket
```

## 错误处理

所有接口方法返回 [`Result<T>`](https://docs.rs/jx3-api/latest/jx3_api/type.Result.html)，
错误类型为 [`Error`](https://docs.rs/jx3-api/latest/jx3_api/enum.Error.html)：

| `Error` 变体 | 触发场景 | `code()` | `message()` |
| --- | --- | --- | --- |
| `Error::Http(reqwest::Error)` | 连接失败、超时、TLS 等网络层错误 | `None` | 错误描述 |
| `Error::HttpStatus { status, body }` | HTTP 状态码非 2xx，且响应体不是标准包络 | `Some(status)` | 原始响应体 |
| `Error::Decode(serde_json::Error)` | 响应不是合法 JSON，或 `data` 与模型不匹配 | `None` | 错误描述 |
| `Error::Encode(String)` | 查询参数序列化失败 | `None` | 错误描述 |
| `Error::Api { code, msg, path }` | 服务端返回业务错误码（如未收录、参数错误、档位不足） | `Some(code)` | 服务端 `msg` |
| `Error::EmptyData { path }` | 期望非空数据但 `data` 为 `null` | `None` | 错误描述 |

按业务错误与网络错误分别处理：

```rust
use jx3_api::{Client, Error};

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    let client = Client::new()?;

    match client.active().calendar(None).await {
        Ok(calendar) => println!("{} 大战：{}", calendar.date, calendar.war),
        Err(Error::Api { code, msg, path }) => {
            // 服务端明确返回的业务错误，例如参数越界、档位不足
            eprintln!("业务错误 {code} @ {path}: {msg}");
        }
        Err(err) => {
            // 网络 / 解析错误
            eprintln!("请求失败：{err}");
        }
    }

    Ok(())
}
```

服务端统一响应包络为：

```json
{ "code": 200, "msg": "success", "data": { "...": "..." }, "time": 1794106931 }
```

SDK 的业务方法默认直接返回 `data`；需要完整包络（`code` / `msg` / `time`）时使用
[`Client::request`](https://docs.rs/jx3-api/latest/jx3_api/struct.Client.html#method.request)，
需要绕过业务方法调用未封装接口时使用 `Client::get`：

```rust
use jx3_api::Client;

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    let client = Client::new()?;

    // 完整包络：ApiResponse<T>
    let response = client
        .request::<serde_json::Value>("/active/celebs", &[("name", "楚天社")])
        .await?;
    assert_eq!(response.code, 200);
    println!("msg = {}, time = {}", response.msg, response.time);
    println!("data = {:#?}", response.data);

    // 只要 data，自行指定反序列化类型
    let raw: serde_json::Value = client
        .get("/active/celebs", &[("name", "楚天社")])
        .await?;
    println!("{raw:#?}");

    Ok(())
}
```

> `Error::Api` 只会在业务码非 `200` 时产生；`Client::request` 成功返回时
> `ApiResponse::is_success()` 一定为 `true`。也可以用 `ApiResponse::into_data(path)`
> 把包络转换为 `data`，`data` 为空时得到 `Error::EmptyData`。

## 测试与夹具

仓库自带三类测试：

| 类型 | 位置 | 是否联网 | 命令 |
| --- | --- | --- | --- |
| 单元测试 | `src/**/*.rs` 内的 `#[cfg(test)]` | 否 | `cargo test` |
| 集成测试 | [`tests/integration.rs`](tests/integration.rs) | 离线部分否 / 线上部分需 `--ignored` | `cargo test --test integration` |
| 线上真实响应测试 | [`tests/live.rs`](tests/live.rs) | 是（全部 `#[ignore]`） | `cargo test --test live -- --ignored` |

离线测试直接读取 [`tests/fixtures/`](tests/fixtures/) 下的真实响应快照（83 份 JSON，
覆盖成功与业务错误两类包络；多数文件按「接口路径把 `/` 换成 `_`」命名，
另有 `/active/calendar` 的 `list` 模式补充快照），不消耗 API 调用次数：

```rust
// tests/integration.rs
#[test]
fn parses_active_calendar_fixture() {
    let raw = include_str!("fixtures/active_calendar.json");
    let value: serde_json::Value = serde_json::from_str(raw).unwrap();
    assert_eq!(value["code"], 200);
}
```

线上冒烟测试默认被 `#[ignore]` 标记，只有显式指定并设置 `JX3API_TOKEN` 时才会执行
（会消耗真实请求额度）：

```bash
# 只跑离线测试
cargo test

# 运行集成测试的离线部分
cargo test --test integration

# 额外运行线上冒烟测试（需要 JX3API_TOKEN）
cargo test --test integration -- --ignored

# 运行 7 个「无成功夹具」业务域的真实响应测试（需要令牌与区服/角色等凭据）
cargo test --test live -- --ignored --nocapture

# 启用 websocket feature 后进行全量测试
cargo test --all-features
```

[`tests/live.rs`](tests/live.rs) 覆盖 `arena` / `school` / `fraud` / `reward` / `recruit` /
`tuilan` / `sound` 这些没有成功夹具的域：它从环境变量（或仓库根目录的 `.env`，该文件
已被 `.gitignore` 忽略）读取令牌与查询参数，缺少某项凭据时自动打印「跳过」并通过；
只有**拿到真实成功响应但模型解析失败**才会报错。支持的变量见该文件头部文档
（`JX3API_TOKEN`、`JX3API_SERVER`、`JX3API_ROLE`、`JX3API_TICKET`、`JX3API_FRAUD_UID`、
`JX3API_SCHOOL`、`JX3API_TTS_*`）。真实区服 / 角色 / 响应不会写入仓库。

刷新夹具（会访问真实接口并消耗额度，维护者使用）：

```bash
python scripts/probe_api.py
```

## 项目结构

```text
jx3-api/
├── Cargo.toml
├── README.md
├── CHANGELOG.md                # 版本变更记录
├── LICENSE
├── openapi/                    # 官方 OpenAPI 规范快照
│   ├── openapi.json            # 82 个 HTTP 接口
│   └── openapi.socket.json     # 39 个 WebSocket 事件
├── scripts/                    # 维护脚本（生成文档、刷新夹具、覆盖率校验）
│   ├── gen_readme_table.py     # 生成本 README 的接口表
│   ├── gen_endpoints.py        # 生成 src/endpoint.rs
│   ├── probe_api.py            # 探测线上接口并刷新夹具
│   └── verify_coverage.py      # 校验接口 / 模型覆盖率
├── examples/
│   ├── basic.rs                # 免费接口
│   ├── role.rs                 # 需要令牌的角色查询
│   └── websocket.rs            # 实时事件（feature = "websocket"）
├── tests/
│   ├── integration.rs          # 离线夹具校验 + #[ignore] 线上冒烟
│   └── fixtures/               # 真实响应快照
├── docs/
│   └── CONTRIBUTIONS.md        # 模块实现约定（新增业务域必读）
└── src/
    ├── lib.rs                  # crate 文档、re-export、prelude
    ├── client.rs               # Client / ClientBuilder
    ├── endpoint.rs             # Endpoint / Level 接口元数据
    ├── error.rs                # Error 枚举
    ├── response.rs             # ApiResponse<T> 包络
    ├── serde_util.rs           # 宽松反序列化工具
    ├── api/                    # 37 个业务域，每个域一个文件
    ├── models/                 # 与 api/ 一一对应的数据模型
    └── socket/                 # WebSocket 客户端（feature = "websocket"）
```

## 更新日志

版本变更与破坏性改动见 [CHANGELOG.md](CHANGELOG.md)。

## License

[MIT](LICENSE) © 2025 natane