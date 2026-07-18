use hops::{adapters::comms::CommsAdapter, ports::comms::CommsPort};
use tracing::{error, info};

const SMS_FROM: &str = "Hops";
const CALL_FROM: &str = "+46766860615";
const TO: &str = "+4792419704";
const MESSAGE: &str = "Hello!";
const AUDIO_URL: &str = "https://4494-62-97-169-16.ngrok-free.app/call.mp3";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let username = std::env::var("ELKS_USERNAME").expect("ELKS_USERNAME not set");
    let password = std::env::var("ELKS_PASSWORD").expect("ELKS_PASSWORD not set");

    let comms = CommsAdapter::new(username, password);

    // match comms.send_sms(SMS_FROM, TO, MESSAGE).await {
    //     Ok(response) => {
    //         info!(id = %response.id, status = %response.status, cost = %response.cost, "SMS sent successfully");
    //     }
    //     Err(e) => {
    //         error!(error = %e, "Failed to send SMS");
    //     }
    // }

    match comms.make_call(CALL_FROM, TO, AUDIO_URL).await {
        Ok(response) => {
            info!(id = %response.id, state = %response.state, "Call initiated");
        }
        Err(e) => {
            error!(error = %e, "Failed to make call");
        }
    }
}
