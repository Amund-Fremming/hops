import { KeyboardAvoidingView, Platform } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { usePalette } from "@/hooks";

import { OtpFlow } from "../../components";
import type { OtpFlowResult } from "../../components/OtpFlow/OtpFlow";
import { styles } from "./styles";

export default function SignUpScreen() {
  const palette = usePalette();

  const handleComplete = (result: OtpFlowResult) => {
    // TODO: create the account via API and start a session
    console.log("signup complete", result.method, result.identifier);
  };

  return (
    <SafeAreaView style={[styles.container, { backgroundColor: palette.bg }]}>
      <KeyboardAvoidingView
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        style={styles.container}
      >
        <OtpFlow onComplete={handleComplete} />
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}
