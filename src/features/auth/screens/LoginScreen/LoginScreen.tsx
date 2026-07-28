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
import { ApiError } from "@/features/auth/api";
import { useSession } from "@/features/auth/session-context";
import { errorCodeToTranslationKey } from "@/features/auth/types";
import { usePalette, useToast, useTranslation } from "@/hooks";

import { styles } from "./styles";

export default function LoginScreen() {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();
  const { t } = useTranslation();
  const router = useRouter();
  const { login } = useSession();

  const [identifier, setIdentifier] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.border,
    color: palette.text,
  };

  const handleLogin = async () => {
    const trimmed = identifier.trim();
    const isPhone = /^\+?\d[\d\s-]{6,}$/.test(trimmed);
    const isEmail = /^\S+@\S+\.\S+$/.test(trimmed);

    if (!isPhone && !isEmail) {
      showError(t("login.error_invalid_input"));
      return;
    }

    if (!password) {
      showError(t("login.error_password_required"));
      return;
    }

    const providerType = isEmail ? "email" : "phone";

    setIsLoading(true);
    try {
      await login(trimmed, password, providerType);
      showSuccess(t("login.success"));
      router.replace("/");
    } catch (err) {
      if (err instanceof ApiError && err.code) {
        const key = errorCodeToTranslationKey[err.code];
        showError(key ? t(key) : err.message);
      } else {
        showError(t("login.error_invalid_credentials"));
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <SafeAreaView style={[styles.container, { backgroundColor: palette.bg }]}>
      <KeyboardAvoidingView
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        style={styles.keyboardView}
      >
        <View style={styles.content}>
          <View style={styles.headerRow}>
            <LanguagePicker style={styles.languagePickerAbsolute} />
          </View>

          <Text style={[styles.title, { color: palette.text }]}>
            {t("login.title")}
          </Text>
          <Text style={[styles.subtitle, { color: palette.textMuted }]}>
            {t("login.subtitle")}
          </Text>

          <Text style={[styles.fieldLabel, { color: palette.textFaint }]}>
            {t("login.label_phone_or_email")}
          </Text>

          <TextInput
            style={[styles.input, inputTheme]}
            value={identifier}
            onChangeText={setIdentifier}
            placeholder={t("login.placeholder_phone_or_email")}
            placeholderTextColor={palette.textFaint}
            keyboardType="email-address"
            autoCapitalize="none"
            autoComplete="username"
          />

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

          <Pressable
            onPress={handleLogin}
            disabled={isLoading}
            style={({ pressed }) => [
              styles.primaryButton,
              {
                backgroundColor: palette.accent,
                opacity: isLoading ? 0.6 : pressed ? 0.85 : 1,
              },
            ]}
          >
            <Text
              style={[styles.primaryButtonText, { color: palette.accentText }]}
            >
              {isLoading ? "..." : t("login.button")}
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
