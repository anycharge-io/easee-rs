#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSession {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: String,
}
