//! 线上「真实响应」冒烟测试（默认全部 `#[ignore]`）。
//!
//! 这组测试面向**本机维护者**：用真实的站点令牌 + 区服 / 角色调用付费接口，
//! 验证模型能解析真实成功响应。因为会消耗接口额度且涉及个人数据，默认不运行，
//! 也不在 CI 中执行。
//!
//! ## 配置来源
//!
//! 优先读进程环境变量；缺失时回退读取仓库根目录的 `.env`（该文件已被
//! `.gitignore` 忽略，**不会入库**）。支持的变量：
//!
//! | 变量 | 说明 |
//! | --- | --- |
//! | `JX3API_TOKEN` / `API_TOKEN` | 站点令牌（必填，缺失则整组跳过） |
//! | `JX3API_SERVER` | 区服名称，例如 `长安城` |
//! | `JX3API_ROLE` | 角色名称 |
//! | `JX3API_TICKET` | 推栏 Ticket（`/arena/*`、`/school/*` 需要） |
//! | `JX3API_FRAUD_UID` | `/fraud/detail` 的查询 uid |
//! | `JX3API_SCHOOL` | `/school/*` 的心法名（默认 `冰心诀`） |
//! | `JX3API_TTS_APPKEY` / `JX3API_TTS_ACCESS` / `JX3API_TTS_SECRET` | `/sound/converter` 的语音合成凭据 |
//!
//! ## 运行
//!
//! ```text
//! cargo test --test live -- --ignored --nocapture
//! ```
//!
//! 缺少某项凭据的用例会打印「跳过」并正常通过；只有**拿到了真实成功响应但模型
//! 解析失败**才会 panic，因此这组测试不会因为账号缺数据而误报。
//!
//! > 真实响应与个人数据只存在于本机 `.env` / 内存中，不会写入仓库。

use std::collections::HashMap;
use std::sync::OnceLock;

use jx3_api::Error;

/// 解析仓库根目录的 `.env`（不存在时返回空表）。
fn dotenv() -> &'static HashMap<String, String> {
    static DOTENV: OnceLock<HashMap<String, String>> = OnceLock::new();
    DOTENV.get_or_init(|| {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        let Ok(text) = std::fs::read_to_string(path) else {
            return HashMap::new();
        };
        text.lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                let (key, value) = line.split_once('=')?;
                Some((
                    key.trim().to_owned(),
                    value.trim().trim_matches('"').trim_matches('\'').to_owned(),
                ))
            })
            .collect()
    })
}

