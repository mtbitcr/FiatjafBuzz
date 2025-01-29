use chrono::{Datelike, Local};
use dotenv::dotenv;
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
        let now = Local::now();
        let message = if now.weekday().number_from_monday() >= 6 {
            MESSAGE_TO_PUBLISH_WEEKEND
        } else {
            MESSAGE_TO_PUBLISH_WEEKDAY
        };

        let event = EventBuilder::text_note(message)
            .build(keys.public_key)
            .sign(&keys)
            .await
            .unwrap();
        client.send_event(event).await?;

        sleep(Duration::from_secs(TWO_DAYS_IN_SECONDS)).await;
    }
}
