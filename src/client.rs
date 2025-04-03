pub mod auth;
mod body;
mod credentials;

pub(crate) use body::*;

use credentials::GetJwt;
use http::Method;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{models, Error, Result};
pub use {auth::Session, credentials::Credentials};

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

#[derive(Clone)]
pub struct Client {
    c: reqwest::Client,
    credentials: Arc<Mutex<Credentials>>,
}

fn build_http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .user_agent("easee-rs")
        .build()
        .unwrap()
}

impl Client {
    pub fn new(
        access_token: impl AsRef<str>,
        refresh_token: impl Into<String>,
    ) -> std::result::Result<Self, auth::ParseError> {
        let session = access_token.as_ref().parse()?;

        let credentials = Credentials {
            session,
            refresh_token: refresh_token.into(),
        };

        Ok(Client {
            c: build_http_client(),
            credentials: Arc::new(Mutex::new(credentials)),
        })
    }

    pub fn from_env() -> std::result::Result<Self, FromEnvError> {
        let access_token = std::env::var("EASEE_ACCESS_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_ACCESS_TOKEN"))?;
        let refresh_token = std::env::var("EASEE_REFRESH_TOKEN")
            .map_err(|_| FromEnvError::Missing("EASEE_REFRESH_TOKEN"))?;

        Self::new(&access_token, refresh_token).map_err(FromEnvError::InvalidAccessToken)
    }

    pub async fn login(username: impl AsRef<str>, password: impl AsRef<str>) -> Result<Self> {
        let c = build_http_client();

        let builder = JsonBody(&LoginRequest {
            username: username.as_ref(),
            password: password.as_ref(),
        })
        .apply_to(c.post(format!("{BASE_URL}/api/accounts/login",)));

        let res = send_and_handle_response::<JsonBody<models::RawSession>>(builder).await?;

        let credentials = Credentials {
            session: res.access_token.parse::<Session>()?,
            refresh_token: res.refresh_token,
        };

        Ok(Self {
            c,
            credentials: Arc::new(Mutex::new(credentials)),
        })
    }

    async fn get_token(&self) -> Result<String> {
        let mut credentials = self.credentials.lock().await;

        let (token, refresh_token) = match credentials.get_valid_session().await {
            GetJwt::Valid { token } => return Ok(token.into()),
            GetJwt::Expired {
                token,
                refresh_token,
            } => (token, refresh_token),
        };

        let res = self
            .inner_req::<_, JsonBody<models::RawSession>>(
                Method::POST,
                "/api/accounts/refresh_token",
                token,
                JsonBody(&RefreshSessionTokenRequest {
                    access_token: token,
                    refresh_token,
                }),
            )
            .await?;

        credentials.session = res.access_token.parse::<Session>()?;
        credentials.refresh_token = res.refresh_token;

        Ok(res.access_token)
    }

    /// Forces refresh of the current session
    pub async fn refresh_session(&self) -> Result<()> {
        let mut credentials = self.credentials.lock().await;
        let (token, refresh_token) = credentials.get_tokens();

        let res = self
            .inner_req::<_, JsonBody<models::RawSession>>(
                Method::POST,
                "/api/accounts/refresh_token",
                &token,
                JsonBody(RefreshSessionTokenRequest {
                    access_token: &token,
                    refresh_token: &refresh_token,
                }),
            )
            .await?;

        credentials.session = res.access_token.parse::<Session>()?;
        credentials.refresh_token = res.refresh_token;

        Ok(())
    }

    /// Returns a copy of the current Credentials
    pub async fn get_credentials(&self) -> Credentials {
        let lock = self.credentials.lock().await;

        Credentials::clone(&lock)
    }

    pub async fn get_credentials_issued_at(&self) -> models::DateTime {
        let lock = self.credentials.lock().await;
        lock.session.issued_at
    }

    async fn inner_req<Req, Rep>(
        &self,
        method: http::Method,
        path: &str,
        access_token: &str,
        body: Req,
    ) -> Result<Rep::Data>
    where
        Req: RequestBody,
        Rep: ResponseBody,
    {
        let b = self
            .c
            .request(
                method,
                format!("{}/{}", BASE_URL, path.trim_start_matches('/')),
            )
            .header("authorization", format!("Bearer {access_token}"));

        send_and_handle_response::<Rep>(body.apply_to(b)).await
    }

    pub(crate) async fn req<Req, Rep>(
        &self,
        method: http::Method,
        path: &str,
        body: Req,
    ) -> Result<Rep::Data>
    where
        Req: RequestBody,
        Rep: ResponseBody,
    {
        let access_token = self.get_token().await?;
        self.inner_req::<Req, Rep>(method, path, &access_token, body)
            .await
    }
}

async fn send_and_handle_response<Rep>(builder: reqwest::RequestBuilder) -> Result<Rep::Data>
where
    Rep: ResponseBody,
{
    let res = builder.send().await?;

    let status = res.status();

    if !status.is_success() {
        let err = match status {
            http::StatusCode::NOT_FOUND => Error::NotFound,

            status => {
                let body = res.text().await?;
                let details = serde_json::from_str::<ApiError>(&body)
                    .map_err(|err| Error::DeserializingErrorReply { err, body })?;

                Error::Api {
                    code: details.error_code,
                    code_name: details.error_code_name,
                    title: details.title,
                    http_status: status.as_u16(),
                }
            }
        };

        return Err(err);
    }

    let body = res.bytes().await?;

    Rep::from_body(body)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshSessionTokenRequest<'a> {
    access_token: &'a str,
    refresh_token: &'a str,
}

#[derive(serde::Serialize)]
struct LoginRequest<'a> {
    #[serde(rename = "userName")]
    pub username: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiError {
    pub error_code: u32,
    pub error_code_name: String,
    pub title: String,
}
