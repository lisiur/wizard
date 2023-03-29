use diesel::prelude::*;
use std::str::FromStr;

use crate::api::client::Client;
use crate::api::openai::chat::OpenAIChatApi;
use crate::schema::settings;
use crate::types::{Id, TextWrapper};

#[derive(Queryable)]
pub struct Setting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

impl Setting {
    pub fn create_openai_chat(&self) -> OpenAIChatApi {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(api_key) = self.api_key.as_deref() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {api_key}").parse().unwrap(),
            );
        }
        if self.forward_url.is_some() && !self.forward_api_key {
            headers.remove(reqwest::header::AUTHORIZATION);
        }

        let proxy = self
            .proxy
            .as_ref()
            .map(|item| reqwest::Proxy::all(item).unwrap());

        let mut client = Client::new();
        client.headers(Some(headers));
        client.proxy(proxy);

        let host = self
            .forward_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com".to_string());

        OpenAIChatApi::new(client, host)
    }
}

#[derive(Debug)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl AsRef<str> for Theme {
    fn as_ref(&self) -> &str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err("Invalid theme".into()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = settings)]
pub struct PatchSetting {
    pub user_id: Id,
    pub language: Option<String>,
    pub theme: Option<TextWrapper<Theme>>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}
