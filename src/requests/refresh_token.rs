use crate::{Client, JsonBody, NewSession, RefreshToken, Result, Session, StateAuthenticated};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshSessionToken {
    pub access_token: String,
    pub refresh_token: String,
}

impl RefreshSessionToken {
    pub fn new(access_token: String, refresh_token: RefreshToken) -> Self {
        Self {
            access_token,
            refresh_token: refresh_token.0,
        }
    }

    pub async fn send(
        &self,
        client: &Client<StateAuthenticated>,
    ) -> Result<(Session, RefreshToken)> {
        let res = client
            .req::<_, NewSession>(
                http::Method::POST,
                "/api/accounts/refresh_token",
                JsonBody(self),
            )
            .await?;

        let session = res.access_token.parse::<Session>()?;

        Ok((session, RefreshToken(res.refresh_token)))
    }
}
