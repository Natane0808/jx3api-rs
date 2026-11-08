//! 集成测试。
//!
//! 分两部分：
//!
//! * **离线部分**（默认运行）：直接读取 `tests/fixtures/` 下的真实响应快照，
//!   只用 [`serde_json::Value`] 校验包络结构，不依赖任何业务域模型，因此不会与
//!   并行开发中的 `src/models/*` 产生编译耦合，也不消耗 API 调用次数。
//! * **线上部分**（`#[ignore]`）：读取环境变量 `JX3API_TOKEN` 后对免费接口做
//!   冒烟调用；未设置令牌时直接跳过。默认 `cargo test` 不会联网。
//!
//! ```text
//! cargo test --test integration                      # 只跑离线夹具校验
//! cargo test --test integration -- --ignored         # 额外跑线上冒烟（会消耗额度）
//! ```

/// 读取 `tests/fixtures/` 下的全部夹具（文件名 + 内容），按文件名排序。
///
/// 这里用运行期目录遍历而不是 `include_str!`，以保证新增/刷新夹具后无需改动测试代码
/// 就能自动纳入校验（夹具由 `scripts/probe_api.py` 生成）。
fn load_fixtures() -> Vec<(String, String)> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|err| panic!("无法读取夹具目录 {}: {err}", dir.display()))
        .map(|entry| entry.expect("读取夹具目录项失败").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort();

    assert!(
        entries.len() >= 80,
        "夹具数量异常偏少（{} 份），请确认 tests/fixtures/ 完整",
        entries.len()
    );

    entries
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .expect("夹具应有文件名")
                .to_string_lossy()
                .into_owned();
            let raw = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("读取夹具 {name} 失败: {err}"));
            (name, raw)
        })
        .collect()
}

/// 所有夹具都必须是合法的 JSON 对象，且包含服务端统一包络的 `code` 字段。
#[test]
fn fixtures_are_valid_envelopes() {
    let fixtures = load_fixtures();
    assert!(!fixtures.is_empty(), "至少应校验一份夹具");

    for (name, raw) in &fixtures {
        let value: serde_json::Value = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("夹具 {name} 不是合法 JSON: {err}"));

        assert!(
            value.is_object(),
            "夹具 {name} 的根节点应为 JSON 对象，实际为 {value:?}"
        );

        let code = value
            .get("code")
            .unwrap_or_else(|| panic!("夹具 {name} 缺少 `code` 字段"));
        assert!(
            code.is_i64() || code.is_u64(),
            "夹具 {name} 的 `code` 应为整数，实际为 {code:?}"
        );

        // `msg` 与 `time` 是官方包络的固定字段。
        assert!(
            value.get("msg").map(serde_json::Value::is_string) == Some(true),
            "夹具 {name} 缺少字符串类型的 `msg` 字段"
        );
        assert!(
            value.get("time").map(|time| time.is_i64() || time.is_u64()) == Some(true),
            "夹具 {name} 缺少整数类型的 `time` 字段"
        );

        // 成功响应必须带 `data` 键（允许为 null，但键要在）。
        let code_value = code.as_i64().unwrap_or_default();
        if code_value == 200 {
            assert!(
                value.get("data").is_some(),
                "夹具 {name} 的成功响应缺少 `data` 字段"
            );
        } else {
            assert!(
                value
                    .get("msg")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|msg| !msg.is_empty()),
                "夹具 {name} 的业务错误响应应带错误描述"
            );
        }
    }
}

/// 夹具集合应同时覆盖「成功」与「业务错误」两类包络，保证校验有意义。
#[test]
fn fixtures_cover_success_and_error_envelopes() {
    let fixtures = load_fixtures();
    let mut success = 0usize;
    let mut failure = 0usize;

    for (name, raw) in &fixtures {
        let value: serde_json::Value = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("夹具 {name} 不是合法 JSON: {err}"));
        match value.get("code").and_then(serde_json::Value::as_i64) {
            Some(200) => success += 1,
            Some(_) => failure += 1,
            None => panic!("夹具 {name} 缺少整数 `code` 字段"),
        }
    }

    assert!(success > 0, "夹具中应包含成功响应样本");
    assert!(failure > 0, "夹具中应包含业务错误响应样本");
    assert_eq!(
        success + failure,
        fixtures.len(),
        "每份夹具都应被归类为成功或业务错误"
    );
}

