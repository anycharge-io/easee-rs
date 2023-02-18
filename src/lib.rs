mod client;
mod models;
pub mod requests;

pub use {client::*, models::*};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Client has no Authentication credentials set. Login first")]
    Unauthenticated,

    #[error("not found")]
    NotFound,

    #[error("Request failed with status: {0}")]
    Failed(http::StatusCode),

    #[error("Error deserializing reply: {err}. Body: {body}")]
    Deserializing {
        err: serde_json::Error,
        body: String,
    },
}

pub trait OptionalResult<T> {
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalResult<T> for Result<T> {
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(res) => Ok(Some(res)),
            Err(Error::NotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
