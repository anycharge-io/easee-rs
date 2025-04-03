// GET :base_url/api/accounts/profile

use crate::{Client, JsonBody, NoBody, Profile, Result};

pub struct GetProfile;

impl GetProfile {
    pub async fn send(&self, client: &Client) -> Result<Profile> {
        client
            .req::<_, JsonBody<Profile>>(http::Method::GET, "api/accounts/profile", NoBody)
            .await
    }
}
