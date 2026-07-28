// Backend error codes for localized messages
export type AppErrorCode =
  | "OTP_EXPIRED"
  | "OTP_WRONG_CODE"
  | "OTP_ALREADY_VERIFIED"
  | "OTP_NOT_FOUND"
  | "OTP_MAX_ATTEMPTS"
  | "OTP_MAX_MESSAGES"
  | "SMS_FAILED"
  | "INVALID_CREDENTIALS"
  | "VALIDATION_FAILED"
  | "PASSWORD_SAME_AS_OLD";

export interface AppError {
  code: AppErrorCode;
  message: string;
}

export type ProviderType = "phone" | "email";

// Request types
export interface CreateOtpRequest {
  identifier: string;
  provider_type: ProviderType;
}

export interface VerifyOtpRequest {
  otp_id: string;
  code: string;
}

export interface LoginRequest {
  device_id: string;
  device_name: string;
  provider_type: ProviderType;
  identifier: string;
  password: string;
}

export interface SignupRequest {
  provider_type: ProviderType;
  device_name: string;
  identifier: string;
  password: string;
  given_name: string;
  family_name: string;
}

export interface RefreshTokenRequest {
  device_id: string;
  refresh_token: string;
}

export interface LogoutRequest {
  device_id: string;
}

// Response types
export interface OtpResponse {
  otp_id: string;
}

export interface TokenResponse {
  access_token: string;
  refresh_token: string;
  access_expires_in: number;
  refresh_expires_in: number;
}

export interface User {
  id: string;
  phone_number: string | null;
  phone_number_verified: boolean;
  email: string | null;
  email_verified: boolean;
  given_name: string;
  family_name: string;
  avatar_url: string | null;
  created_at: string;
  updated_at: string;
  last_active_at: string;
}

export interface SessionDto {
  device_id: string;
  device_name: string;
  user_agent: string | null;
  active: boolean;
}

// Session types
export interface AuthSession {
  accessToken: string;
  accessExpiresAt: number;
  refreshToken: string;
  refreshExpiresAt: number;
  deviceId: string;
}

export function tokenResponseToSession(
  res: TokenResponse,
  deviceId: string,
): AuthSession {
  const now = Date.now();
  return {
    accessToken: res.access_token,
    accessExpiresAt: now + res.access_expires_in * 1000,
    refreshToken: res.refresh_token,
    refreshExpiresAt: now + res.refresh_expires_in * 1000,
    deviceId,
  };
}

// Error code to translation key mapping
export const errorCodeToTranslationKey: Record<string, string> = {
  OTP_EXPIRED: "otp.code_expired",
  OTP_WRONG_CODE: "otp.invalid_code",
  OTP_ALREADY_VERIFIED: "otp.already_verified",
  OTP_NOT_FOUND: "otp.not_found",
  OTP_MAX_ATTEMPTS: "otp.too_many_attempts",
  OTP_MAX_MESSAGES: "otp.too_many_messages",
  SMS_FAILED: "otp.send_failed",
  INVALID_CREDENTIALS: "login.error_invalid_credentials",
  VALIDATION_FAILED: "error.validation_failed",
  PASSWORD_SAME_AS_OLD: "error.password_same_as_old",
};
