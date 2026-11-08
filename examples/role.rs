//! 需要令牌的角色查询示例（`/role/detail`，VIP1 及以上档位）。
//!
//! 令牌只从环境变量 `JX3API_TOKEN` 读取，**不要**把令牌硬编码进代码。
//! 未设置令牌时示例会打印提示并正常退出（返回 `Ok(())`），不会发起网络请求。
//!
//! 运行（PowerShell）：
//!
//! ```text
//! $env:JX3API_TOKEN = "jx3api::xxxxxxxx"
//! cargo run --example role
//! ```
//!
//! 运行（Bash / Zsh）：
//!
//! ```text
//! JX3API_TOKEN="jx3api::xxxxxxxx" cargo run --example role
//! ```

use jx3_api::Client;
use jx3_api::api::role::RoleDetailQuery;

/// 查询的区服，按需修改（例如 `长安城`、`梦江南`）。
const SERVER: &str = "长安城";
/// 查询的角色名，按需修改。
const ROLE: &str = "漓心";

/// 读取令牌：优先 `JX3API_TOKEN`，同时兼容常用的 `API_TOKEN` 环境变量。
fn token_from_env() -> Option<String> {
    ["JX3API_TOKEN", "API_TOKEN"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .map(|value| value.trim().to_owned())
        .find(|value| !value.is_empty())
}

#[tokio::main]
async fn main() -> jx3_api::Result<()> {
    let token = match token_from_env() {
        Some(token) => token,
        None => {
            eprintln!("未设置环境变量 JX3API_TOKEN（或 API_TOKEN），跳过角色查询。");
            eprintln!("PowerShell 示例：$env:JX3API_TOKEN = \"jx3api::xxxxxxxx\"");
            return Ok(());
        }
    };

    let client = Client::with_token(token)?;

    // `/role/detail` 需要 `server` + `name`；`history` 可选
    //（0 不返回改名 / 帮会历史，1 返回）。令牌由 Client 自动注入。
    let query = RoleDetailQuery::default()
        .server(SERVER)
        .name(ROLE)
        .history(0);

    println!("正在查询 {SERVER} 的 [{ROLE}]……");
    let detail = client.role().detail(&query).await?;

    println!("区服：{}", detail.server_name);
    println!("角色：{}（{}）", detail.role_name, detail.body_name);
    println!("门派：{}", detail.force_name);
    println!("阵营：{}", detail.camp_name);
    println!("role_id：{}", detail.role_id);

    Ok(())
}
