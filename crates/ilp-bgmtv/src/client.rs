use std::ops::{Deref, DerefMut};

use reqwest::header::*;
use url::Url;

use crate::{schemas::*, GetSubjectByIdError};

const API_BASE_URL: &str = "https://api.bgm.tv/v0/";

const DEFAULT_USER_AGENT: &str = concat!(
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
    pub fn new() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        Self {
            inner: reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .default_headers(default_headers)
                .build()
                .unwrap(),

            base_url: Url::parse(API_BASE_URL).unwrap(),
        }
    }
}

// 条目
impl Client {
    pub async fn get_subject_by_id(&self, id: SubjectId) -> Result<Subject, GetSubjectByIdError> {
        let url = self.base_url.join(&format!("subjects/{}", id))?;

        let res = self.get(url).send().await?;

        let status = res.status();

        if status.is_success() {
            let subject = res.json().await?;

            Ok(subject)
        } else {
            Err(GetSubjectByIdError::HttpStatus { status })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_subject_by_id() -> anyhow::Result<()> {
        let client = Client::new();

        let subject = client.get_subject_by_id(3559).await?;

        assert_eq!(subject.name, "とある魔術の禁書目録");
        assert_eq!(subject.r#type, SubjectType::Book);

        Ok(())
    }
}
