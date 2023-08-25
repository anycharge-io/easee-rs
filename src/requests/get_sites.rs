use crate::{Client, NoBody, Result, SiteSub, StateAuthenticated};

#[derive(Default, Clone)]
pub struct GetSites {
    search: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
}

impl GetSites {
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn send(&self, client: &Client<StateAuthenticated>) -> Result<Vec<SiteSub>> {
        client.req(http::Method::GET, "api/sites", NoBody).await
    }
}

#[cfg(test)]
mod tests {
    use crate::SiteSub;

    #[test]
    fn deserialize_resp() {
        let s = r#"[
  {
    "id": 1,
    "siteKey": "123-2323",
    "name": "derpprer",
    "levelOfAccess": null,
    "address": {
      "street": "Beasdasdasd",
      "buildingNumber": null,
      "zip": "123 45",
      "area": "Bromma",
      "country": null,
      "latitude": null,
      "longitude": null,
      "altitude": null
    }
  },
  {
    "id": 2,
    "siteKey": "2331-2322",
    "name": "asdasd",
    "levelOfAccess": null,
    "address": {
      "street": "steretetet",
      "buildingNumber": null,
      "zip": "123 45",
      "area": "AAAAAArea",
      "country": null,
      "latitude": null,
      "longitude": null,
      "altitude": null
    }
  },
  {
    "id": 3,
    "siteKey": "1234-1324",
    "name": "Deru",
    "levelOfAccess": null,
    "address": {
      "street": "Streeety",
      "buildingNumber": null,
      "zip": "12345",
      "area": "Areaaaa",
      "country": null,
      "latitude": null,
      "longitude": null,
      "altitude": null
    }
  }
]"#;

        serde_json::from_str::<Vec<SiteSub>>(s).expect("deserializing");
    }
}
