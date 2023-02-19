use crate::{
    models::{datetime, DateTime},
    ChargerSession, Client, NoBody, Result,
};

pub struct GetChargerSessions {
    charger_id: String,
    from: DateTime,
    to: DateTime,
}

impl GetChargerSessions {
    pub fn new(charger_id: impl Into<String>, from: DateTime, to: DateTime) -> Self {
        Self {
            charger_id: charger_id.into(),
            from,
            to,
        }
    }

    pub async fn send(&self, client: &Client) -> Result<Vec<ChargerSession>> {
        let charger_id = &self.charger_id;
        let from_s = &self.from.to_rfc3339();
        let to_s = &self.to.to_rfc3339();

        let url = format!("api/sessions/charger/{charger_id}/{from_s}/{to_s}");
        client
            .req::<_, Vec<ChargerSession>>(http::Method::GET, &url, NoBody)
            .await
    }
}
