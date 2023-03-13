mod chat;
mod client;
mod completion;
mod error;
mod image;
mod result;
mod types;

pub use chat::{StreamContent, Topic};
use client::{Client, ClientOpts};
pub use error::Error;
use error::{ApiError, ApiErrorResponse};
pub use result::Result;

pub struct OpenAIApi {
    client: Client,
}

impl OpenAIApi {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Self::create_client(api_key, None),
        }
    }

    fn create_client(api_key: &str, proxy: Option<String>) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {api_key}").parse().unwrap(),
        );

        let proxy = proxy.map(|item| reqwest::Proxy::all(item).unwrap());

        Client::new(ClientOpts {
            headers: Some(headers),
            proxy,
        })
    }

    pub async fn set_proxy(&self, proxy: &str) {
        if proxy.is_empty() {
            self.client.clear_proxy().await;
        } else {
            self.client
                .set_proxy(reqwest::Proxy::all(proxy).unwrap())
                .await;
        }
    }

    pub async fn clear_proxy(&self) {
        self.client.clear_proxy().await;
    }

    pub async fn check_api_key(api_key: &str) -> Result<()> {
        let client = Self::create_client(api_key, None);
        let response = client.get("https://api.openai.com/v1/models", None).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error: ApiErrorResponse = response.json().await?;
            Err(error.into())
        }
    }
}
