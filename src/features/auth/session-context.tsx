import type { ReactNode } from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";

import { login as loginApi, logout as logoutApi } from "./api";
import { getDeviceId, getDeviceName } from "./device";
import {
  clearSession,
  getHasLoggedInBefore,
  getStoredSession,
  setHasLoggedInBefore,
  storeSession,
} from "./token-storage";
import {
  tokenResponseToSession,
  type AuthSession,
  type ProviderType,
} from "./types";

interface SessionContextValue {
  isAuthenticated: boolean;
  isLoading: boolean;
  hasLoggedInBefore: boolean;
  login: (
    identifier: string,
    password: string,
    providerType: ProviderType,
  ) => Promise<void>;
  logout: () => Promise<void>;
}

const SessionContext = createContext<SessionContextValue | null>(null);

export function SessionProvider({ children }: { children: ReactNode }) {
  const [session, setSession] = useState<AuthSession | null>(null);
  const [hasLoggedInBefore, setHasLoggedInBeforeState] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    Promise.all([getStoredSession(), getHasLoggedInBefore()])
      .then(([storedSession, hasLogged]) => {
        setSession(storedSession);
        setHasLoggedInBeforeState(hasLogged);
      })
      .finally(() => setIsLoading(false));
  }, []);

  const login = async (
    identifier: string,
    password: string,
    providerType: ProviderType,
  ) => {
    const deviceId = await getDeviceId();
    const deviceName = await getDeviceName();

    const res = await loginApi({
      device_id: deviceId,
      device_name: deviceName,
      provider_type: providerType,
      identifier,
      password,
    });

    const newSession = tokenResponseToSession(res, deviceId);
    await storeSession(newSession);
    await setHasLoggedInBefore();
    setSession(newSession);
    setHasLoggedInBeforeState(true);
  };

  const logout = async () => {
    if (session) {
      await logoutApi(session.accessToken, {
        device_id: session.deviceId,
      }).catch(() => {});
    }
    await clearSession();
    setSession(null);
  };

  const isAuthenticated = useMemo(() => {
    if (!session) return false;
    return Date.now() < session.refreshExpiresAt;
  }, [session]);

  const value = useMemo<SessionContextValue>(
    () => ({
      isAuthenticated,
      isLoading,
      hasLoggedInBefore,
      login,
      logout,
    }),
    [isAuthenticated, isLoading, hasLoggedInBefore],
  );

  return (
    <SessionContext.Provider value={value}>{children}</SessionContext.Provider>
  );
}

export function useSession() {
  const context = useContext(SessionContext);
  if (!context) {
    throw new Error("useSession must be used within SessionProvider");
  }
  return context;
}
