use anyhow::Result;
use easeeapi::{requests::GetSites, Client};
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let client = Client::from_env()?;

    info!("fetching sites");
    let sites = GetSites::default().send(&client).await?;

    println!("Fetched sites:");
    println!("{:#?}", &sites);
    Ok(())
}
