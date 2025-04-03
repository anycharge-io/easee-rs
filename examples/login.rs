use anyhow::{Result, anyhow};
use easeeapi::Client;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let username = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("first argument must be username"))?;

    let password = std::env::args()
        .nth(2)
        .ok_or_else(|| anyhow!("first argument must be password"))?;

    let client = Client::login(username, password).await?;

    let profile = easeeapi::requests::GetProfile.send(&client).await?;
    let credentials = client.get_credentials().await;

    println!("{credentials:#?}");
    println!("current profile:\n{profile:#?}");
    Ok(())
}
