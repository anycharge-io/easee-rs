use anyhow::{anyhow, Result};
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

    let client = Client::unauthenticated();

    let client = client.login(username, password).await?;

    let (session, refresh_token) = client.get_auth_session().await;

    println!("{session:#?}");
    println!("{refresh_token:#?}");
    Ok(())
}
