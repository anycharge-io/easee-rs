pub mod auth;

use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;

use crate::{requests, Error, Result};
pub use auth::Session;

//use leaky_bucket_lite::LeakyBucket;

static BASE_URL: &str = "https://api.easee.com";

#[derive(Debug, thiserror::Error)]
pub enum FromEnvError {
    // @TODO: Actually parse this
    #[error("env variable {0} not set")]
    Missing(&'static str),

    #[error("invalid access token: {0}")]
    InvalidAccessToken(#[from] auth::ParseError),
}

trait AuthState {}

pub struct StateAuthenticated {
    session: auth::Session,
    refresh_token: RefreshToken,
}

pub struct StateUnauthenticated;

impl AuthState for StateAuthenticated {}

impl AuthState for StateUnauthenticated {}

#[derive(Clone)]
pub struct AccessToken(pub String);

#[derive(Clone, Debug)]
pub struct RefreshToken(pub String);

#[derive(Clone)]
pub struct Client<T>
where
    T: Send + Sync + 'static,
{
    c: reqwest::Client,
    base_url: Cow<'static, str>,
    auth_state: Arc<Mutex<T>>,
}

impl Client<StateAuthenticated> {
    pub fn authenticated(
        access_token: &str,
        refresh_token: RefreshToken,
    ) -> std::result::Result<Client<StateAuthenticated>, auth::ParseError> {
        let session = access_token.parse()?;

        Ok(Client {
            c: reqwest::ClientBuilder::new()
                .user_agent("easee-rs")
                .build()
                .unwrap(), // Unwrap OK due to Infallible
            base_url: Cow::Borrowed(BASE_URL),
            auth_state: Arc::new(Mutex::new(StateAuthenticated {
                session,
                refresh_token,
            })),
        })
    }

    pub fn from_env() -> std::result::Result<Client<StateAuthenticated>, FromEnvError> {
        let access_token = std::env::var("EASEE_ACCESS_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_ACCESS_TOKEN"))?;
        let refresh_token = std::env::var("EASEE_REFRESH_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_REFRESH_TOKEN"))?;

        Self::authenticated(&access_token, RefreshToken(refresh_token))
            .map_err(FromEnvError::InvalidAccessToken)
    }

    /// Attempts to refresh the active session. Applies and returns the newly fetched tokens.
    pub async fn refresh_auth(&mut self) -> Result<(auth::Session, RefreshToken)> {
        // Important to release lock before sending refresh.
        let (session, refresh_token) = {
            let state = self.auth_state.lock().await;
            (state.session.clone(), state.refresh_token.clone())
        };

        let (new_session, new_refresh) =
            requests::RefreshSessionToken::new(session.raw.clone(), refresh_token.clone())
                .send(self)
                .await?;

        {
            let mut state = self.auth_state.lock().await;
            state.refresh_token = new_refresh.clone();
            state.session = new_session.clone();
        }

        Ok((new_session, new_refresh))
    }

    pub async fn get_auth_session(&self) -> (auth::Session, RefreshToken) {
        let state = self.auth_state.lock().await;
        (state.session.clone(), state.refresh_token.clone())
    }

    pub(crate) async fn req<B, R>(&self, method: http::Method, path: &str, body: B) -> Result<R>
    where
        B: Body,
        R: serde::de::DeserializeOwned,
    {
        let state = self.auth_state.lock().await;
        let access_token = &state.session.raw;

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
                http::StatusCode::UNAUTHORIZED => Error::CredentialsExpired,
                status => Error::Failed(status.as_u16()),
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
}

impl Client<StateUnauthenticated> {
    pub fn unauthenticated() -> Client<StateUnauthenticated> {
        Client {
            c: reqwest::ClientBuilder::new()
                .user_agent("easee-rs")
                .build()
                .unwrap(), // Unwrap OK due to Infallible
            base_url: Cow::Borrowed(BASE_URL),
            auth_state: Arc::new(Mutex::new(StateUnauthenticated)),
        }
    }

    pub async fn login(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Client<StateAuthenticated>> {
        let (session, refresh_token) = requests::Login::new(username, password).send(self).await?;

        Ok(Client {
            c: self.c.clone(),
            base_url: self.base_url.clone(),
            auth_state: Arc::new(Mutex::new(StateAuthenticated {
                session,
                refresh_token,
            })),
        })
    }

    pub(crate) async fn req<B, R>(&self, method: http::Method, path: &str, body: B) -> Result<R>
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
                status => Error::Failed(status.as_u16()),
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
