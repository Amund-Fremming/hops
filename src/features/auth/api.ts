import type {
  AppErrorCode,
  CreateOtpRequest,
  LoginRequest,
  LogoutRequest,
  OtpResponse,
  RefreshTokenRequest,
  SessionDto,
  SignupRequest,
  TokenResponse,
  User,
  VerifyOtpRequest,
} from "./types";

const API_BASE = "http://localhost:3000"; // TODO: replace with production URL

export class ApiError extends Error {
  code?: AppErrorCode;
  status: number;

  constructor(status: number, message: string, code?: AppErrorCode) {
    super(message);
    this.status = status;
    this.code = code;
    this.name = "ApiError";
  }
}

async function handleResponse<T>(res: Response): Promise<T> {
  if (!res.ok) {
    let code: AppErrorCode | undefined;
    let message = "Request failed";

    try {
      const body = await res.json();
      if (body.code) {
        code = body.code as AppErrorCode;
        message = body.message || message;
      }
    } catch {
      // Response wasn't JSON
    }

    throw new ApiError(res.status, message, code);
  }

  const text = await res.text();
  if (!text) {
    return undefined as T;
  }

  return JSON.parse(text);
}

// OTP endpoints
export async function createOtp(req: CreateOtpRequest): Promise<OtpResponse> {
  const res = await fetch(`${API_BASE}/auth/otp`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  return handleResponse<OtpResponse>(res);
}

export async function verifyOtp(req: VerifyOtpRequest): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/otp/verify`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  return handleResponse<void>(res);
}

// Auth endpoints
export async function login(req: LoginRequest): Promise<TokenResponse> {
  const res = await fetch(`${API_BASE}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  return handleResponse<TokenResponse>(res);
}

export async function signup(req: SignupRequest): Promise<TokenResponse> {
  const res = await fetch(`${API_BASE}/auth/signup`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  return handleResponse<TokenResponse>(res);
}

export async function refreshTokens(
  req: RefreshTokenRequest,
): Promise<TokenResponse> {
  const res = await fetch(`${API_BASE}/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  return handleResponse<TokenResponse>(res);
}

// Protected endpoints
export async function listSessions(accessToken: string): Promise<SessionDto[]> {
  const res = await fetch(`${API_BASE}/auth/sessions`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
  });
  return handleResponse<SessionDto[]>(res);
}

export async function logout(
  accessToken: string,
  req: LogoutRequest,
): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/logout`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify(req),
  });
  return handleResponse<void>(res);
}

export async function getMe(accessToken: string): Promise<User> {
  const res = await fetch(`${API_BASE}/user/me`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
  });
  return handleResponse<User>(res);
}
