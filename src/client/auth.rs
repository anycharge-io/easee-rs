use crate::models;
use base64::{engine::general_purpose, Engine};
use chrono::{TimeZone, Utc};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("malformed access token: {0}")]
    Malformed(&'static str),

    #[error("invalid base64: {0}")]
    InvalidB64(#[from] base64::DecodeError),

    #[error("invalid json")]
    InvalidJson,

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
    pub account_id: Option<u64>,
    pub user_id: Option<u64>,
    pub role: UserRole,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[derive(serde::Deserialize)]
struct JsonAccessToken {
    #[serde(rename = "AccountId")]
    account_id: Option<u64>,
    #[serde(rename = "UserId")]
    user_id: Option<u64>,

    //unique_name: Option<String>,

    //nbf: i64,
    exp: i64,
    iat: i64,

    #[serde(rename = "role")]
    role: Option<UserRole>,
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

        let bs = general_purpose::STANDARD.decode(claims)?;
        let des = match serde_json::from_slice::<JsonAccessToken>(&bs) {
            Ok(res) => res,

            Err(err) if err.is_syntax() => return Err(ParseError::InvalidJson),
            Err(err) => return Err(ParseError::UnexpectedData(err)),
        };

        let issued_at = chrono::Utc.timestamp_millis_opt(des.iat).latest().ok_or(
            ParseError::InvalidUtcTime {
                unix_ts: des.iat,
                field: "iat",
            },
        )?;
        let expires_at = chrono::Utc.timestamp_millis_opt(des.exp).latest().ok_or(
            ParseError::InvalidUtcTime {
                unix_ts: des.exp,
                field: "exp",
            },
        )?;

        Ok(Session {
            raw: s.into(),
            issued_at,
            expires_at,
            account_id: des.account_id,
            user_id: des.user_id,
            role: des.role.unwrap_or(UserRole::Unknown),
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
    use super::{JsonAccessToken, Session};

    #[test]
    fn deserialize_access_token() {
        let s = r#"
{
  "AccountId": 300981,
  "UserId": 265514,
  "unique_name": "Niclas  Rosengren",
  "nbf": 1676731057,
  "exp": 1676817457,
  "iat": 1676731057,
  "role": "User"
}
"#;

        serde_json::from_str::<JsonAccessToken>(s).expect("deserializing");
    }

    #[test]
    fn parse_access_token() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCIsImN0eSI6IkpXVCJ9.eyJBY2NvdW50SWQiOjM1MDk4MSwiVXNlcklkIjoyNjU1MTQsInVuaXF1ZV9uYW1lIjoiTmlib3F4ICB3bTNlb2d3ZW4iLCJuYmYiOjE2NzY3MzEwNTcsImV4cCI6MTY3NjgxNzQ1NywiaWF0IjoxNjc2NzMxMDU3LCJyb2xlIjoiVXNlciJ9.W1-G1yhGv3w9tMn8vIyXgTqUMetkqyNFsQSh9BnjiHY";

        token.parse::<Session>().expect("Parsing");
    }
}
