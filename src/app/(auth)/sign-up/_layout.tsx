import { Stack } from "expo-router";

import { SignupProvider } from "@/features/auth/signup-context";

export default function SignUpLayout() {
  return (
    <SignupProvider>
      <Stack
        screenOptions={{
          headerShown: false,
          animation: "slide_from_right",
          gestureEnabled: true,
        }}
      />
    </SignupProvider>
  );
}
