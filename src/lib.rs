pub mod api;
pub mod enums;
pub mod err;
pub mod model;

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Debug, Clone)]
pub struct Jx3ApiClient {
    client: Client,
    base_url: String,
    pub token: Option<String>,
}

impl Jx3ApiClient {
    pub fn new() -> Self {
        let client = Client::builder().build().expect("客户端初始化失败！");
        Self {
            client,
            token: None,
            base_url: String::from(api::DEFAULT_BASE_URL),
        }
    }

    pub fn from_token(token: &str) -> Self {
        let mut header_map = HeaderMap::new();
        let token_value = HeaderValue::from_str(token).expect("设置token失败！");
        header_map.insert(api::HEADER_TOKEN_KEY, token_value);
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .expect("客户端初始化失败！");
        Self {
            client,
            token: Some(token.to_string()),
            base_url: String::from(api::DEFAULT_BASE_URL),
        }
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.to_string();
    }
}
