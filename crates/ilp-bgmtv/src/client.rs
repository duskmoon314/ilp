use std::ops::{Deref, DerefMut};

use reqwest::header::*;
use url::Url;

#[cfg(feature = "v0")]
pub mod v0;

const API_BASE_URL: &str = "https://api.bgm.tv/";

pub(crate) const DEFAULT_USER_AGENT: &str = concat!(
    "duskmoon/ilp-bgmtv/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/duskmoon314/ilp)"
);

#[derive(Debug)]
pub struct Client {
    inner: reqwest::Client,

    base_url: Url,
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Client {
    pub fn new(user_agent: Option<&str>) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        Self {
            inner: reqwest::Client::builder()
                .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT))
                .default_headers(default_headers)
                .build()
                .unwrap(),

            base_url: Url::parse(API_BASE_URL).unwrap(),
        }
    }
}
