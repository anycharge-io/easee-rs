/// The raw response received from Login and RefreshToken
#[allow(dead_code)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawSession {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: String,
}
