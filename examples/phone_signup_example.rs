use std::sync::Arc;

use hops::adapters::comms::CommsAdapter;
use hops::adapters::crypto::CryptoAdapter;
use hops::db;
use hops::db::otp::{create_otp, get_otp_by_id, mark_verified};
use hops::models::otp::Otp;
use hops::services::auth::AuthService;
use hops::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

// 🚨 These keys are just used in this example and should not be used elsewhere

const PHONE_NUMBER: &str = "+4741387142";
const SEND_REAL_SMS: bool = false;
const CRYPTO_SECRET: &str = "example-secret-key-do-not-use-in-production";

const PRIVATE_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCzMzWPk4RwZCfP
DneGzgAyt0XqCMXGe2Ev5z1HP7Hnf0FRKuDzlwGM0LqFejkIBUzQgPAadfzbw7Gs
DUM48RZCKAovRmsOlU2jdmpuIAnTqIVYRPZDuHN2obJa+wNTKsB3n969e+pv/tC6
hw0Q/kpYSXcd7ZrECdrlds1ezoVPTMSqnAdx08/N0e4jsjlf18Sueyj77fILE75t
fCsLw41uQzoMl0e0THSy4D7rLws4tTGWbJf3VYmlMb5PBXx3lkwS4VBilQvb9ptu
6pZEH0Unf+ZGxV4pZ/hb7rsVes2WRR69x7bDlCMCAKq+4RiwwIIXURSHJBOIRsHO
JdPA6i2PAgMBAAECggEAAqWBp6+g1Lz0BggTEcg9rnXfvdBOWnzNXYW5DbIDblew
zjw851nFf9O5/2Agy0HK4GmGsX27Bg8AUYY+bZmkX7n8wCNFpVguO/8v7AMOf8+F
WrjruOoThtjTx3jpslklBnwzl5Ly/w0l/lfMZhpNOClJ+eZXpW/FcTvHVkD849ZO
OzEjwz81li5zL7psttC2simeThTMr8NGW8bCKHY8YncdYTt2iYBZoO/Jp6VKYMMn
H/lfYyFQkdnVAJTxQWQ1/sE3m1TwEkHjxZQe1EgUxnDtRXWoOVyxM8JCB4HXYfix
d2hXnurnXK+vtgmzlkZyqphSUgCceHo2GYl7gKuz4QKBgQD2y3PC1VEm6RxR5/Nw
DCjJNZg9XmGySge1gM7NFfr/Bg2FyVMaiiMrpkJLUNvcSm6Umje327+wvtJy20Q2
FFzIvOL1s7EaobOcuiHfOxlsz5F6b7/uR8QmG8U6hEfYUEV5yt2yTAh7CR63atzf
IQKx4fiFdufA1wmPaQwHm2h44QKBgQC54lJAvkKFAfIDXVg1f6pgOJMyOhn3ctEl
ceoF6CCClXUhKXTkV+rUfqYyLHEKM8rL18UTHEzbTknIkzAY83Cwt/IN1ZVGNhJR
8UMoGFsUVxpyluMvyvp7srL7gzinrLdQoukqhF34JauXpXw8IxTQ9YLjTSaQ42al
Qj3AhMJEbwKBgEj7z8cdeHtOUs6yDp7jKaifTd9QKwojtHXrmryxtGF4s8UNzaK6
mT4OU+qcBfj2lg8iMDoSJXUqaWgICfsIOIwwt9m7gzOCAHDn5p5yhslT9QzFQXhB
BvPSIJh2iByjWHh1EuzoaVWhU9EgLCNcSsS6M9mcWVsA/NXJVgJl5hZhAoGAfhjj
x3vJ0ETTkii+b/xc7c0zPX1gpBZFfutZ4AvqEeule4uN+mERsnj/8UVooY0k40dK
L36hPJxNPT1sAWETby45i9z52JlRsDjEX+y1zISSMm3dTEybw1IkTK5lvolSCeeZ
2PfWb0HOt57ROlJqCp6h3eQ2Z098EFtxXKoyxw0CgYB9S7EiTLTsuco3YeceN1p0
S9knEAv2uqVvo36tNDQTl3knddiuo4jlCu0oxEowPcEgQRzewEKNyyipa0Erzfjs
6ch33JvR36/3u2PDdpai5BlQ8tBaKmD6XcgQuFccq6dePCzS87BV86KFbm6duPaK
EOOp4KFL7ImDF2WAX+n5RA==
-----END PRIVATE KEY-----"#;

const PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAszM1j5OEcGQnzw53hs4A
MrdF6gjFxnthL+c9Rz+x539BUSrg85cBjNC6hXo5CAVM0IDwGnX828OxrA1DOPEW
QigKL0ZrDpVNo3ZqbiAJ06iFWET2Q7hzdqGyWvsDUyrAd5/evXvqb/7QuocNEP5K
WEl3He2axAna5XbNXs6FT0zEqpwHcdPPzdHuI7I5X9fErnso++3yCxO+bXwrC8ON
bkM6DJdHtEx0suA+6y8LOLUxlmyX91WJpTG+TwV8d5ZMEuFQYpUL2/abbuqWRB9F
J3/mRsVeKWf4W+67FXrNlkUevce2w5QjAgCqvuEYsMCCF1EUhyQTiEbBziXTwOot
jwIDAQAB
-----END PUBLIC KEY-----"#;

async fn otp_flow_successful_code(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let otp_response = create_otp(state.get_pool(), PHONE_NUMBER, &hash).await?;

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
    let otp_response = create_otp(state.get_pool(), PHONE_NUMBER, &hash).await?;

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
    let otp = db::otp::create_otp(state.get_pool(), PHONE_NUMBER, &hash).await?;

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
    let (user_id, tokens) = state
        .auth
        .phone_signup(otp.otp_id, "Test", "User", "SecurePassword123!")
        .await?;

    info!(
        user_id = %user_id,
        access_token_len = tokens.access_token.len(),
        refresh_token_len = tokens.refresh_token.len(),
        expires_in = tokens.expires_in,
        "Full signup flow completed"
    );

    Ok(())
}

async fn create_state() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let crypto = Arc::new(CryptoAdapter::new(
        std::env::var("CRYPTO_SECRET").unwrap_or_else(|_| CRYPTO_SECRET.to_string()),
    ));

    let auth = AuthService::new(
        pool.clone(),
        crypto.clone(),
        PRIVATE_KEY_PEM,
        PUBLIC_KEY_PEM,
        "hops-api",
        "https://hops.example.com",
    )?;

    let comms = CommsAdapter::new(
        std::env::var("ELKS_USERNAME").unwrap_or_default(),
        std::env::var("ELKS_PASSWORD").unwrap_or_default(),
    );

    let state = AppState::new(pool, Arc::new(auth), Arc::new(comms), crypto);
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
