use hops::{adapters::comms::CommsAdapter, ports::comms::CommsPort};
use tracing::{error, info};

const SMS_FROM: &str = "Hops";
const TO: &str = "+4741387142";
const MESSAGE: &str = "Hello!";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let username = std::env::var("ELKS_USERNAME").expect("ELKS_USERNAME not set");
    let password = std::env::var("ELKS_PASSWORD").expect("ELKS_PASSWORD not set");

    let comms = CommsAdapter::new(username, password);

    match comms.send_sms(SMS_FROM, TO, MESSAGE).await {
        Ok(response) => {
            info!(id = %response.id, status = %response.status, cost = %response.cost, "SMS sent successfully");
        }
        Err(e) => {
            error!(error = %e, "Failed to send SMS");
        }
    }
}
