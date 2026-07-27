import { Platform } from "react-native";

import type { AuthSession } from "./types";

const SESSION_KEY = "auth_session";

async function getSecureStore() {
  if (Platform.OS === "web") return null;
  return await import("expo-secure-store");
}

export async function storeSession(session: AuthSession): Promise<void> {
  const json = JSON.stringify(session);

  if (Platform.OS === "web") {
    localStorage.setItem(SESSION_KEY, json);
    return;
  }

  const SecureStore = await getSecureStore();
  await SecureStore?.setItemAsync(SESSION_KEY, json);
}

export async function getStoredSession(): Promise<AuthSession | null> {
  try {
    let json: string | null = null;

    if (Platform.OS === "web") {
      json = localStorage.getItem(SESSION_KEY);
    }
    if (Platform.OS !== "web") {
      const SecureStore = await getSecureStore();
      json = (await SecureStore?.getItemAsync(SESSION_KEY)) ?? null;
    }

    if (!json) return null;
    return JSON.parse(json) as AuthSession;
  } catch {
    return null;
  }
}

export async function clearSession(): Promise<void> {
  if (Platform.OS === "web") {
    localStorage.removeItem(SESSION_KEY);
    return;
  }

  const SecureStore = await getSecureStore();
  await SecureStore?.deleteItemAsync(SESSION_KEY);
}

const HAS_LOGGED_IN_KEY = "has_logged_in_before";

export async function setHasLoggedInBefore(): Promise<void> {
  if (Platform.OS === "web") {
    localStorage.setItem(HAS_LOGGED_IN_KEY, "true");
    return;
  }

  const SecureStore = await getSecureStore();
  await SecureStore?.setItemAsync(HAS_LOGGED_IN_KEY, "true");
}

export async function getHasLoggedInBefore(): Promise<boolean> {
  if (Platform.OS === "web") {
    return localStorage.getItem(HAS_LOGGED_IN_KEY) === "true";
  }

  const SecureStore = await getSecureStore();
  const value = await SecureStore?.getItemAsync(HAS_LOGGED_IN_KEY);
  return value === "true";
}
