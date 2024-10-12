use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetSubjectError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
