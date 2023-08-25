// GET :base_url/api/accounts/profile

use crate::{Client, NoBody, Profile, Result, StateAuthenticated};

pub struct GetProfile;

impl GetProfile {
    pub async fn send(&self, client: Client<StateAuthenticated>) -> Result<Profile> {
        client
            .req::<_, Profile>(http::Method::GET, "api/accounts/profile", NoBody)
            .await
    }
}
