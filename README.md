# jx3api-rs

[剑侠三API](https://www.jx3api.com/)的Rust SDK

# 路线图

- [x] 免费
- [ ] vip1
- [ ] vip2
- [ ] WebSocket

# 安装

## Cargo

```bash
cargo add jx3-api
```

## Github

Cargo.toml添加github仓库地址

```toml
jx3-api = { git = "https://github.com/Natane0808/jx3api-rs" }
```

# 快速开始

```rust

use jx3_api::Jx3ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Jx3ApiClient::new();
    let resp = client.calendar(None, None).await?;
    println!("resp:{:#?}", resp);
    Ok(())
}

```