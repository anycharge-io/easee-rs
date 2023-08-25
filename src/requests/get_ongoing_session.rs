use crate::{ChargerSession, Client, NoBody, Result, StateAuthenticated};

pub struct GetOngoingSession {
    charger_id: String,
}

impl GetOngoingSession {
    /// Fetches active session for given Charger ID (if any)
    pub fn new(charger_id: impl Into<String>) -> Self {
        Self {
            charger_id: charger_id.into(),
        }
    }

    pub async fn send(
        &self,
        client: &Client<StateAuthenticated>,
    ) -> Result<Option<ChargerSession>> {
        let charger_id = &self.charger_id;
        let url = format!("api/chargers/{charger_id}/sessions/ongoing");

        client
            .req::<_, Option<ChargerSession>>(http::Method::GET, &dbg!(url), NoBody)
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize_session() {
        let s = r#"
 {
    "carConnected": "2023-01-20T19:31:46Z",
    "carDisconnected": "2023-01-20T19:35:28Z",
    "kiloWattHours": 0.581242,
    "pricePerKwhExcludingVat": 0.0,
    "pricePrKwhIncludingVat": 0.0,
    "costExcludingVat": 0.0,
    "costIncludingVat": 0.0,
    "vatPercentage": null,
    "currency": null,
    "actualDurationSeconds": 215,
    "firstEnergyTransferPeriodStarted": "2023-01-20T19:31:50+00:00",
    "lastEnergyTransferPeriodEnd": "2023-01-20T19:35:25+00:00",
    "id": 4
  }
"#;

        let session = serde_json::from_str::<ChargerSession>(s).expect("des");

        assert_eq!(session.kilo_watt_hours, 0.581242);

        assert_eq!(session.car_connected.0.year(), 2023);
        assert_eq!(session.car_connected.0.month(), time::Month::January);
        assert_eq!(session.car_connected.0.day(), 20);
        assert_eq!(session.car_connected.0.hour(), 19);
        assert_eq!(session.car_connected.0.minute(), 31);
        assert_eq!(session.car_connected.0.second(), 46);

        assert_eq!(session.first_energy_transfer_period_started.0.year(), 2023);
        assert_eq!(
            session.first_energy_transfer_period_started.0.month(),
            time::Month::January
        );
        assert_eq!(session.first_energy_transfer_period_started.0.day(), 20);
        assert_eq!(session.first_energy_transfer_period_started.0.hour(), 19);
        assert_eq!(session.first_energy_transfer_period_started.0.minute(), 31);
        assert_eq!(session.first_energy_transfer_period_started.0.second(), 50);
    }
}
