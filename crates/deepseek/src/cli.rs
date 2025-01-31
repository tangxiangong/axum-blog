// use reqwest::{Client, RequestBuilder};

use crate::chat::{ChatBuilder, Model};

const BASE_URL: &str = "https://api.deepseek.com";

#[derive(Debug, Clone)]
pub struct DeepSeek {
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    // pub cli: RequestBuilder,
}

impl Default for DeepSeek {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.into(),
            api_key: "".into(),
        }
    }
}

impl DeepSeek {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    pub fn from_env() -> Self {
        let api_key = std::env::var("DEEPSEEK_API_KEY").expect("API KEY 未在环境变量中设置");
        let base_url = std::env::var("DEEPSEEK_BASE_URL").unwrap_or_else(|_| BASE_URL.into());
        Self { base_url, api_key }
    }

    pub fn model(&self, m: Model) -> ChatBuilder {
        ChatBuilder::default().cli(self.clone()).model(m).clone()
    }
}
