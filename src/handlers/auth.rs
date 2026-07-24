use std::sync::Arc;

use crate::{
    handlers::extract_user_agent,
    models::auth::{Claims, RefreshTokenRequest},
};
use axum::{
    Extension, Json, Router,
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::StatusCode;
use tracing::{error, info, warn};

use crate::{
    config::CONFIG,
    db,
    error::{OtpError, ServerError},
    models::{
        otp::{CreateOtpRequest, Otp, VerifyOtpRequest},
        user::{PhoneLoginRequest, PhoneSignupRequest},
    },
    state::AppState,
};

pub fn public_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login/phone", post(phone_login))
        .route("/signup/phone", post(phone_signup))
        .route("/refresh", post(refresh_tokens))
        .route("/otp", post(create_otp))
        .route("/otp/verify", post(verify_otp))
        .with_state(state)
}

pub fn protected_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sessions", get(list_sessions))
        .with_state(state)
}

async fn list_sessions(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, ServerError> {
    let user_id = claims.user_id();
    let sessions = db::auth::list_session_dtos(state.get_pool(), user_id).await?;
    Ok((StatusCode::OK, Json(sessions)))
}

async fn phone_login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<PhoneLoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let user_agent = extract_user_agent(&headers);
    let token_response = state
        .auth
        .phone_login(
            req.device_id,
            &req.device_name,
            user_agent,
            &req.phone_number,
            &req.password,
        )
        .await?;

    Ok((StatusCode::OK, Json(token_response)))
}

async fn phone_signup(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(req): Json<PhoneSignupRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let otp = db::otp::get_otp_by_phone_number(state.get_pool(), &req.phone_number).await?;
    let user_agent = extract_user_agent(&headers);

    let response = state
        .auth
        .phone_signup(
            otp.id,
            &req.device_name,
            user_agent,
            &req.given_name,
            &req.family_name,
            &req.password,
        )
        .await?;

    info!(
        user_id = %claims.user_id(),
        "Phone signup successful"
    );
    Ok((StatusCode::CREATED, Json(response)))
}

async fn create_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOtpRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let code = Otp::generate_code();
    let hash = state.crypto.hash(&code);
    let response = db::otp::create_otp(
        state.get_pool(),
        &req.phone_number,
        &hash,
        CONFIG.otp.ttl_minutes,
        CONFIG.otp.max_messages_per_day,
    )
    .await?;

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

    let otp = db::otp::get_otp_by_id(state.get_pool(), otp_id).await?;

    if otp.is_expired() {
        return Err(ServerError::Otp(OtpError::Expired));
    }

    if otp.is_max_attempts_exceeded(CONFIG.otp.max_attempts as i32) {
        return Err(ServerError::Otp(OtpError::MaxAttemptsExceeded));
    }

    let valid_code = state.crypto.verify(&req.code, &otp.hash);

    if !valid_code {
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

async fn refresh_tokens(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let response = state
        .auth
        .refresh_token(req.device_id, &req.refresh_token)
        .await?;

    Ok((StatusCode::OK, Json(response)))
}
