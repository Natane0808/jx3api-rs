mod api;
mod model;

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Jx3ApiClient {
    client: Client,
    pub token: Option<String>,
}

impl Jx3ApiClient {
    pub fn new() -> Self {
        let client = Client::builder().build().expect("客户端初始化失败！");
        Self {
            client,
            token: None,
        }
    }
    #[cfg(feature = "vip")]
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
        }
    }

    pub async fn calendar(
        &self,
        server: Option<String>,
        num: Option<u8>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let full_url = format!("{}{}", api::API_PREFIX, api::CALENDAR);
        let body = {
            let mut body_map = HashMap::new();
            if let Some(server) = server {
                body_map.insert("server", Value::String(server));
            }
            if let Some(num) = num {
                body_map.insert("num", Value::Number(num.into()));
            }
            body_map
        };

        let resp = self
            .client
            .post(full_url)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}

#[tokio::test]
pub async fn test_calendar() -> Result<(), Box<dyn std::error::Error>> {
    let client = Jx3ApiClient::new();
    let text = client
        .calendar(Some(String::from("长安城")), Some(2))
        .await?;
    println!("{}", text);
    Ok(())
}
