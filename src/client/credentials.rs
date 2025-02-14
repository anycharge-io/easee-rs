use super::Session;
use crate::models::DateTime;
use std::time;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub session: Session,
    pub refresh_token: String,
}

pub enum GetJwt<'a> {
    Valid {
        token: &'a str,
    },
    Expired {
        token: &'a str,
        refresh_token: &'a str,
    },
}

impl Credentials {
    pub(crate) async fn get_valid_session(&self) -> GetJwt<'_> {
        let now = DateTime::now_utc().0;

        // Allow 1 minute grace period.
        let graced_expired_at = self.session.expires_at.0 - time::Duration::from_secs(60);

        if now < graced_expired_at {
            GetJwt::Valid {
                token: &self.session.raw,
            }
        } else {
            GetJwt::Expired {
                token: &self.session.raw,
                refresh_token: &self.refresh_token,
            }
        }
    }

    pub(crate) fn get_tokens(&self) -> (String, String) {
        (self.session.raw.clone(), self.refresh_token.clone())
    }
}
