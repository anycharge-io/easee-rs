use anyhow::Result;
use easee_rs::{requests::GetSites, Client};

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = Client::from_env()?;

    let sites = GetSites::default().send(&client).await?;

    println!("Fetched sites:");
    println!("{:#?}", &sites);
    Ok(())
}
