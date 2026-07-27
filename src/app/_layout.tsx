import { useFonts } from "expo-font";
import { Slot, useRouter, useSegments } from "expo-router";
import { useEffect } from "react";

import { SessionProvider, useSession } from "@/features/auth";
import { LanguageProvider, ThemeProvider, ToastProvider } from "@/hooks";

const appFonts = {
  "SpaceGrotesk-SemiBold": require("../../assets/fonts/SpaceGrotesk-SemiBold.ttf"),
  "SpaceGrotesk-Bold": require("../../assets/fonts/SpaceGrotesk-Bold.ttf"),
  "Manrope-Regular": require("../../assets/fonts/Manrope-Regular.ttf"),
  "Manrope-SemiBold": require("../../assets/fonts/Manrope-SemiBold.ttf"),
  "Manrope-Bold": require("../../assets/fonts/Manrope-Bold.ttf"),
  "Manrope-ExtraBold": require("../../assets/fonts/Manrope-ExtraBold.ttf"),
};

function RootLayoutNav() {
  const { isAuthenticated, isLoading, hasLoggedInBefore } = useSession();
  const segments = useSegments();
  const router = useRouter();

  useEffect(() => {
    if (isLoading) return;

    const inAuthGroup = segments[0] === "(auth)";

    if (!isAuthenticated && !inAuthGroup) {
      router.replace(hasLoggedInBefore ? "/sign-in" : "/sign-up");
    }
    if (isAuthenticated && inAuthGroup) {
      router.replace("/");
    }
  }, [isAuthenticated, isLoading, segments]);

  if (isLoading) return null;

  return <Slot />;
}

export default function RootLayout() {
  const [fontsLoaded] = useFonts(appFonts);

  if (!fontsLoaded) return null;

  return (
    <ThemeProvider>
      <LanguageProvider>
        <ToastProvider>
          <SessionProvider>
            <RootLayoutNav />
          </SessionProvider>
        </ToastProvider>
      </LanguageProvider>
    </ThemeProvider>
  );
}
