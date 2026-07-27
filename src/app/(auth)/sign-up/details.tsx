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
import { useSignup } from "@/features/auth/signup-context";
import { usePalette, useToast, useTranslation } from "@/hooks";

import { styles } from "./styles";

export default function DetailsScreen() {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();
  const { t } = useTranslation();
  const router = useRouter();
  const signup = useSignup();

  const [firstName, setFirstName] = useState(signup.firstName);
  const [lastName, setLastName] = useState(signup.lastName);
  const [password, setPassword] = useState(signup.password);
  const [showPassword, setShowPassword] = useState(false);

  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.border,
    color: palette.text,
  };

  const passwordChecks = [
    { key: "signup.req_length", pass: password.length >= 8 },
    { key: "signup.req_uppercase", pass: /[A-Z]/.test(password) },
    { key: "signup.req_number", pass: /[0-9]/.test(password) },
  ];

  const handleSignUp = () => {
    if (!firstName.trim() || !lastName.trim()) {
      showError(t("signup.error_name"));
      return;
    }
    if (password.length < 8) {
      showError(t("signup.error_password_short"));
      return;
    }
    if (!/[A-Z]/.test(password) || !/[0-9]/.test(password)) {
      showError(t("signup.error_password_rules"));
      return;
    }
    signup.setFirstName(firstName.trim());
    signup.setLastName(lastName.trim());
    signup.setPassword(password);
    showSuccess(t("success.toast"));
    router.push("/sign-up/success");
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
                    backgroundColor: palette.accent,
                    width: i === 2 ? 22 : 6,
                  },
                ]}
              />
            ))}
          </View>
          <LanguagePicker style={styles.languagePickerAbsolute} />
        </View>

        <Text style={[styles.title, { color: palette.text }]}>
          {t("signup.title")}
        </Text>
        <Text style={[styles.subtitle, { color: palette.textMuted }]}>
          {t("signup.subtitle")}
        </Text>
        <View style={styles.nameRow}>
          <TextInput
            style={[styles.input, styles.rowInput, inputTheme]}
            value={firstName}
            onChangeText={setFirstName}
            placeholder={t("signup.first_name")}
            placeholderTextColor={palette.textFaint}
            autoCapitalize="words"
            autoComplete="given-name"
          />
          <TextInput
            style={[styles.input, styles.rowInput, inputTheme]}
            value={lastName}
            onChangeText={setLastName}
            placeholder={t("signup.last_name")}
            placeholderTextColor={palette.textFaint}
            autoCapitalize="words"
            autoComplete="family-name"
          />
        </View>
        <View>
          <TextInput
            style={[styles.input, styles.passwordInput, inputTheme]}
            value={password}
            onChangeText={setPassword}
            placeholder={t("signup.password")}
            placeholderTextColor={palette.textFaint}
            secureTextEntry={!showPassword}
            autoCapitalize="none"
            autoComplete="new-password"
          />
          <Pressable
            onPress={() => setShowPassword((s) => !s)}
            hitSlop={8}
            style={styles.eyeButton}
          >
            <EyeIcon open={showPassword} color={palette.textMuted} />
          </Pressable>
        </View>
        <View style={styles.checklist}>
          {passwordChecks.map((check) => (
            <View key={check.key} style={styles.checkRow}>
              <View
                style={[
                  styles.checkMarker,
                  {
                    borderColor: check.pass
                      ? palette.success
                      : palette.textFaint,
                    backgroundColor: check.pass
                      ? palette.successBg
                      : "transparent",
                  },
                ]}
              >
                {check.pass && (
                  <Text
                    style={[styles.checkMarkerText, { color: palette.success }]}
                  >
                    ✓
                  </Text>
                )}
              </View>
              <Text
                style={[
                  styles.checkLabel,
                  {
                    color: check.pass ? palette.success : palette.textFaint,
                  },
                ]}
              >
                {t(check.key)}
              </Text>
            </View>
          ))}
        </View>
        <Pressable
          onPress={handleSignUp}
          style={({ pressed }) => [
            styles.primaryButton,
            { backgroundColor: palette.accent, opacity: pressed ? 0.85 : 1 },
          ]}
        >
          <Text
            style={[styles.primaryButtonText, { color: palette.accentText }]}
          >
            {t("signup.button")}
          </Text>
        </Pressable>
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
