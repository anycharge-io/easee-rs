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
    pub user_role: Option<u8>, // 1, 2, 3, 20
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
    pub user_role: Option<u8>,       // 1, 2, 3, 20
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

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_site_example1() {
        let json = r#"
{"uuid":"48cd5363-6520-4869-8bf4-7774598a3f87","id":85096,"siteKey":"E6US-N222","name":"Brf Ryssjan 222","accessMode":2,"levelOfAccess":3,"address":{"street":"Blecktornsstigen","buildingNumber":null,"zip":"116 66","area":"Stockholm","country":{"id":"SE","name":"Sweden","phonePrefix":0},"latitude":null,"longitude":null,"altitude":null},"contactInfo":{"installerName":"Easycharging ","installerPhoneNumber":"020100053","ownerName":"EcoVision AB","ownerPhoneNumber":"+46703564597","company":null},"costModel":"fixedprice","costModelVariant":null,"costPerKWh":2.19,"costPerKwhExcludeVat":1.752,"currencyId":"SEK","siteType":100,"siteCategory":0,"ratedCurrent":0.0,"vat":25.0,"partnerId":7680,"installerId":4735,"installerUserId":null,"supportId":null,"installerAlias":null,"useDynamicMaster":false,"circuits":[{"id":84131,"uuid":"c40a9cb4-33ed-4b1e-871e-d9bb928d703b","siteId":85096,"circuitPanelId":4,"panelName":"4","ratedCurrent":20.0,"fuse":null,"chargers":[{"id":"EC32KXFN","name":"Höger EC32KXFN","color":null,"createdOn":"2021-02-24T11:02:21.282008","updatedOn":"2021-04-29T11:52:09.870076","backPlate":{"id":"816CA5D2ED1304","masterBackPlateId":"806CA5D264A104","name":"","backplateType":0,"features":[]},"levelOfAccess":null,"productCode":100,"userRole":null,"isTemporary":false},{"id":"ECS7ZRJY","name":"Vänster ECS7ZRJY","color":null,"createdOn":"2021-02-24T10:48:17.259166","updatedOn":"2021-04-29T11:52:09.873356","backPlate":{"id":"806CA5D264A104","masterBackPlateId":"806CA5D264A104","name":"","backplateType":0,"features":[]},"levelOfAccess":null,"productCode":100,"userRole":null,"isTemporary":false}],"masterBackplate":null,"useDynamicMaster":false,"parentCircuitId":null}],"equalizers":[],"createdOn":"2021-04-19T11:47:10.105092","updatedOn":"2025-02-13T08:41:10.861026","userRole":null,"allowedSiteActions":[],"support":null,"regulations":[]}
"#;

        serde_json::from_str::<super::Site>(json).expect("deserializing charger example 1");
    }
}
