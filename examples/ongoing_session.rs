use anyhow::{Context, Result};
use easeeapi::{Client, OptionalResult, requests};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let charger_id = std::env::args()
        .nth(1)
        .context("expected charger id as first arg")?;

    let client = Client::from_env()?;

    let session = requests::GetOngoingSession::new(charger_id)
        .send(&client)
        .await
        .optional()?;

    println!("{:#?}", &session);
    Ok(())
}
