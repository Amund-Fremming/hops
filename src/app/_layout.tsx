import { Stack } from "expo-router";

// TODO: wrap (app) in <Stack.Protected guard={!!session}> once session-context is implemented.
export default function RootLayout() {
  return <Stack screenOptions={{ headerShown: false }} />;
}
