import * as Device from "expo-device";
import { Platform } from "react-native";

const DEVICE_ID_KEY = "device_id";

function generateUUID(): string {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    const v = c === "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

async function getSecureStore() {
  if (Platform.OS === "web") return null;
  return await import("expo-secure-store");
}

export async function getDeviceId(): Promise<string> {
  if (Platform.OS === "web") {
    let id = localStorage.getItem(DEVICE_ID_KEY);
    if (!id) {
      id = generateUUID();
      localStorage.setItem(DEVICE_ID_KEY, id);
    }
    return id;
  }

  const SecureStore = await getSecureStore();
  let id = await SecureStore?.getItemAsync(DEVICE_ID_KEY);
  if (!id) {
    id = generateUUID();
    await SecureStore?.setItemAsync(DEVICE_ID_KEY, id);
  }
  return id;
}

export async function getDeviceName(): Promise<string> {
  if (Platform.OS === "web") {
    return navigator.userAgent.split(" ")[0] || "Web Browser";
  }

  const deviceName = Device.deviceName;
  if (deviceName) return deviceName;

  const modelName = Device.modelName;
  const osName = Device.osName;
  if (modelName && osName) return `${modelName} (${osName})`;
  if (modelName) return modelName;

  return Platform.OS === "ios" ? "iPhone" : "Android Device";
}
