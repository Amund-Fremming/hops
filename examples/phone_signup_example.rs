use std::sync::Arc;

use hops::adapters::comms::CommsAdapter;
use hops::adapters::crypto::CryptoAdapter;
use hops::config::CONFIG;
use hops::db;
use hops::db::otp::{create_otp, get_otp_by_id, mark_verified};
use hops::models::otp::Otp;
use hops::services::auth::AuthService;
use hops::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

const PHONE_NUMBER: &str = "+4741387142";
const SEND_REAL_SMS: bool = false;

async fn otp_flow_successful_code(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let otp_response = create_otp(state.get_pool(), PHONE_NUMBER, &hash, 5, 10).await?;

    if SEND_REAL_SMS {
        state
            .comms
            .send_sms(
                "Hops",
                PHONE_NUMBER,
                &format!("Your login code is: {}", code),
            )
            .await?;
    }

    let otp = get_otp_by_id(state.get_pool(), otp_response.otp_id).await?;

    if otp.is_expired() {
        return Err("OTP expired".into());
    }

    if otp.is_max_attempts_exceeded(3) {
        return Err("Max attempts exceeded".into());
    }

    if !state.crypto.verify(&code, &otp.hash) {
        return Err("Code verification failed".into());
    }

    mark_verified(state.get_pool(), otp_response.otp_id).await?;

    Ok(())
}

async fn otp_flow_failed_code(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let otp_response = create_otp(state.get_pool(), PHONE_NUMBER, &hash, 5, 10).await?;

    if SEND_REAL_SMS {
        state
            .comms
            .send_sms(
                "Hops",
                PHONE_NUMBER,
                &format!("Your login code is: {}", code),
            )
            .await?;
    }

    let otp = get_otp_by_id(state.get_pool(), otp_response.otp_id).await?;

    if otp.is_expired() {
        return Err("OTP expired".into());
    }

    if otp.is_max_attempts_exceeded(3) {
        return Err("Max attempts exceeded".into());
    }

    if !state.crypto.verify("000000", &otp.hash) {
        return Err("Wrong code".into());
    }

    mark_verified(state.get_pool(), otp_response.otp_id).await?;

    Ok(())
}

async fn complete_signup_flow_successful(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create OTP
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let otp = db::otp::create_otp(state.get_pool(), PHONE_NUMBER, &hash, 5, 10).await?;

    // 2. Verify OTP (simulating user entering correct code)
    let fetched_otp = db::otp::get_otp_by_id(state.get_pool(), otp.otp_id).await?;
    if fetched_otp.is_expired() {
        return Err("OTP expired".into());
    }
    if !state.crypto.verify(&code, &fetched_otp.hash) {
        return Err("Code verification failed".into());
    }
    db::otp::mark_verified(state.get_pool(), otp.otp_id).await?;

    // 3. Complete signup via auth service
    let tokens = state
        .auth
        .phone_signup(
            otp.otp_id,
            "Test Device",
            Some("example-user-agent"),
            "Test",
            "User",
            "SecurePassword123!",
        )
        .await?;

    info!(
        access_token_len = tokens.access_token.len(),
        refresh_token_len = tokens.refresh_token.len(),
        access_expires_in = tokens.access_expires_in,
        refresh_expires_in = tokens.refresh_expires_in,
        "Full signup flow completed"
    );

    Ok(())
}

async fn create_state() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database.url)
        .await?;

    let crypto = Arc::new(CryptoAdapter::new(CONFIG.crypto.secret.clone()));

    let auth = Arc::new(AuthService::new(
        CONFIG.auth.clone(),
        pool.clone(),
        crypto.clone(),
        CONFIG.auth.private_key_pem(),
        CONFIG.auth.public_key_pem(),
        &CONFIG.auth.audience,
        &CONFIG.auth.issuer,
    )?);

    let comms = Arc::new(CommsAdapter::new(
        CONFIG.comms.username.clone(),
        CONFIG.comms.password.clone(),
        CONFIG.comms.resend_api_key.clone(),
    ));

    let state = AppState::new(pool, auth, comms, crypto);
    Ok(Arc::new(state))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let state = create_state().await?;

    match otp_flow_successful_code(state.clone()).await {
        Ok(()) => info!("✅ OTP flow successful"),
        Err(e) => info!("❌ OTP flow failed: {}", e),
    }

    match otp_flow_failed_code(state.clone()).await {
        Ok(()) => info!("❌ OTP flow was correct, should fail"),
        Err(..) => info!("✅ OTP flow failed successfully"),
    }

    match complete_signup_flow_successful(state.clone()).await {
        Ok(()) => info!("✅ Singup flow successful"),
        Err(e) => info!("❌ Signup flow failed: {}", e),
    }

    // TODO
    // - expired otp does not work
    // - max attempts exceeded fails
    // - max entries per day fails

    Ok(())
}
