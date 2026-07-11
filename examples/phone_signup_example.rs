use std::sync::Arc;

use hops::adapters::comms::CommsAdapter;
use hops::db::otp::{create_otp, verify_otp};
use hops::services::auth::AuthService;
use hops::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

// 🚨 These keys are just used in this example and should not be used elsewhere

const PHONE_NUMBER: &str = "+4741387142";
const SEND_REAL_SMS: bool = false;

const PRIVATE_KEY_PEM: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCeDvL4uflGzB+O
jhu7yBd+cTFPVQ8LR0IYx0kD2TAfQO8mnuAUZOBxenva1taQyYpJMhK9x50b1s5C
UNvMSCxaIxH/WvpRhQbv6fCKjgFQIyfHM45pJdHMCcy+LYHs7NiTgLc+KT1O2l7Q
rDyfUuFGTABAYfy8HxxgA4wHN5AbQBNCyvcytQwUAgaNYKG4WcPfcmzRH34nGZgL
9BawcFiXEFUK38aJDCxoW9nt2PVBA35AKyd/vdecdxlCV6WXFp3voY7mWHe/+X7T
xuue7vxkiE5Pw5xMRMcDlBoYddKx9MBF7lqJ4VPT8KyPBLG8FyFhJX1ue0BQ0TQv
JuONXsTVAgMBAAECgf9UYPv/zJ3x2FxVvjJlYmx9vpqUaimd28IYIvYtF8VDlLwX
N2Ro1BxTIxBRGQrIQM+SQ9O9fNMNXvS9x8I59vhhJlfKpXGnaLJLYe2ytMdUAMXm
PPUfLRF9271xyYQ9ojNR4LqdH2bwsaNBw1vvB6U74gGVsrnkXrdx38g2vMgxwyv5
ObXTXlRixm6Klg16fPWqc+7XnP20WU31K+6txRlc5j03DFWy5M+99l1Kt9i8OQGb
trD6SYS4Gm676juqt5tKFtz1dIMcRXSsPz9eWmeFpMRJ09EU9shZODkBx5/WG/1B
N/VtQtftFlVNX54kKPNaGUsFZY2V5C+n207wUKkCgYEA1nCAw/mMh6Vhza5uD6Ms
tqE+6nUHQ9cMz799RsyDWw4kOL0q8mS213D8P/YIL7EyTyf9TWaq3FFNbeIWngri
7u4ZqQGoh4MYZvcQ38lDtLhVygi9H1DCv2c0/IKnuHclZMHhOsn2XigKps8hMrv1
XG/G3mlJcG62dpxp8IdpZU8CgYEAvLET2JdLMTlbQaTYe3vP0kMMTLvaGd+lvbv2
ZMaDQpfpHBbYo0n+Nc32gSP+F8Gjdpd0eKWaK1wbBjekXn8uzPbch4UvDmXeyKtR
C05MzEb0yRZ1bsqXg0dzRl7aqA+Z/I0HGstelwmv1tCsE0P7gmxILwemHzbQPcwa
DQGrMpsCgYAeH1OXM9jPvSWN9PC09aD0TpY97Q6GMxEzpZx9c4EIK2ZfKgN8ZTVh
8hcdDPx9ZpDAmcd1NfTOWgVcaCPxM2pJUdz85qS71Gh7Hj2akfUWz8YNSUj3uyqA
JIlG5zuUJ/hyvOFclr4q38kPQY1SSSDgSTtQRs3wIz0yUCp5hSwC9QKBgQCqBq4p
Zvr8WgCfABmJ+6DiiEQXCNaYpexFMY/ucupoIVaOVw/S46PLe9H5wCL/6R6QiB0N
cbugApjfW1gjRls3meJRw3MJeEXtcGHQ3Ddbgzyjzjb3JFqukr2O1X4WHijVZ4bV
YBfV5Yaq/NFxcrq5ZTUOG8hXLB8s8DMxMSXArQKBgQDMg7DrBwtNjvDHIZZ/v690
9AvHh1fUZHYETCEPheiJzv2rb+P/65qGRSIhgjfFvzBDcCRrw4r/baCcgSXzoM6s
zc3yRLOJZMGUCmP4Qekwfj11NYgdrIOFosMLCv91dHlJHSLNom8VQuZxZJNn0eTN
R1JiU7mesME6ZVBFUwCY7A==
-----END RSA PRIVATE KEY-----"#;

const PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAng7y+Ln5Rswfjo4bu8gX
fnExT1UPC0dCGMdJA9kwH0DvJp7gFGTgcXp72tbWkMmKSTISvcedG9bOQlDbzEgs
WiMR/1r6UYUG7+nwio4BUCMnxzOOaSXRzAnMvi2B7OzYk4C3Pik9Ttpe0Kw8n1Lh
RkwAQGH8vB8cYAOMBzeQG0ATQsr3MrUMFAIGjWChuFnD33Js0R9+JxmYC/QWsHBY
lxBVCt/GiQwsaFvZ7dj1QQN+QCsnf73XnHcZQlellxad76GO5lh3v/l+08brnu78
ZIhOT8OcTETHA5QaGHXSsfTARe5aieFT0/CsjwSxvBchYSV9bntAUNE0LybjjV7E
1QIDAQAB
-----END PUBLIC KEY-----"#;

async fn opt_flow_successfull_code(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let login_object = create_otp(&state.pool, PHONE_NUMBER).await?;

    if SEND_REAL_SMS {
        state
            .comms
            .send_sms(
                "Hops",
                PHONE_NUMBER,
                &format!("Your login code is: {}", login_object.code),
            )
            .await?;
    }

    verify_otp(&state.pool, PHONE_NUMBER, &login_object.code).await?;
    Ok(())
}

async fn opt_flow_failed_code(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let login_object = create_otp(&state.pool, PHONE_NUMBER).await?;

    if SEND_REAL_SMS {
        state
            .comms
            .send_sms(
                "Hops",
                PHONE_NUMBER,
                &format!("Your login code is: {}", login_object.code),
            )
            .await?;
    }

    verify_otp(&state.pool, PHONE_NUMBER, "000000").await?;
    Ok(())
}

async fn create_state() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let auth = AuthService::new(
        pool.clone(),
        PRIVATE_KEY_PEM,
        PUBLIC_KEY_PEM,
        "hops-api",
        "https://hops.example.com",
    )?;

    let comms = CommsAdapter::new(
        std::env::var("ELKS_USERNAME").unwrap_or_default(),
        std::env::var("ELKS_PASSWORD").unwrap_or_default(),
    );

    let state = AppState::new(pool, Arc::new(auth), Arc::new(comms));
    Ok(Arc::new(state))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let state = create_state().await?;

    match opt_flow_successfull_code(state.clone()).await {
        Ok(()) => info!("✅ OTP flow successfull"),
        Err(e) => info!("❌ OTP flow failed: {}", e),
    }

    match opt_flow_failed_code(state.clone()).await {
        Ok(()) => info!("❌ OTP flow was correct, should fail"),
        Err(..) => info!("✅ OTP flow failed successfull"),
    }

    // TODO
    // - expired otp does not work
    // - max attempts exceeded fails
    // - max entries per day fails

    Ok(())
}
