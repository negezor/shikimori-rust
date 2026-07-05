use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("http request failed: {}", .0)]
    HttpError(reqwest::Error),
    #[error("error urlencoded serialize: {}", .0)]
    UrlencodedSerializeError(comma_serde_urlencoded::ser::Error),
    #[error("box error: {}", .0)]
    BoxError(Box<dyn std::error::Error + Send + Sync>),
    #[error("failed parse json {0} {1}")]
    Json(serde_json::Error, String),

    #[error("shikimori error: {}, code: {:?}", .message, .code)]
    ShikimoriError { message: String, code: Option<u32> },
}
