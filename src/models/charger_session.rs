use super::DateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargerSession {
    pub car_connected: DateTime,

    pub car_disconnected: DateTime,
    pub kilo_watt_hours: f64,
    pub price_per_kwh_excluding_vat: f64,
    pub price_pr_kwh_including_vat: f64,
    pub cost_excluding_vat: f64,
    pub cost_including_vat: f64,
    pub vat_percentage: Option<f64>,
    pub currency: Option<String>,
    pub actual_duration_seconds: i64,
    pub first_energy_transfer_period_started: DateTime,
    pub last_energy_transfer_period_end: DateTime,
    pub id: i64,
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
}
