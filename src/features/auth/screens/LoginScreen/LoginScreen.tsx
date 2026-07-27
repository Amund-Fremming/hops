import { useRouter } from "expo-router";
import { useState } from "react";
import {
  KeyboardAvoidingView,
  Platform,
  Pressable,
  Text,
  TextInput,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { LanguagePicker } from "@/components";
import { usePalette, useToast, useTranslation } from "@/hooks";

import { styles } from "./styles";

type Method = "phone" | "email";
const PHONE_PREFIX = "+47";
const DEMO_PASSWORD = "hops123";

export default function LoginScreen() {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();
  const { t } = useTranslation();
  const router = useRouter();

  const [method, setMethod] = useState<Method>("phone");
  const [value, setValue] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.border,
    color: palette.text,
  };

  const handleSwitchMethod = () => {
    setMethod((m) => (m === "phone" ? "email" : "phone"));
    setValue("");
  };

  const handleLogin = () => {
    const trimmed = value.trim();
    const isValid =
      method === "phone"
        ? trimmed.replace(/\D/g, "").length >= 8
        : /^\S+@\S+\.\S+$/.test(trimmed);

    if (!isValid) {
      showError(
        method === "phone" ? t("otp.invalid_phone") : t("otp.invalid_email"),
      );
      return;
    }

    if (!password) {
      showError(t("login.error_password_required"));
      return;
    }

    if (password !== DEMO_PASSWORD) {
      showError(t("login.error_invalid_credentials"));
      return;
    }

    showSuccess(t("login.success"));
    // TODO: call login API and start session
  };

  return (
    <SafeAreaView style={[styles.container, { backgroundColor: palette.bg }]}>
      <KeyboardAvoidingView
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        style={styles.keyboardView}
      >
        <View style={styles.content}>
          <LanguagePicker style={styles.languagePicker} />

          <Text style={[styles.title, { color: palette.text }]}>
            {t("login.title")}
          </Text>
          <Text style={[styles.subtitle, { color: palette.textMuted }]}>
            {t("login.subtitle")}
          </Text>

          <View style={styles.labelRow}>
            <Text style={[styles.fieldLabel, { color: palette.textFaint }]}>
              {method === "phone" ? t("otp.label_phone") : t("otp.label_email")}
            </Text>
            <Pressable onPress={handleSwitchMethod} hitSlop={8}>
              <Text
                style={[
                  styles.switchText,
                  styles.linkUnderline,
                  { color: palette.textMuted },
                ]}
              >
                {method === "phone" ? t("otp.use_email") : t("otp.use_phone")}
              </Text>
            </Pressable>
          </View>

          {method === "phone" ? (
            <View style={styles.phoneRow}>
              <View
                style={[
                  styles.prefixPill,
                  {
                    backgroundColor: palette.inputBg,
                    borderColor: palette.border,
                  },
                ]}
              >
                <Text style={[styles.prefixText, { color: palette.textMuted }]}>
                  {PHONE_PREFIX}
                </Text>
              </View>
              <TextInput
                style={[styles.input, styles.rowInput, inputTheme]}
                value={value}
                onChangeText={setValue}
                placeholder={t("login.placeholder_phone")}
                placeholderTextColor={palette.textFaint}
                keyboardType="phone-pad"
                autoComplete="tel"
              />
            </View>
          ) : (
            <TextInput
              style={[styles.input, inputTheme]}
              value={value}
              onChangeText={setValue}
              placeholder={t("otp.placeholder_email")}
              placeholderTextColor={palette.textFaint}
              keyboardType="email-address"
              autoCapitalize="none"
              autoComplete="email"
            />
          )}

          <View style={styles.passwordWrapper}>
            <TextInput
              style={[styles.input, styles.passwordInput, inputTheme]}
              value={password}
              onChangeText={setPassword}
              placeholder={t("signup.password")}
              placeholderTextColor={palette.textFaint}
              secureTextEntry={!showPassword}
              autoCapitalize="none"
              autoComplete="password"
            />
            <Pressable
              onPress={() => setShowPassword((s) => !s)}
              hitSlop={8}
              style={styles.eyeButton}
            >
              <EyeIcon open={showPassword} color={palette.textMuted} />
            </Pressable>
          </View>

          <Pressable hitSlop={8} style={styles.forgotLink}>
            <Text
              style={[
                styles.linkText,
                styles.linkUnderline,
                { color: palette.textMuted },
              ]}
            >
              {t("login.forgot_password")}
            </Text>
          </Pressable>

          <Text style={[styles.demoTip, { color: palette.textFaint }]}>
            {t("login.demo_tip")}
          </Text>

          <Pressable
            onPress={handleLogin}
            style={({ pressed }) => [
              styles.primaryButton,
              { backgroundColor: palette.accent, opacity: pressed ? 0.85 : 1 },
            ]}
          >
            <Text
              style={[styles.primaryButtonText, { color: palette.accentText }]}
            >
              {t("login.button")}
            </Text>
          </Pressable>

          <View style={styles.toggleRow}>
            <Text style={[styles.toggleText, { color: palette.textMuted }]}>
              {t("auth.no_account")}{" "}
            </Text>
            <Pressable onPress={() => router.replace("/sign-up")} hitSlop={8}>
              <Text
                style={[
                  styles.linkText,
                  styles.linkUnderline,
                  { color: palette.textMuted },
                ]}
              >
                {t("auth.sign_up")}
              </Text>
            </Pressable>
          </View>
        </View>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

function EyeIcon({ open, color }: { open: boolean; color: string }) {
  return (
    <View style={styles.eyeIcon}>
      <View style={[styles.eyeOutline, { borderColor: color }]} />
      <View style={[styles.eyePupil, { borderColor: color }]} />
      {!open && <View style={[styles.eyeSlash, { backgroundColor: color }]} />}
    </View>
  );
}
