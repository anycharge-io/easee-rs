use anyhow::{Context, Result};
use easee_rs::{requests::GetSite, Client, SiteId};

#[tokio::main]
pub async fn main() -> Result<()> {
    let site_id = std::env::args()
        .nth(1)
        .context("expected site id as first arg")?
        .parse::<i64>()
        .context("site id must be an int")?;

    let client = Client::from_env()?;

    let site = GetSite(SiteId(site_id)).send(&client).await?;

    println!("Fetched sites {site_id}");
    println!("{:#?}", &site);
    Ok(())
}
