use hops::adapters::comms::adapter::CommsAdapter;
use hops::ports::comms_port::CommsPort;
use tracing::{error, info};

const FROM: &str = "Mordi";
const TO: &str = "+4747666050";
const MESSAGE: &str = "Du er heldig som har Amund";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let username = std::env::var("ELKS_USERNAME").expect("ELKS_USERNAME not set");
    let password = std::env::var("ELKS_PASSWORD").expect("ELKS_PASSWORD not set");

    let comms = CommsAdapter::new(username, password);

    match comms.send_sms(FROM, TO, MESSAGE).await {
        Ok(response) => {
            info!(id = %response.id, status = %response.status, cost = %response.cost, "SMS sent successfully");
        }
        Err(e) => {
            error!(error = %e, "Failed to send SMS");
        }
    }
}
