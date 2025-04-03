use anyhow::{Context, Result};
use easeeapi::{Client, requests};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let charger_id = std::env::args()
        .nth(1)
        .context("expected charger id as first arg")?;

    let from = time::macros::date!(2024 - 04 - 01);
    let to = time::macros::date!(2024 - 04 - 30);

    let client = Client::from_env()?;

    let sessions = requests::GetChargerSessions::new(charger_id, from, to)
        .send(&client)
        .await?;

    println!("{:#?}", &sessions);
    println!("Fetched {} sessions", sessions.len());
    Ok(())
}
