//! 免费接口示例：活动日历。
//!
//! 免费接口不需要令牌，直接 [`Client::new`] 即可调用。
//! 如果环境变量中设置了 `JX3API_TOKEN`，示例会自动带上令牌，
//! 方便把它扩展成会员接口的调用示例。
//!
//! 运行：
//!
//! ```text
//! cargo run --example basic
//! ```
//!
//! 该示例会访问真实网络（`https://www.jx3api.com`）。

use jx3_api::Client;

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
    // 免费接口无需令牌；设置了令牌时顺带带上，方便后续切到会员接口。
    let client = match token_from_env() {
        Some(token) => Client::with_token(token)?,
        None => {
            println!("提示：未设置 JX3API_TOKEN，仅调用免费接口。");
            Client::new()?
        }
    };

    // 单日活动日历（默认当天）。
    let today = client.active().calendar(None).await?;
    println!("====== 活动日历 {} 星期{} ======", today.date, today.week);
    println!("大战：{}", today.war);
    println!("战场：{}", today.battle);
    println!("矿车：{}", today.orecar);
    println!("驰援：{}", today.rescue);
    println!("门派：{}", today.school);
    if !today.card.is_empty() {
        println!("名片：{}", today.card.join("、"));
    }
    if !today.lucky.is_empty() {
        println!("奇遇：{}", today.lucky.join("、"));
    }
    if let Some(weekly) = &today.weekly {
        println!("五人周常：{}", weekly.conn.join("、"));
        println!("团队周常：{}", weekly.raid.join("、"));
    }

    // 前后 3 天的活动月历（区间半径可调，0..=15）。
    let month = client.active().list_calendar(Some(3)).await?;
    println!(
        "\n====== 活动月历（以 {} 为中心前后 3 天） ======",
        month.today.date
    );
    for day in &month.total {
        println!(
            "{} 星期{} 大战：{} 战场：{}",
            day.date, day.week, day.war, day.battle
        );
    }

    Ok(())
}
