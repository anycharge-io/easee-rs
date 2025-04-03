use crate::{Client, JsonBody, NoBody, Result, Site, SiteId};

pub struct GetSite(pub SiteId);

impl GetSite {
    pub async fn send(&self, client: &Client) -> Result<Site> {
        let site_id = self.0 .0;
        client
            .req::<_, JsonBody<Site>>(http::Method::GET, &format!("api/sites/{site_id}"), NoBody)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Site;

    #[test]
    fn deserialize() {
        serde_json::from_str::<Site>(EXAMPLE_1).expect("deser");
        serde_json::from_str::<Site>(EXAMPLE_2).expect("deser");
    }

    static EXAMPLE_1: &str = r#"
{
  "id": 576136,
  "siteKey": "534P-E622",
  "name": "Brf Lindblomman 212",
  "levelOfAccess": 3,
  "address": {
    "street": "Beckombergavägen",
    "buildingNumber": "212",
    "zip": "168 61",
    "area": "Bromma",
    "country": {
      "id": "SE",
      "name": "Sweden",
      "phonePrefix": 0
    },
    "latitude": null,
    "longitude": null,
    "altitude": null
  },
  "contactInfo": {
    "installerName": "Mattias Ström",
    "installerPhoneNumber": "0703940167",
    "ownerName": "ChargeBuddy AB",
    "ownerPhoneNumber": "+46722518116",
    "company": null
  },
  "costPerKWh": 0.0,
  "costPerKwhExcludeVat": 0.0,
  "currencyId": null,
  "siteType": 100,
  "ratedCurrent": 100.0,
  "vat": 0.0,
  "partnerId": 20,
  "installerId": null,
  "useDynamicMaster": false,
  "circuits": [
    {
      "id": 545162,
      "siteId": 576136,
      "circuitPanelId": 1,
      "panelName": "Parkering 1-6 ",
      "ratedCurrent": 32.0,
      "chargers": [
        {
          "id": "ECQK2K2L",
          "name": "Parkering 6",
          "color": null,
          "createdOn": "2022-10-12T05:01:39.518305",
          "updatedOn": "2023-01-12T14:58:32.612267",
          "backPlate": {
            "id": "910FA03AD00604",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 6",
            "features": []
          },
          "levelOfAccess": 3,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "EC2QAEP6",
          "name": "Parkering 5",
          "color": null,
          "createdOn": "2022-10-12T04:59:12.986055",
          "updatedOn": "2023-01-12T14:56:58.449368",
          "backPlate": {
            "id": "90105D82F9FD04",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 5",
            "features": []
          },
          "levelOfAccess": null,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECPZZGKE",
          "name": "Parkering 4",
          "color": null,
          "createdOn": "2022-10-12T05:59:50.301933",
          "updatedOn": "2023-01-12T14:56:20.718129",
          "backPlate": {
            "id": "91105D82B85B04",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 4",
            "features": []
          },
          "levelOfAccess": null,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECJ2A77E",
          "name": "Parkering 3",
          "color": null,
          "createdOn": "2022-10-10T20:44:38.780023",
          "updatedOn": "2023-01-12T14:55:21.52399",
          "backPlate": {
            "id": "91105D82847C04",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 3",
            "features": []
          },
          "levelOfAccess": null,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECSHXRBH",
          "name": "Parkering 2",
          "color": null,
          "createdOn": "2022-10-11T13:50:06.277153",
          "updatedOn": "2023-01-12T14:53:50.101645",
          "backPlate": {
            "id": "90105D82E33C04",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 2",
            "features": []
          },
          "levelOfAccess": 1,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "EC67VLHP",
          "name": "Parkering 1",
          "color": null,
          "createdOn": "2022-10-12T05:01:38.670077",
          "updatedOn": "2023-01-12T14:52:54.11734",
          "backPlate": {
            "id": "90105D82E08604",
            "masterBackPlateId": "90105D82E08604",
            "name": "Parkering 1",
            "features": []
          },
          "levelOfAccess": 3,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        }
      ],
      "masterBackplate": null,
      "useDynamicMaster": false,
      "parentCircuitId": null
    }
  ],
  "equalizers": [],
  "createdOn": "2023-01-12T14:44:57.04698",
  "updatedOn": "2023-03-30T08:17:59.292387",
  "userRole": 2,
  "allowedSiteActions": [
    "AllowToConfigureLevelOfAccess"
  ]
}
"#;

    static EXAMPLE_2: &str = r#"
{
  "id": 575766,
  "siteKey": "EJ7L-E622",
  "name": "Brf Lindblomman 213",
  "levelOfAccess": 3,
  "address": {
    "street": "Beckombergavägen",
    "buildingNumber": "213",
    "zip": "168 61",
    "area": "Bromma",
    "country": {
      "id": "SE",
      "name": "Sweden",
      "phonePrefix": 0
    },
    "latitude": null,
    "longitude": null,
    "altitude": null
  },
  "contactInfo": {
    "installerName": "Mattias Strm",
    "installerPhoneNumber": "0703940167",
    "ownerName": "ChargeBuddy AB",
    "ownerPhoneNumber": "+46722518116",
    "company": null
  },
  "costPerKWh": 0.0,
  "costPerKwhExcludeVat": 0.0,
  "currencyId": null,
  "siteType": 100,
  "ratedCurrent": 63.0,
  "vat": 0.0,
  "partnerId": 20,
  "installerId": null,
  "useDynamicMaster": false,
  "circuits": [
    {
      "id": 544874,
      "siteId": 575766,
      "circuitPanelId": 1,
      "panelName": "Parkering 11-16",
      "ratedCurrent": 32.0,
      "chargers": [
        {
          "id": "EC9TVZ6L",
          "name": "Parkering 16",
          "color": null,
          "createdOn": "2022-09-14T16:13:11.719836",
          "updatedOn": "2023-01-12T13:37:40.639359",
          "backPlate": {
            "id": "80731B72BEB004",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 16",
            "features": []
          },
          "levelOfAccess": 1,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECJQGJW9",
          "name": "Parkering 15",
          "color": null,
          "createdOn": "2022-09-14T13:33:34.970759",
          "updatedOn": "2023-01-12T13:37:58.856556",
          "backPlate": {
            "id": "8072D692ED6504",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 15",
            "features": []
          },
          "levelOfAccess": 3,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "EC3VJ7GU",
          "name": "Parkering 14",
          "color": 1,
          "createdOn": "2022-09-14T14:32:34.137645",
          "updatedOn": "2023-02-23T20:22:40.635291",
          "backPlate": {
            "id": "81705F8AC70004",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 14",
            "features": []
          },
          "levelOfAccess": null,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECP739TA",
          "name": "Parkering 13",
          "color": null,
          "createdOn": "2022-09-14T14:22:10.558498",
          "updatedOn": "2023-01-12T13:34:23.61977",
          "backPlate": {
            "id": "81731B72732004",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 13",
            "features": []
          },
          "levelOfAccess": 3,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECJFS7YL",
          "name": "Parkering 12 ( gamla handikapp)",
          "color": null,
          "createdOn": "2022-09-14T13:33:36.456982",
          "updatedOn": "2023-01-12T13:32:38.292393",
          "backPlate": {
            "id": "81731B72939204",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 12 ( gamla handikapp)",
            "features": []
          },
          "levelOfAccess": 1,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        },
        {
          "id": "ECLETDJ4",
          "name": "Parkering 11",
          "color": null,
          "createdOn": "2022-09-14T14:32:38.884901",
          "updatedOn": "2023-01-12T13:32:26.347577",
          "backPlate": {
            "id": "8072B5B2406C04",
            "masterBackPlateId": "8072B5B2406C04",
            "name": "Parkering 11",
            "features": []
          },
          "levelOfAccess": 3,
          "productCode": 100,
          "userRole": 2,
          "isTemporary": false
        }
      ],
      "masterBackplate": null,
      "useDynamicMaster": false,
      "parentCircuitId": null
    }
  ],
  "equalizers": [
    {
      "id": "QPTJH7UN",
      "name": "QPTJH7UN",
      "siteId": 575766,
      "circuitId": null
    }
  ],
  "createdOn": "2023-01-12T12:04:22.129187",
  "updatedOn": "2023-03-30T08:19:40.685372",
  "userRole": 2,
  "allowedSiteActions": [
    "AllowToConfigureLevelOfAccess"
  ]
}

"#;
}