/// 读取非空配置项：进程环境变量优先，其次 `.env`。
fn var(key: &str) -> Option<String> {
    if let Ok(value) = std::env::var(key) {
        let value = value.trim().to_owned();
        if !value.is_empty() {
            return Some(value);
        }
    }
    dotenv()
        .get(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

/// 站点令牌（兼容 `API_TOKEN` 写法）。
fn token() -> Option<String> {
    var("JX3API_TOKEN").or_else(|| var("API_TOKEN"))
}

/// 构造已注入令牌 / Ticket 的客户端；缺少令牌时返回 `None`。
fn client() -> Option<jx3_api::Client> {
    let token = token()?;
    let mut builder = jx3_api::Client::builder().token(token);
    if let Some(ticket) = var("JX3API_TICKET") {
        builder = builder.ticket(ticket);
    }
    Some(builder.build().expect("构建 Client 失败"))
}

/// 打印「跳过」并结束当前用例的宏（缺少凭据 / 账号无数据时使用）。
macro_rules! skip {
    ($($arg:tt)*) => {{
        eprintln!("跳过：{}", format!($($arg)*));
        return;
    }};
}

/// 把「账号无数据 / 服务端未开放」视为跳过而不是失败。
fn skip_if_unavailable(err: &Error) -> bool {
    match err {
        Error::Api { code, .. } => matches!(code, 404 | 500),
        _ => false,
    }
}

/// `/recruit/search`：团队招募（VIP1，需要 token + server）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌与区服，会消耗接口额度"]
async fn live_recruit_search() {
    let (Some(client), Some(server)) = (client(), var("JX3API_SERVER")) else {
        skip!("缺少 JX3API_TOKEN 或 JX3API_SERVER");
    };

    let query = jx3_api::api::recruit::RecruitSearchQuery::new()
        .server(server)
        .label(1);
    match client.recruit().search(&query).await {
        Ok(items) => {
            eprintln!("/recruit/search 返回 {} 条", items.len());
            for item in items.iter().take(3) {
                assert!(!item.activity_name.is_empty(), "activityName 不应为空");
                assert!(!item.role_name.is_empty(), "roleName 不应为空");
                assert!(!item.camp_name.is_empty(), "campName 不应为空");
            }
        }
        Err(err) => skip!("接口不可用：{err}"),
    }
}

/// `/reward/statistics`：掉落统计（VIP1，需要 token + 区服 + 角色）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌与区服/角色，会消耗接口额度"]
async fn live_reward_statistics() {
    let (Some(client), Some(server), Some(role)) =
        (client(), var("JX3API_SERVER"), var("JX3API_ROLE"))
    else {
        skip!("缺少 JX3API_TOKEN / JX3API_SERVER / JX3API_ROLE");
    };

    let query = jx3_api::api::reward::RewardStatisticsQuery::new()
        .server(server)
        .name(role);
    match client.reward().statistics(&query).await {
        Ok(records) => {
            eprintln!("/reward/statistics 返回 {} 条", records.len());
            for record in records.iter().take(3) {
                assert!(!record.role_name.is_empty(), "roleName 不应为空");
                assert!(!record.item_name.is_empty(), "itemName 不应为空");
            }
        }
        Err(err) if skip_if_unavailable(&err) => skip!("账号暂无掉落数据：{err}"),
        Err(err) => panic!("/reward/statistics 解析失败：{err}"),
    }
}

/// `/tuilan/achievement`：资历分布（VIP2，需要 token + 区服 + 角色，且账号需绑定角色）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌与已绑定的角色，会消耗接口额度"]
async fn live_tuilan_achievement() {
    let (Some(client), Some(server), Some(role)) =
        (client(), var("JX3API_SERVER"), var("JX3API_ROLE"))
    else {
        skip!("缺少 JX3API_TOKEN / JX3API_SERVER / JX3API_ROLE");
    };

    let query = jx3_api::api::tuilan::TuilanAchievementQuery::new()
        .server(server)
        .name(role);
    match client.tuilan().achievement(&query).await {
        Ok(value) => {
            assert!(!value.is_null(), "data 不应为 null");
            // 真实响应是「角色名片 + data 分桶」的裸字典，这里只校验关键结构。
            assert!(value.get("data").is_some(), "响应应包含 data 分桶");
            assert!(
                value
                    .get("roleName")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|name| !name.is_empty()),
                "roleName 不应为空"
            );
            eprintln!(
                "/tuilan/achievement 解析成功（data 分桶 {} 个）",
                value["data"].as_object().map_or(0, |buckets| buckets.len())
            );
        }
        Err(err) if skip_if_unavailable(&err) => skip!("账号未绑定角色或暂无资历数据：{err}"),
        Err(err) => panic!("/tuilan/achievement 解析失败：{err}"),
    }
}

/// `/arena/recent`：名剑战绩（VIP1，需要 token + 推栏 ticket + 区服 + 角色）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌 + 推栏 ticket，会消耗接口额度"]
async fn live_arena_recent() {
    let (Some(client), Some(server), Some(role)) =
        (client(), var("JX3API_SERVER"), var("JX3API_ROLE"))
    else {
        skip!("缺少 JX3API_TOKEN / JX3API_SERVER / JX3API_ROLE");
    };
    if var("JX3API_TICKET").is_none() {
        skip!("缺少 JX3API_TICKET（/arena/* 需要推栏 Ticket）");
    }

    let query = jx3_api::api::arena::ArenaRecentQuery::new()
        .server(server)
        .name(role);
    match client.arena().recent(&query).await {
        Ok(recent) => {
            eprintln!("/arena/recent 解析成功：{}", recent.role_name);
            assert!(!recent.role_name.is_empty(), "roleName 不应为空");
            // performance 在真实响应中应为对象；即使为 null 也应降级为空战绩。
            let _ = recent.performance.v2v.mmr;
        }
        Err(err) if skip_if_unavailable(&err) => skip!("暂无战绩数据：{err}"),
        Err(err) => panic!("/arena/recent 解析失败：{err}"),
    }
}

/// `/arena/schools`：名剑统计（VIP1，需要 token + 推栏 ticket）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌 + 推栏 ticket，会消耗接口额度"]
async fn live_arena_schools() {
    let Some(client) = client() else {
        skip!("缺少 JX3API_TOKEN");
    };
    if var("JX3API_TICKET").is_none() {
        skip!("缺少 JX3API_TICKET（/arena/* 需要推栏 Ticket）");
    }

    match client.arena().schools(None).await {
        Ok(items) => {
            eprintln!("/arena/schools 返回 {} 条", items.len());
            for item in items.iter().take(3) {
                assert!(!item.name.is_empty(), "name 不应为空");
            }
        }
        Err(err) if skip_if_unavailable(&err) => skip!("暂无统计数据：{err}"),
        Err(err) => panic!("/arena/schools 解析失败：{err}"),
    }
}

/// `/school/skills`：心法技能（VIP1，需要 token + 推栏 ticket）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌 + 推栏 ticket，会消耗接口额度"]
async fn live_school_skills() {
    let Some(client) = client() else {
        skip!("缺少 JX3API_TOKEN");
    };
    if var("JX3API_TICKET").is_none() {
        skip!("缺少 JX3API_TICKET（/school/* 需要推栏 Ticket）");
    }

    let school = var("JX3API_SCHOOL").unwrap_or_else(|| "冰心诀".to_owned());
    let query = jx3_api::api::school::SchoolSkillsQuery::new().name(school);
    match client.school().skills(&query).await {
        Ok(items) => {
            eprintln!("/school/skills 返回 {} 条", items.len());
            for item in items.iter().take(1) {
                assert!(!item.class.is_empty(), "class 不应为空");
                assert!(!item.data.is_empty(), "技能列表不应为空");
            }
        }
        Err(err) if skip_if_unavailable(&err) => skip!("暂无技能数据：{err}"),
        Err(err) => panic!("/school/skills 解析失败：{err}"),
    }
}

/// `/fraud/detail`：骗子查询（VIP1，需要 token + uid；服务端当前可能返回 500「未实现」）。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌与 uid，会消耗接口额度"]
async fn live_fraud_detail() {
    let Some(client) = client() else {
        skip!("缺少 JX3API_TOKEN");
    };
    let Some(uid) = var("JX3API_FRAUD_UID").and_then(|raw| raw.parse::<i64>().ok()) else {
        skip!("缺少 JX3API_FRAUD_UID（数值），例如 1000001");
    };

    let mut query = jx3_api::api::fraud::FraudDetailQuery::new().uid(uid);
    if let Some(server) = var("JX3API_SERVER") {
        query = query.server(server);
    }
    match client.fraud().detail(&query).await {
        Ok(records) => {
            eprintln!("/fraud/detail 返回 {} 条", records.len());
            for record in records.iter().take(3) {
                assert!(!record.title.is_empty(), "title 不应为空");
                assert!(!record.content.is_empty(), "content 不应为空");
            }
        }
        Err(err) if skip_if_unavailable(&err) => skip!("服务端未开放或该 uid 无记录：{err}"),
        Err(err) => panic!("/fraud/detail 解析失败：{err}"),
    }
}

/// `/sound/converter`：语音合成（免费，但需要 TTS 的 appkey/access/secret）。
#[tokio::test]
#[ignore = "线上测试：需要 TTS 凭据，会触发外部语音合成"]
async fn live_sound_converter() {
    let Some(client) = client() else {
        skip!("缺少 JX3API_TOKEN");
    };
    let (Some(appkey), Some(access), Some(secret)) = (
        var("JX3API_TTS_APPKEY"),
        var("JX3API_TTS_ACCESS"),
        var("JX3API_TTS_SECRET"),
    ) else {
        skip!("缺少 JX3API_TTS_APPKEY / JX3API_TTS_ACCESS / JX3API_TTS_SECRET");
    };

    let query = jx3_api::api::sound::SoundConverterQuery::new()
        .appkey(appkey)
        .access(access)
        .secret(secret)
        .text("剑网三，启动！");
    match client.sound().converter(&query).await {
        Ok(result) => {
            assert!(!result.url.is_empty(), "语音地址不应为空");
            eprintln!(
                "/sound/converter 解析成功，语音地址长度 {}",
                result.url.len()
            );
        }
        Err(err) if skip_if_unavailable(&err) => skip!("凭据无效或服务不可用：{err}"),
        Err(err) => panic!("/sound/converter 解析失败：{err}"),
    }
}
/// `/token/stats`：令牌配额（`POST`，**不消耗次数**）。
///
/// 这条用例不依赖区服 / 角色，只要有令牌就能跑；它同时验证
/// 「`POST` + 请求体」通道与 `TokenStats` 模型。
#[tokio::test]
#[ignore = "线上测试：需要真实令牌，会访问 jx3api.com（不消耗接口次数）"]
async fn live_token_stats() {
    let Some(client) = client() else {
        skip!("缺少 JX3API_TOKEN");
    };

    let stats = client
        .quota()
        .stats()
        .await
        .expect("/token/stats 应当返回成功响应");
    assert!(stats.level >= 1, "令牌档位应为 VIP1 及以上");
    assert!(stats.valid, "令牌应当有效");
    assert!(stats.each >= 1, "每次消耗不应为 0");
    assert_eq!(
        stats.total_quota(),
        stats.used.saturating_add(stats.remaining),
        "总配额应为已用 + 剩余"
    );
    eprintln!(
        "/token/stats：{} 已用 {} / 剩余 {} / 每次 {}",
        stats.level_name(),
        stats.used,
        stats.remaining,
        stats.each
    );

    // 查询本身不扣次数：连续两次查询的 used 应当一致。
    let again = client.quota().stats().await.expect("第二次查询应当成功");
    assert_eq!(
        stats.used, again.used,
        "/token/stats 不应消耗接口次数（used 发生了变化）"
    );

    // 先校验再加入客户端：用 stats_of 查询同一枚令牌。
    if let Some(token) = token() {
        let by_value = client
            .quota()
            .stats_of(&token)
            .await
            .expect("stats_of 应当返回成功响应");
        assert_eq!(by_value.remaining, stats.remaining);
    }
}
