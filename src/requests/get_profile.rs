// GET :base_url/api/accounts/profile

use crate::{Client, NoBody, Profile, Result};

pub struct GetProfile;

impl GetProfile {
    pub async fn send(&self, client: Client) -> Result<Profile> {
        client
            .req::<_, Profile>(http::Method::GET, "api/accounts/profile", NoBody)
            .await
    }
}