/// `Endpoint` 元数据应与 OpenAPI 的 82 条路径一一对应（不联网）。
#[test]
fn endpoint_metadata_covers_all_paths() {
    use std::collections::HashSet;

    use jx3_api::Endpoint;

    assert_eq!(Endpoint::ALL.len(), 82, "接口总数应为 82");

    let unique: HashSet<&str> = Endpoint::ALL
        .iter()
        .map(|endpoint| endpoint.path())
        .collect();
    assert_eq!(unique.len(), Endpoint::ALL.len(), "接口路径不应重复");

    for endpoint in Endpoint::ALL {
        assert!(
            endpoint.path().starts_with('/'),
            "接口路径应以 / 开头：{}",
            endpoint.path()
        );
        assert_eq!(
            Endpoint::from_path(endpoint.path()),
            Some(*endpoint),
            "按路径反查失败：{}",
            endpoint.path()
        );
        assert!(!endpoint.summary().is_empty(), "接口缺少中文名");
    }

    // 免费接口数量与 OpenAPI 的 x-level=0 一致。
    let free = Endpoint::ALL
        .iter()
        .filter(|endpoint| endpoint.is_free())
        .count();
    assert_eq!(free, 20, "免费接口数量应为 20");
}

/// 客户端在无令牌时也能构造（不会联网）。
#[test]
fn client_can_be_constructed_offline() {
    use jx3_api::Client;

    let client = Client::new().expect("构建 Client 失败");
    assert!(client.token().is_none());
    assert!(client.base_url().starts_with("https://"));
}

/// 读取环境变量中的令牌；未设置或为空时返回 `None`。
fn token_from_env() -> Option<String> {
    // 优先 JX3API_TOKEN，同时兼容常用的 API_TOKEN 环境变量。
    ["JX3API_TOKEN", "API_TOKEN"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .map(|token| token.trim().to_owned())
        .find(|token| !token.is_empty())
}

/// 线上冒烟：免费接口「活动日历」。
///
/// 需要设置 `JX3API_TOKEN`，并显式加 `--ignored` 才会执行；默认 `cargo test` 跳过。
#[tokio::test]
#[ignore = "需要 JX3API_TOKEN，且会消耗真实请求额度"]
async fn online_active_calendar_smoke() {
    let Some(token) = token_from_env() else {
        eprintln!("跳过：未设置 JX3API_TOKEN");
        return;
    };

    let client = jx3_api::Client::with_token(token).expect("构建 Client 失败");
    let today = client
        .active()
        .calendar(None)
        .await
        .expect("调用 /active/calendar 失败");

    assert!(!today.date.is_empty(), "活动日历应返回日期");
    assert!(!today.war.is_empty(), "活动日历应返回大战");
}

/// 线上冒烟：通用 `Client::request` 返回完整响应包络。
///
/// 用免费接口 `/active/celebs` 校验 `ApiResponse` 的 `code` / `msg` / `time`。
#[tokio::test]
#[ignore = "需要 JX3API_TOKEN，且会消耗真实请求额度"]
async fn online_api_response_envelope_smoke() {
    let Some(token) = token_from_env() else {
        eprintln!("跳过：未设置 JX3API_TOKEN");
        return;
    };

    let client = jx3_api::Client::with_token(token).expect("构建 Client 失败");
    let query = vec![("name", "楚天社")];
    let response = client
        .request::<serde_json::Value>("/active/celebs", &query)
        .await
        .expect("调用 /active/celebs 失败");

    assert!(
        response.is_success(),
        "业务码应为 200，实际 {}",
        response.code
    );
    assert!(!response.msg.is_empty(), "响应应带 msg");
    assert!(response.time > 0, "响应应带服务端时间戳");
}
