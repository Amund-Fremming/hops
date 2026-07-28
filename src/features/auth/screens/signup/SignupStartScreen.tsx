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
import { ApiError, createOtp } from "@/features/auth/api";
import { useSignup } from "@/features/auth/signup-context";
import { errorCodeToTranslationKey } from "@/features/auth/types";
import { usePalette, useToast, useTranslation } from "@/hooks";

import { styles } from "./styles";

const PHONE_PREFIX = "+47";

export default function SignupStartScreen() {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();
  const { t } = useTranslation();
  const router = useRouter();
  const signup = useSignup();

  const [value, setValue] = useState(signup.identifier);
  const [isLoading, setIsLoading] = useState(false);

  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.border,
    color: palette.text,
  };

  const handleSwitchMethod = () => {
    signup.setMethod(signup.method === "phone" ? "email" : "phone");
    setValue("");
  };

  const handleContinue = async () => {
    const trimmed = value.trim();
    const isValid =
      signup.method === "phone"
        ? /^\d{8,}$/.test(trimmed)
        : /^\S+@\S+\.\S+$/.test(trimmed);

    if (!isValid) {
      showError(
        signup.method === "phone"
          ? t("otp.invalid_phone")
          : t("otp.invalid_email"),
      );
      return;
    }

    const identifier =
      signup.method === "phone" ? `${PHONE_PREFIX}${trimmed}` : trimmed;

    setIsLoading(true);
    try {
      const res = await createOtp({
        identifier,
        provider_type: signup.method,
      });

      signup.setIdentifier(identifier);
      signup.setOtpId(res.otp_id);
      showSuccess(t("otp.code_sent"));
      router.push("/sign-up/otp");
    } catch (err) {
      if (err instanceof ApiError && err.code) {
        const key = errorCodeToTranslationKey[err.code];
        showError(key ? t(key) : err.message);
      } else {
        showError(t("error.unknown"));
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <SafeAreaView style={[styles.container, { backgroundColor: palette.bg }]}>
      <KeyboardAvoidingView
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        style={styles.inner}
      >
        <View style={styles.headerRow}>
          <View style={styles.dotsRow}>
            {[0, 1, 2].map((i) => (
              <View
                key={i}
                style={[
                  styles.dot,
                  {
                    backgroundColor: i === 0 ? palette.accent : palette.border,
                    width: i === 0 ? 22 : 6,
                  },
                ]}
              />
            ))}
          </View>
          <LanguagePicker style={styles.languagePickerAbsolute} />
        </View>

        <Text style={[styles.title, { color: palette.text }]}>
          {t("otp.title")}
        </Text>
        <Text style={[styles.subtitle, { color: palette.textMuted }]}>
          {t("otp.subtitle")}
        </Text>
        <Text style={[styles.fieldLabel, { color: palette.textFaint }]}>
          {signup.method === "phone"
            ? t("otp.label_phone")
            : t("otp.label_email")}
        </Text>
        {signup.method === "phone" ? (
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
              onChangeText={(text) => setValue(text.replace(/\D/g, ""))}
              placeholder={t("otp.placeholder_phone")}
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
        <Pressable
          onPress={handleSwitchMethod}
          hitSlop={8}
          style={styles.switchLink}
        >
          <Text
            style={[
              styles.linkText,
              styles.linkUnderline,
              { color: palette.textMuted },
            ]}
          >
            {signup.method === "phone"
              ? t("otp.use_email")
              : t("otp.use_phone")}
          </Text>
        </Pressable>
        <Pressable
          onPress={handleContinue}
          disabled={isLoading}
          style={({ pressed }) => [
            styles.primaryButton,
            styles.continueButton,
            {
              backgroundColor: palette.accent,
              opacity: isLoading ? 0.6 : pressed ? 0.85 : 1,
            },
          ]}
        >
          <Text
            style={[styles.primaryButtonText, { color: palette.accentText }]}
          >
            {isLoading ? "..." : t("otp.button_continue")}
          </Text>
        </Pressable>
        <View style={styles.toggleRow}>
          <Text style={[styles.toggleText, { color: palette.textMuted }]}>
            {t("auth.have_account")}{" "}
          </Text>
          <Pressable onPress={() => router.replace("/sign-in")} hitSlop={8}>
            <Text
              style={[
                styles.linkText,
                styles.linkUnderline,
                { color: palette.textMuted },
              ]}
            >
              {t("auth.log_in")}
            </Text>
          </Pressable>
        </View>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}
