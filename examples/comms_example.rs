use hops::adapters::comms::send_sms;
use tracing::{error, info};

const FROM: &str = "Hops";
const TO: &str = "+4741387142";
const MESSAGE: &str = "Hello from Hops!";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let username = std::env::var("ELKS_USERNAME").expect("ELKS_USERNAME not set");
    let password = std::env::var("ELKS_PASSWORD").expect("ELKS_PASSWORD not set");

    match send_sms(&username, &password, FROM, TO, MESSAGE).await {
        Ok(response) => {
            info!(id = %response.id, status = %response.status, cost = %response.cost, "SMS sent successfully");
        }
        Err(e) => {
            error!(error = %e, "Failed to send SMS");
        }
    }
}
