use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: i64,
    pub site_key: String,
    pub name: String,
    pub level_of_access: u8, // 1 - 3
    pub address: Address,
    pub contact_info: ContactInfo,

    #[serde(rename = "costPerKWh")]
    pub cost_per_kwh: f64,
    pub cost_per_kwh_exclude_vat: f64,
    pub currency_id: Option<String>,
    pub site_type: u32, // 1, 100, 400, 1000
    pub rated_current: f64,
    pub vat: f64,
    pub partner_id: u32,
    pub installer_id: Option<u64>,
    pub use_dynamic_master: bool,
    pub circuits: Vec<Circuit>,
    pub equalizers: Vec<Equalizer>,
    pub created_on: String,
    pub updated_on: String,
    pub user_role: u8, // 1, 2, 3, 20
    pub allowed_site_actions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: Option<String>,
    pub building_number: Option<String>,
    pub zip: Option<String>,
    pub area: Option<String>,
    pub country: Country,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: String,
    pub name: String,
    pub phone_prefix: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInfo {
    pub installer_name: String,
    pub installer_phone_number: String,
    pub owner_name: Option<String>,
    pub owner_phone_number: Option<String>,
    pub company: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub id: i64,
    pub site_id: i64,
    pub circuit_panel_id: i64,
    pub panel_name: String,
    pub rated_current: f64,
    pub chargers: Vec<Charger>,
    pub master_backplate: Option<String>,
    pub use_dynamic_master: bool,
    pub parent_circuit_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Charger {
    pub id: String,
    pub name: String,
    pub color: u8, // 1 - 5
    pub created_on: String,
    pub updated_on: String,
    pub back_plate: BackPlate,
    pub level_of_access: u8, // 1 - 3
    pub product_code: u32,   // 1, 100, 1000
    pub user_role: u8,       // 1, 2, 3, 20
    pub is_temporary: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackPlate {
    pub id: String,
    pub master_back_plate_id: String,
    pub name: String,
    pub features: Vec<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equalizer {
    pub id: String,
    pub name: String,
    pub site_id: i32,
    pub circuit_id: i32,
}
