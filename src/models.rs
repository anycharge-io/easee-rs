use serde::{Deserialize, Serialize};

mod charger_session;
mod raw_session;

pub mod datetime;

pub(crate) use raw_session::RawSession;
pub use {charger_session::*, datetime::DateTime};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SiteId(pub i64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: SiteId,
    pub site_key: String,
    pub name: String,
    pub level_of_access: Option<u8>, // 1 - 3
    pub address: Address,
    pub contact_info: Option<ContactInfo>,

    #[serde(rename = "costPerKWh")]
    pub cost_per_kwh: Option<f64>,
    pub cost_per_kwh_exclude_vat: Option<f64>,
    pub currency_id: Option<String>,
    pub site_type: u32, // 1, 100, 400, 1000
    pub rated_current: f64,
    pub vat: Option<f64>,
    pub partner_id: Option<u32>,
    pub installer_id: Option<u64>,
    pub use_dynamic_master: Option<bool>,

    #[serde(default = "Vec::new")]
    pub circuits: Vec<Circuit>,
    #[serde(default = "Vec::new")]
    pub equalizers: Vec<Equalizer>,

    pub created_on: DateTime,

    pub updated_on: DateTime,
    pub user_role: u8, // 1, 2, 3, 20
    #[serde(default = "Vec::new")]
    pub allowed_site_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteSub {
    pub id: SiteId,
    pub site_key: String,
    pub name: String,
    pub level_of_access: Option<u8>, // 1 - 3
    pub address: Address,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: Option<String>,
    pub building_number: Option<String>,
    pub zip: Option<String>,
    pub area: Option<String>,
    pub country: Option<Country>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: String,
    pub name: String,
    pub phone_prefix: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInfo {
    pub installer_name: String,
    pub installer_phone_number: String,
    pub owner_name: Option<String>,
    pub owner_phone_number: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Charger {
    pub id: String,
    pub name: String,
    pub color: Option<u8>, // 1 - 5
    pub created_on: String,
    pub updated_on: String,
    pub back_plate: BackPlate,
    pub level_of_access: Option<u8>, // 1 - 3
    pub product_code: u32,           // 1, 100, 1000
    pub user_role: u8,               // 1, 2, 3, 20
    pub is_temporary: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackPlate {
    pub id: String,
    pub master_back_plate_id: String,
    pub name: String,
    pub features: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equalizer {
    pub id: String,
    pub name: String,
    pub site_id: i32,
    pub circuit_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: i64,
    #[serde(rename = "eMail")]
    pub email: String,

    #[serde(rename = "phoneNo")]
    pub phone_nr: String,

    #[serde(rename = "firstName")]
    pub firstname: String,

    #[serde(rename = "lastName")]
    pub lastname: String,
}
