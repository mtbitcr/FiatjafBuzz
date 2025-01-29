use chrono::{Datelike, Utc, Weekday};
use dotenv::dotenv;
use log::error;
use nostr_sdk::{Client, EventBuilder, Keys, Result};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

pub const NOSTR_RELAY_TO_PUBLISH: &str = "wss://nostr.mom";
pub const MESSAGE_TO_PUBLISH_WEEKDAY: &str = "gfy fiatja";
pub const MESSAGE_TO_PUBLISH_WEEKEND: &str = "GM fiatjaf";
pub const TWO_DAYS_IN_SECONDS: u64 = 172800;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key =
        env::var("NOSTR_PRIVATE_KEY").expect("NOSTR_PRIVATE_KEY is missing in the environment");
    let keys = Keys::from_str(&private_key)?;

    let client = Client::new(keys.clone());
    client.add_relay(NOSTR_RELAY_TO_PUBLISH).await?;
    client.connect().await;

    loop {
        let now = Utc::now();

        let message = match now.weekday() {
            Weekday::Sun | Weekday::Sat => MESSAGE_TO_PUBLISH_WEEKEND,
            _ => MESSAGE_TO_PUBLISH_WEEKDAY,
        };

        let event = EventBuilder::text_note(message)
            .build(keys.public_key)
            .sign(&keys)
            .await?;

        if let Err(_e) = client.send_event(event).await {
            error!("Could not send event, cause: {_e}");
        }

        sleep(Duration::from_secs(TWO_DAYS_IN_SECONDS)).await
    }
}
