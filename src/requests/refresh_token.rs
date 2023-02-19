use crate::{Client, JsonBody, RefreshToken, Result, Session};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshSession {
    pub access_token: String,
    pub refresh_token: String,
}

impl RefreshSession {
    pub fn new(access_token: String, refresh_token: RefreshToken) -> Self {
        Self {
            access_token,
            refresh_token: refresh_token.0,
        }
    }

    pub async fn send(&self, client: &Client) -> Result<(Session, RefreshToken)> {
        let res = client
            .req_no_auth::<_, Reply>(
                http::Method::POST,
                "/api/accounts/refresh_token",
                JsonBody(self),
            )
            .await?;

        let session = self.access_token.parse::<Session>()?;

        Ok((session, RefreshToken(res.refresh_token)))
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Reply {
    access_token: String,
    expires_in: u64,
    access_claims: Vec<String>,
    token_type: String,
    refresh_token: String,
}
