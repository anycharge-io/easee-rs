//# Login
//POST :base_url/api/accounts/login
//Accept: application/json
//Content-type: application/json
//
//{
//     "userName": ":username",
//     "password": ":password"
//}

use crate::{auth, Client, JsonBody, NewSession, RefreshToken, Result, StateUnauthenticated};

#[derive(serde::Serialize)]
pub struct Login {
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl Login {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub async fn send(
        &self,
        client: &Client<StateUnauthenticated>,
    ) -> Result<(auth::Session, RefreshToken)> {
        let res = client
            .req::<_, NewSession>(http::Method::POST, , JsonBody(self))
            .await?;
        let session = res.access_token.parse::<auth::Session>()?;

        Ok((session, RefreshToken(res.refresh_token)))
    }
}
