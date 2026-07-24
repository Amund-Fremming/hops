use std::sync::Arc;

use hops::adapters::comms::CommsAdapter;
use hops::adapters::crypto::CryptoAdapter;
use hops::config::CONFIG;
use hops::models::auth::ProviderType;
use hops::services::auth::AuthService;
use hops::services::otp::OtpService;
use hops::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

const PHONE_NUMBER: &str = "+4741387142";
const EMAIL: &str = "hops@resend.dev";

/// Demonstrates the OTP verification flow using OtpService
async fn phone_otp_flow(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create and send OTP (sends real SMS in production)
    let otp_response = state
        .otp
        .create_and_send(PHONE_NUMBER, ProviderType::Phone)
        .await?;
    info!(otp_id = %otp_response.otp_id, "OTP created and sent");

    // 2. In real flow, user enters code received via SMS
    // For testing, we'd need to capture the code before hashing
    // This is just a placeholder - verification would fail without real code
    let user_entered_code = "123456";

    match state
        .otp
        .verify(otp_response.otp_id, user_entered_code)
        .await
    {
        Ok(()) => info!("OTP verified successfully"),
        Err(e) => info!("OTP verification failed (expected in test): {:?}", e),
    }

    Ok(())
}

/// Demonstrates full phone signup flow using OtpService
/// Note: This requires a real SMS to be sent, so it will create an OTP but
/// verification will fail without the actual code.
async fn phone_signup_flow(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create and send OTP via OtpService
    let otp_response = state
        .otp
        .create_and_send(PHONE_NUMBER, ProviderType::Phone)
        .await?;
    info!(otp_id = %otp_response.otp_id, "OTP created for signup");

    // 2. In production: user receives SMS and enters code
    // For this example, we skip verification

    // Skip verification in example - in real flow you'd call:
    // state.otp.verify(otp_response.otp_id, user_code).await?;
    info!("Skipping OTP verification (would need real code from SMS)");

    // 3. For demo purposes, manually mark as verified using db directly
    // In production, use state.otp.verify() with the real code
    hops::db::otp::mark_verified(state.get_pool(), otp_response.otp_id).await?;

    // 4. Complete signup via auth service
    let tokens = state
        .auth
        .signup(
            otp_response.otp_id,
            ProviderType::Phone,
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
        "Phone signup flow completed"
    );

    Ok(())
}

/// Demonstrates full email signup flow using OtpService
async fn email_signup_flow(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create and send OTP via OtpService (sends real email)
    let otp_response = state
        .otp
        .create_and_send(EMAIL, ProviderType::Email)
        .await?;
    info!(otp_id = %otp_response.otp_id, "OTP created for email signup");

    // 2. In production: user receives email and enters code
    info!("Skipping OTP verification (would need real code from email)");

    // 3. For demo purposes, manually mark as verified using db directly
    hops::db::otp::mark_verified(state.get_pool(), otp_response.otp_id).await?;

    // 4. Complete signup via auth service
    let tokens = state
        .auth
        .signup(
            otp_response.otp_id,
            ProviderType::Email,
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
        "Email signup flow completed"
    );

    Ok(())
}

async fn create_state() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database.url)
        .await?;

    let crypto = Arc::new(CryptoAdapter::new(CONFIG.crypto.secret.clone()));
    let comms = Arc::new(CommsAdapter::new(
        CONFIG.comms.username.clone(),
        CONFIG.comms.password.clone(),
        CONFIG.comms.resend_api_key.clone(),
    ));

    let auth = Arc::new(AuthService::new(
        CONFIG.auth.clone(),
        pool.clone(),
        crypto.clone(),
        CONFIG.auth.private_key_pem(),
        CONFIG.auth.public_key_pem(),
        &CONFIG.auth.audience,
        &CONFIG.auth.issuer,
    )?);

    let otp = Arc::new(OtpService::new(
        CONFIG.otp.clone(),
        CONFIG.comms.clone(),
        pool.clone(),
        comms,
        crypto,
    ));

    let state = AppState::new(pool, auth, otp);
    Ok(Arc::new(state))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let state = create_state().await?;

    info!("--- Phone OTP Flow ---");
    match phone_otp_flow(state.clone()).await {
        Ok(()) => info!("✅ Phone OTP flow successful"),
        Err(e) => info!("❌ Phone OTP flow failed: {}", e),
    }

    info!("--- Phone Signup Flow ---");
    match phone_signup_flow(state.clone()).await {
        Ok(()) => info!("✅ Phone signup flow successful"),
        Err(e) => info!("❌ Phone signup flow failed: {}", e),
    }

    info!("--- Email Signup Flow ---");
    match email_signup_flow(state.clone()).await {
        Ok(()) => info!("✅ Email signup flow successful"),
        Err(e) => info!("❌ Email signup flow failed: {}", e),
    }

    Ok(())
}
