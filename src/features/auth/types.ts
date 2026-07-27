export interface AuthSession {
  accessToken: string;
  accessTokenExpiresAt: number;
  refreshToken: string;
  refreshTokenExpiresAt: number;
  userId: string;
  deviceId: string;
}
