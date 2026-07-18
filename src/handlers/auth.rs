use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use reqwest::StatusCode;
use tracing::{error, info, warn};

use crate::{
    config::CONFIG,
    db,
    error::{OtpError, ServerError},
    models::{
        otp::{CreateOtpRequest, Otp, OtpResponse, VerifyOtpRequest},
        user::{PhoneLoginRequest, PhoneSignupRequest},
    },
    state::AppState,
};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login/phone", post(phone_login))
        .route("/signup/phone", post(phone_signup))
        .route("/otp", post(create_otp))
        .route("/otp/verify", post(verify_otp))
        .with_state(state)
}

async fn phone_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PhoneLoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let token_response = state
        .auth
        .phone_login(&req.phone_number, &req.password)
        .await?;

    Ok((StatusCode::OK, Json(token_response)))
}

async fn phone_signup(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<PhoneSignupRequest>,
) -> Result<impl IntoResponse, ServerError> {
    todo!();

    // cannot exist any user identity phone with credential phone number to the incoming
    // verify that there exists a verified otp for this phone number
    //

    Ok(())
}

async fn create_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOtpRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let response = db::otp::create_otp(state.get_pool(), &req.phone_number, &hash).await?;

    let from = CONFIG.comms.from.clone();
    let message = &CONFIG.comms.otp_message_template.replace("{code}", &code);

    if let Err(e) = state
        .comms
        .send_sms(&from, &req.phone_number, message)
        .await
    {
        error!(
            otp_id = %response.otp_id,
            phone_number = %req.phone_number,
            error = %e,
            "Failed to send OTP, deleting entry"
        );
        db::otp::delete_otp(state.get_pool(), response.otp_id).await?;

        return Err(ServerError::Otp(OtpError::SmsFailed));
    }

    info!(
        phone_number = %req.phone_number,
        "Created otp entry"
    );

    Ok((StatusCode::OK, Json(response)))
}

// TODO - find subsctitute for ip address field on otp, moviles on 5g have misleading ip
async fn verify_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let otp_id = req.otp_id;

    let otp = db::otp::get_valid_otp(state.get_pool(), otp_id).await?;
    let is_valid = state.crypto.verify(&req.code, &otp.hash);

    if !is_valid {
        warn!(
            otp_id = %otp_id,
            code = %req.code,
            phone_number = %otp.phone_number,
            "Invalid code for OTP"
        );

        tokio::spawn(async move {
            if let Err(e) = db::otp::increment_failed_attempts(state.get_pool(), otp_id).await {
                error!(
                    otp_id = %otp_id,
                    code = %req.code,
                    phone_number = %otp.phone_number,
                    error = %e,
                    "Failed to increment failed OTP code"
                );
            };
        });

        // TODO - increment asyn failed attampts
        /*

                ip_address field exists in the OTP table but isn't populated during creation
        OTP response doesn't include expires_at (helpful for client-side countdown) */

        return Err(ServerError::Otp(OtpError::WrongCode));
    }

    if let Err(e) = db::otp::mark_verified(state.get_pool(), req.otp_id).await {
        error!(
            otp_id = %req.otp_id,
            code = %req.code,
            phone_number = %otp.phone_number,
            "Failed to mark OTP as verified, phone should be manually verified"
        );
        return Err(e.into());
    }

    Ok(StatusCode::OK)
}
