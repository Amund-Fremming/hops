const API_BASE = "https://api.example.com"; // TODO: replace with actual API

export interface LoginResponse {
  accessToken: string;
  accessTokenExpiresAt: number;
  refreshToken: string;
  refreshTokenExpiresAt: number;
  userId: string;
}

export interface RefreshResponse {
  accessToken: string;
  accessTokenExpiresAt: number;
  refreshToken: string;
  refreshTokenExpiresAt: number;
}

export async function loginRequest(
  identifier: string,
  password: string,
  deviceId: string,
): Promise<LoginResponse> {
  const res = await fetch(`${API_BASE}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ identifier, password, deviceId }),
  });

  if (!res.ok) {
    throw new Error("Login failed");
  }

  return res.json();
}

export async function refreshRequest(
  refreshToken: string,
  deviceId: string,
): Promise<RefreshResponse> {
  const res = await fetch(`${API_BASE}/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ refreshToken, deviceId }),
  });

  if (!res.ok) {
    throw new Error("Refresh failed");
  }

  return res.json();
}

export async function logoutRequest(
  accessToken: string,
  deviceId: string,
): Promise<void> {
  await fetch(`${API_BASE}/auth/logout`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify({ deviceId }),
  });
}
