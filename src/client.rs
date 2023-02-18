mod auth;

use std::{borrow::Cow, ops::Deref, sync::Arc};

use tokio::sync::RwLock;

use crate::{Error, Result};

//use leaky_bucket_lite::LeakyBucket;

static BASE_URL: &str = "https://api.easee.cloud";

#[derive(Debug, thiserror::Error)]
pub enum FromEnvError {
    // @TODO: Actually parse this
    #[error("env variable {0} not set")]
    Missing(&'static str),

    #[error("invalid access token: {0}")]
    InvalidAccessToken(#[from] auth::ParseError),
}

#[derive(Clone)]
enum AuthState {
    Authenticated {
        session: auth::Session,
        refresh_token: String,
    },
    Unauthenticated,
}

pub struct AccessToken(pub String);
pub struct RefreshToken(pub String);

pub struct Client {
    c: reqwest::Client,
    base_url: Cow<'static, str>,
    auth_state: Arc<RwLock<AuthState>>,
}

impl Client {
    pub fn authenticated(
        access_token: &str,
        refresh_token: RefreshToken,
    ) -> std::result::Result<Self, auth::ParseError> {
        let session = access_token.parse()?;

        Ok(Self::new(AuthState::Authenticated {
            session,
            refresh_token: refresh_token.0,
        }))
    }

    pub fn unauthenticated() -> Self {
        Self::new(AuthState::Unauthenticated)
    }

    pub fn from_env() -> std::result::Result<Self, FromEnvError> {
        let access_token = std::env::var("EASEE_ACCESS_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_ACCESS_TOKEN"))?;
        let refresh_token = std::env::var("EASEE_REFRESH_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_REFRESH_TOKEN"))?;

        let session = access_token.parse::<auth::Session>()?;

        Ok(Self::new(AuthState::Authenticated {
            session,
            refresh_token,
        }))
    }

    fn new(auth_state: AuthState) -> Self {
        let c = reqwest::ClientBuilder::new()
            .user_agent("easee-rs")
            .build()
            .unwrap(); // Unwrap OK due to Infallible

        Self {
            c,
            base_url: Cow::Borrowed(BASE_URL),
            auth_state: Arc::new(RwLock::new(auth_state)),
        }
    }

    pub(crate) async fn req<B, R>(&self, method: http::Method, path: &str, body: B) -> Result<R>
    where
        B: Body,
        R: serde::de::DeserializeOwned,
    {
        let (access_token, _refresh_token) = self.tokens().await?;

        let b = self
            .c
            .request(
                method,
                format!("{}/{}", &self.base_url, path.trim_start_matches('/')),
            )
            .header("authorization", format!("Bearer {access_token}"));

        let res = body.apply_to(b).send().await?;

        let status = res.status();

        if !status.is_success() {
            let err = match status {
                http::StatusCode::NOT_FOUND => Error::NotFound,
                status => Error::Failed(status),
            };

            return Err(err);
        }

        let text_body = res.text().await?;

        match serde_json::from_str::<R>(&text_body) {
            Ok(res) => Ok(res),

            Err(err) => Err(Error::Deserializing {
                err,
                body: text_body,
            }),
        }
    }

    async fn tokens(&self) -> Result<(String, String)> {
        let state = self.auth_state.read().await;
        match state.deref() {
            AuthState::Authenticated {
                session,
                refresh_token,
            } => Ok((session.raw.clone(), refresh_token.clone())),
            AuthState::Unauthenticated => Err(Error::Unauthenticated),
        }
    }
}

pub(crate) struct JsonBody<'a, T>(&'a T);
pub(crate) struct NoBody;

pub(crate) trait Body {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

impl<'a, T> Body for JsonBody<'a, T>
where
    T: serde::Serialize,
{
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.json(self.0)
    }
}

impl Body for NoBody {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
    }
}
