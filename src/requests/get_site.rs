use crate::{Client, NoBody, Result, Site, SiteId};

pub struct GetSite(pub SiteId);

impl GetSite {
    pub async fn send(&self, client: &Client) -> Result<Site> {
        let site_id = self.0 .0;
        client
            .req::<_, Site>(http::Method::GET, &format!("api/sites/{site_id}"), NoBody)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Site;

    #[test]
    fn deserialize() {
        serde_json::from_str::<Site>(EXAMPLE).expect("deser");
    }

    static EXAMPLE: &str = r#"
{"id":302011,"siteKey":"5SGR-B422","name":"Ulf Rosengren","levelOfAccess":1,"address":{"street":"Badelundavägen","buildingNumber":"5","zip":"16856","area":"Bromma","country":{"id":"SE","name":"Sweden","phonePrefix":0},"latitude":null,"longitude":null,"altitude":null},"contactInfo":{"installerName":"Marcus Johansson","installerPhoneNumber":"0735362435","ownerName":"Ulf Rosengren","ownerPhoneNumber":"+46705876361","company":null},"costPerKWh":5.2,"costPerKwhExcludeVat":5.2,"currencyId":"SEK","siteType":1,"ratedCurrent":25.0,"vat":0.0,"partnerId":20,"installerId":null,"useDynamicMaster":false,"circuits":[{"id":288464,"siteId":302011,"circuitPanelId":1,"panelName":"Laddbox","ratedCurrent":16.0,"chargers":[{"id":"EHNCA966","name":"Uppfart","color":3,"createdOn":"2021-11-01T12:00:18.044574","updatedOn":"2022-04-03T15:09:06.507413","backPlate":{"id":"806CA5C285BE04","masterBackPlateId":"806CA5C285BE04","name":"Uppfart","features":[]},"levelOfAccess":2,"productCode":1,"userRole":2,"isTemporary":false}],"masterBackplate":null,"useDynamicMaster":false,"parentCircuitId":null}],"equalizers":[],"createdOn":"2022-03-31T09:22:30.319709","updatedOn":"2022-12-24T19:41:35.647331","userRole":2,"allowedSiteActions":["AllowToConfigureLevelOfAccess"]}
"#;
}
