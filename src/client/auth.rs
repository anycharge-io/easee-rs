use crate::{models, DateTime};
use base64::{engine::general_purpose, Engine};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("malformed access token: {0}")]
    Malformed(&'static str),

    #[error("invalid base64: {0}")]
    InvalidB64(#[from] base64::DecodeError),

    #[error("invalid json: {0}")]
    InvalidJson(serde_json::Error),

    #[error("token json contained unexpected data: {0}")]
    UnexpectedData(serde_json::Error),

    #[error("could not create UTC time from {field}: {unix_ts}")]
    InvalidUtcTime { unix_ts: i64, field: &'static str },
}

#[derive(Debug, Clone)]
pub struct Session {
    pub raw: String,
    pub issued_at: models::DateTime,
    pub expires_at: models::DateTime,
    pub account_id: Option<String>,

    pub user_id: Option<String>,
    pub role: Vec<UserRole>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        dbg!(self.expires_at.0 < DateTime::now_utc().0)
    }
}

#[derive(Debug, serde::Deserialize)]
struct JsonAccessToken {
    #[serde(rename = "AccountId")]
    account_id: Option<String>,
    #[serde(rename = "UserId")]
    user_id: Option<String>,

    //unique_name: Option<String>,

    //nbf: i64,
    exp: i64,
    iat: i64,

    #[serde(rename = "role")]
    role: Vec<UserRole>,
}

impl FromStr for Session {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');
        let _header = split
            .next()
            .ok_or(ParseError::Malformed("missing header block"))?;
        let claims = split
            .next()
            .ok_or(ParseError::Malformed("missing claims block"))?;

        let bs = general_purpose::STANDARD_NO_PAD.decode(claims)?;
        let des = match serde_json::from_slice::<JsonAccessToken>(&bs) {
            Ok(res) => res,

            Err(err) if err.is_syntax() => return Err(ParseError::InvalidJson(err)),
            Err(err) => return Err(ParseError::UnexpectedData(err)),
        };

        let issued_at =
            DateTime::from_unix_timestamp(des.iat).map_err(|_| ParseError::InvalidUtcTime {
                unix_ts: des.iat,
                field: "iat",
            })?;

        let expires_at =
            DateTime::from_unix_timestamp(des.exp).map_err(|_| ParseError::InvalidUtcTime {
                unix_ts: des.exp,
                field: "exp",
            })?;

        Ok(Session {
            raw: s.into(),
            issued_at,
            expires_at,
            account_id: des.account_id,
            user_id: des.user_id,
            role: des.role,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum UserRole {
    User,

    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::Session;

    #[test]
    fn parse_exmaple_token_invalid_kind() {
        let token = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJlcVJKMDhGMTFBQi14QWhnNnNSbXdxSE5PdzllS09JbmJaSVpISmpoOVc4In0.eyJleHAiOjE2ODg2MDYwMTIsImlhdCI6MTY4ODYwMjQxMiwianRpIjoiNTgyOWY5NzYtN2E2OS00ZGMyLThkYWQtMzQ4ZjQyNmVkNGFjIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLmVhc2VlLmNvbS9yZWFsbXMvZWFzZWUiLCJhdWQiOlsiZWFzZWUtY2xvdWQiLCJlYXNlZS1kZXZpY2Utc2V0dGluZ3MiLCJlYXNlZS1hY2Nlc3MtY29udHJvbCIsImVhc2VlLXByaW1vcmRpYWwiLCJlYXNlZS1jaHJvbmljbGUiLCJlYXNlZS1jdXJyZW50LWRldmljZS1zdGF0ZSIsImVhc2VlLWNvbW1hbmRldXIiLCJlYXNlZSIsImFjY291bnQiXSwic3ViIjoiOWMxMjNhOWQtNDdiNS00NzQ4LWFhNTEtZWRlZDhhMGQxY2I0IiwidHlwIjoiQmVhcmVyIiwiYXpwIjoiZWFzZWUiLCJzZXNzaW9uX3N0YXRlIjoiYWE3NjVhY2EtOGRmYS00YzZmLTlmOTMtMTdhZTU3MGJkNmQ3IiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwczovL3BvcnRhbC5lYXNlZS5jb20iLCJodHRwczovL29hdXRoLnBzdG1uLmlvIiwiaHR0cHM6Ly9maXJtd2FyZS1tYW5hZ2VyLmVhc2VlLmNsb3VkIiwiaHR0cHM6Ly9lYXNlZS5jbG91ZCJdLCJyZXNvdXJjZV9hY2Nlc3MiOnsiZWFzZWUtY29tbWFuZGV1ciI6eyJyb2xlcyI6WyJjb21tYW5kX2hpc3RvcnlfcmVhZCIsImNvbW1hbmRfd3JpdGUiXX0sImVhc2VlLXByaW1vcmRpYWwiOnsicm9sZXMiOlsiZGV2aWNlX21hbnVmYWN0dXJpbmdfcmVhZCJdfSwiZWFzZWUtY2hyb25pY2xlIjp7InJvbGVzIjpbIm9ic2VydmF0aW9uX2hpc3RvcnlfcmVhZCJdfSwiZWFzZWUtZGV2aWNlLXNldHRpbmdzIjp7InJvbGVzIjpbInNldHRpbmdzX3dyaXRlIiwic2V0dGluZ3NfcmVhZCJdfSwiYWNjb3VudCI6eyJyb2xlcyI6WyJtYW5hZ2UtYWNjb3VudCIsIm1hbmFnZS1hY2NvdW50LWxpbmtzIiwidmlldy1wcm9maWxlIl19LCJlYXNlZS1jdXJyZW50LWRldmljZS1zdGF0ZSI6eyJyb2xlcyI6WyJvYnNlcnZhdGlvbl9zdGF0ZV9yZWFkIl19fSwic2NvcGUiOiJwcm9maWxlIGVtYWlsIiwic2lkIjoiYWE3NjVhY2EtOGRmYS00YzZmLTlmOTMtMTdhZTU3MGJkNmQ3IiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIkFjY291bnRJZCI6IjMwMDk4MSIsInJvbGUiOlsiVXNlciJdLCJTdXBwb3J0IjoiRmFsc2UiLCJVc2VySWQiOiIyNjU1MTQiLCJyb2xlcyI6WyJjb21tYW5kX2hpc3RvcnlfcmVhZCIsImNvbW1hbmRfd3JpdGUiLCJkZXZpY2VfbWFudWZhY3R1cmluZ19yZWFkIiwib2JzZXJ2YXRpb25faGlzdG9yeV9yZWFkIiwic2V0dGluZ3Nfd3JpdGUiLCJzZXR0aW5nc19yZWFkIiwibWFuYWdlLWFjY291bnQiLCJtYW5hZ2UtYWNjb3VudC1saW5rcyIsInZpZXctcHJvZmlsZSIsIm9ic2VydmF0aW9uX3N0YXRlX3JlYWQiXSwibmFtZSI6Ik5pY2xhcyBSb3NlbmdyZW4iLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiIrNDY3MzcwNzc3NDYiLCJnaXZlbl9uYW1lIjoiTmljbGFzIiwiZmFtaWx5X25hbWUiOiJSb3NlbmdyZW4iLCJlbWFpbCI6Im5pY2xhc0BuaXJvLm51In0.kY1gDIIA6fx_jlt34SPQH4L9zA6_EBLyx3tfUF_9xDSoJcP0r5cp332qNRMs9IgsRYLjTtl5maahyAx-3ijnjlFDGr6Hr4cq9jAp7luV1YrpNuAK2UP91fZIEB6oSXw6slTZilQt1ccCp0Gj3S69QSkeKkInr-LVlaqQ4_xcWkzwBZGrC02umeKZVg2Sw7liTM9KYhEKzF_ZOPZL-ILly7fxZaPO10AV6S2XrA-qaCHDrxcKgP-u4OFdigONQE9IV5u6fxsE5FfW50ipRbuNwMO6BxKyshAWCWCmG2j_-fj1oRjCnthg-plUyQkKEYYltKNljbqORw_bxxq3exwVWQ";

        token.parse::<Session>().expect("Parsing");
    }
}
