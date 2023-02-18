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

    #[test]
    fn deserialize() {}
}
