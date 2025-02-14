mod client;
pub mod from_str;
mod models;
pub mod requests;

pub use {client::*, models::*};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,

    #[error("status:{http_status:03} code:{code:03}({code_name}): {title}")]
    Api {
        code: u32,
        code_name: String,
        title: String,
        http_status: u16,
    },

    #[error("Error deserializing reply: {err}. Body: {body}")]
    Deserializing {
        err: serde_json::Error,
        body: String,
    },

    #[error("invalid access token: {0}")]
    AccessTokenParse(#[from] client::auth::ParseError),
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
