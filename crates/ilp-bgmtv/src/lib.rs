use error_set::error_set;

pub mod client;
pub mod schemas;

error_set! {
    GetSubjectByIdError = {
        Reqwest(reqwest::Error),
        UrlParse(url::ParseError),
        #[display("HTTP not success, receive: {status}")]
        HttpStatus {
            status: reqwest::StatusCode,
        }
    };
}
