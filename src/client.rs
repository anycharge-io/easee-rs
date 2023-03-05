pub mod auth;

use std::{borrow::Cow, ops::Deref, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

use crate::{requests, Error, Result};
pub use auth::Session;

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

#[derive(Clone, Debug)]
enum AuthState {
    Authenticated {
        session: auth::Session,
        refresh_token: RefreshToken,
    },
    Unauthenticated,
}

#[derive(Clone)]
pub struct AccessToken(pub String);

#[derive(Clone, Debug)]
pub struct RefreshToken(pub String);

#[derive(Clone)]
pub struct Client {
    c: reqwest::Client,
    base_url: Cow<'static, str>,
    auth_state: Arc<Mutex<AuthState>>,
}

impl Client {
    pub fn authenticated(
        access_token: &str,
        refresh_token: RefreshToken,
    ) -> std::result::Result<Self, auth::ParseError> {
        let session = access_token.parse()?;

        Ok(Self::new(AuthState::Authenticated {
            session,
            refresh_token,
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
            refresh_token: RefreshToken(refresh_token),
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
            auth_state: Arc::new(Mutex::new(auth_state)),
        }
    }

    pub(crate) async fn req<B, R>(&self, method: http::Method, path: &str, body: B) -> Result<R>
    where
        B: Body,
        R: serde::de::DeserializeOwned,
    {
        let access_token = self.access_token().await?;

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

    pub(crate) async fn req_no_auth<B, R>(
        &self,
        method: http::Method,
        path: &str,
        body: B,
    ) -> Result<R>
    where
        B: Body,
        R: serde::de::DeserializeOwned,
    {
        let b = self.c.request(
            method,
            format!("{}/{}", &self.base_url, path.trim_start_matches('/')),
        );

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

    async fn access_token(&self) -> Result<String> {
        let mut state = self.auth_state.lock().await;

        info!("getting access token from {:?}", state.deref());

        match state.deref() {
            AuthState::Authenticated {
                session,
                refresh_token,
            } if session.is_expired() => {
                info!("Refreshing access token");
                match requests::RefreshSession::new(session.raw.clone(), refresh_token.clone())
                    .send(self)
                    .await
                {
                    Ok((session, refresh_token)) => {
                        let access_token = session.raw.clone();
                        info!(
                            "new access token: `{}`\nRefresh token: `{}`",
                            access_token, refresh_token.0
                        );
                        *state = AuthState::Authenticated {
                            session,
                            refresh_token,
                        };

                        Ok(access_token)
                    }

                    Err(err) => Err(Error::RefreshSessionFailed(Box::new(err))),
                }
            }

            AuthState::Authenticated { session, .. } => Ok(session.raw.clone()),

            AuthState::Unauthenticated => Err(Error::Unauthenticated),
        }
    }
}

pub(crate) struct JsonBody<'a, T>(pub &'a T);
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
