use super::DateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargerSession {
    #[serde(alias = "sessionId")]
    pub id: i64,

    #[serde(alias = "sessionStart")]
    pub car_connected: DateTime,

    #[serde(alias = "sessionEnd")]
    pub car_disconnected: Option<DateTime>,

    #[serde(alias = "sessionEnergy")]
    pub kilo_watt_hours: f64,

    pub price_per_kwh_excluding_vat: f64,
    pub price_pr_kwh_including_vat: f64,
    pub cost_excluding_vat: f64,
    pub cost_including_vat: f64,
    pub vat_percentage: Option<f64>,
    pub currency: Option<String>,
    pub actual_duration_seconds: Option<i64>,

    pub first_energy_transfer_period_started: Option<DateTime>,
    pub last_energy_transfer_period_end: Option<DateTime>,

    #[serde(alias = "authToken")]
    pub auth_token: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize() {
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
   }"#;

        serde_json::from_str::<ChargerSession>(&s).expect("deserializing");
    }

    #[test]
    fn deserialize_failing_1() {
        // Easee API: Error deserializing reply: missing field `actualDurationSeconds` at line 1 column 610. Body:
        let s = r#"
[
  {
    "carConnected": "2024-04-29T21:08:15Z",
    "carDisconnected": "2024-04-30T16:24:29Z",
    "kiloWattHours": 7.544205,
    "pricePerKwhExcludingVat": 0,
    "pricePrKwhIncludingVat": 0,
    "costExcludingVat": 0,
    "costIncludingVat": 0,
    "actualDurationSeconds": 3924,
    "firstEnergyTransferPeriodStarted": "2024-04-29T21:08:18+00:00",
    "lastEnergyTransferPeriodEnd": "2024-04-29T22:13:42+00:00",
    "id": 191,
    "isComplete": true
  },
  {
    "carConnected": "2024-04-30T22:55:54Z",
    "carDisconnected": "2024-05-01T11:48:45Z",
    "kiloWattHours": 6.602936,
    "pricePerKwhExcludingVat": 0,
    "pricePrKwhIncludingVat": 0,
    "costExcludingVat": 0,
    "costIncludingVat": 0,
    "id": 192,
    "isComplete": true
  }
]
"#;

        serde_json::from_str::<Vec<ChargerSession>>(&s).expect("deserializing");
    }

    #[test]
    fn deserialize_failing_2() {
        // 2024-05-01T18:04:33.491368Z  WARN anycharge::easee_worker: EaseeWorker:
        // Easee API: Error deserializing reply: missing field `carConnected` at line 1 column 385. Body:

        let s = r#"
{
  "chargerId": "EC3VJ7GU",
  "sessionEnergy": 2.454530715942383,
  "sessionStart": "2024-05-01T15:55:38Z",
  "sessionEnd": "2024-05-01T16:24:49Z",
  "sessionId": 193,
  "chargeDurationInSeconds": 1748,
  "firstEnergyTransferPeriodStart": "2024-05-01T15:55:41Z",
  "lastEnergyTransferPeriodEnd": "2024-05-01T16:24:49Z",
  "pricePrKwhIncludingVat": 0,
  "pricePerKwhExcludingVat": 0,
  "costIncludingVat": 0,
  "costExcludingVat": 0
}
"#;

        serde_json::from_str::<ChargerSession>(s).expect("deserializing");
    }

    // caught when skimming through logs, seems to be a list a sessions in which the field actualDurationSeconds fails to deserialize
    #[test]
    fn deserialize_failing_3() {
        let s = include_str!("../../test_data/failing_sessions.json");

        serde_json::from_str::<Vec<ChargerSession>>(s)
            .expect("deserializing `test_data/failing_sessions.json`");
    }
}
