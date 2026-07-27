import { useRouter } from "expo-router";
import { useEffect, useRef, useState } from "react";
import {
  Animated,
  Easing,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  Text,
  TextInput,
  useAnimatedValue,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { LanguagePicker } from "@/components";
import { useSignup } from "@/features/auth/signup-context";
import { usePalette, useToast, useTranslation } from "@/hooks";

import { styles } from "./styles";

const OTP_LENGTH = 6;
const RESEND_SECONDS = 30;
const PHONE_PREFIX = "+47";
const DEMO_CODE = "123456";

const emptyOtp = () => Array<string>(OTP_LENGTH).fill("");

export default function OtpScreen() {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();
  const { t } = useTranslation();
  const router = useRouter();
  const signup = useSignup();

  const [otp, setOtp] = useState<string[]>(emptyOtp);
  const [resendSeconds, setResendSeconds] = useState(RESEND_SECONDS);
  const otpRefs = useRef<(TextInput | null)[]>([]);
  const shakeAnim = useAnimatedValue(0);

  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.borderStrong,
    color: palette.text,
  };

  useEffect(() => {
    if (resendSeconds <= 0) return;
    const timer = setTimeout(() => setResendSeconds((s) => s - 1), 1000);
    return () => clearTimeout(timer);
  }, [resendSeconds]);

  useEffect(() => {
    setTimeout(() => otpRefs.current[0]?.focus(), 400);
  }, []);

  const focusOtpBox = (index: number) => {
    otpRefs.current[index]?.focus();
  };

  const runShake = () => {
    const move = (toValue: number) =>
      Animated.timing(shakeAnim, {
        toValue,
        duration: 50,
        easing: Easing.linear,
        useNativeDriver: true,
      });
    shakeAnim.setValue(0);
    Animated.sequence([
      move(-2),
      move(4),
      move(-8),
      move(8),
      move(-8),
      move(8),
      move(-2),
      move(0),
    ]).start();
  };

  const handleOtpChange = (index: number, text: string) => {
    const digits = text.replace(/\D/g, "");
    setOtp((current) => {
      const next = [...current];
      if (!digits) {
        next[index] = "";
        return next;
      }
      let target = index;
      for (const digit of digits) {
        if (target >= OTP_LENGTH) break;
        next[target] = digit;
        target += 1;
      }
      return next;
    });
    if (digits) {
      focusOtpBox(Math.min(index + digits.length, OTP_LENGTH - 1));
    }
  };

  const handleOtpKeyPress = (index: number, key: string) => {
    if (key === "Backspace" && !otp[index] && index > 0) {
      focusOtpBox(index - 1);
    }
  };

  const handleVerify = () => {
    const code = otp.join("");
    if (code.length < OTP_LENGTH) {
      showError(t("otp.error_incomplete"));
      return;
    }
    if (code !== DEMO_CODE) {
      showError(t("otp.error_incorrect"));
      setOtp(emptyOtp());
      runShake();
      focusOtpBox(0);
      return;
    }
    router.push("/sign-up/details");
  };

  const handleResend = () => {
    if (resendSeconds > 0) return;
    setResendSeconds(RESEND_SECONDS);
    showSuccess(t("otp.code_resent"));
  };

  const displayIdentifier =
    signup.method === "phone"
      ? `${PHONE_PREFIX} ${signup.identifier}`
      : signup.identifier;

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
                    backgroundColor: i <= 1 ? palette.accent : palette.border,
                    width: i === 1 ? 22 : 6,
                  },
                ]}
              />
            ))}
          </View>
          <LanguagePicker style={styles.languagePickerAbsolute} />
        </View>

        <Text style={[styles.title, { color: palette.text }]}>
          {t("otp.verify_title")}
        </Text>
        <Text style={[styles.subtitle, { color: palette.textMuted }]}>
          {t("otp.verify_subtitle")}{" "}
          <Text style={[styles.subtitleStrong, { color: palette.text }]}>
            {displayIdentifier}
          </Text>
        </Text>
        <Animated.View
          style={[styles.otpRow, { transform: [{ translateX: shakeAnim }] }]}
        >
          {otp.map((digit, i) => (
            <TextInput
              key={i}
              ref={(el) => {
                otpRefs.current[i] = el;
              }}
              style={[styles.otpBox, inputTheme]}
              value={digit}
              onChangeText={(text) => handleOtpChange(i, text)}
              onKeyPress={({ nativeEvent }) =>
                handleOtpKeyPress(i, nativeEvent.key)
              }
              keyboardType="number-pad"
              maxLength={i === 0 ? OTP_LENGTH : 1}
              textContentType="oneTimeCode"
              autoComplete="one-time-code"
              selectTextOnFocus
            />
          ))}
        </Animated.View>
        <View style={styles.resendRow}>
          <Pressable
            onPress={handleResend}
            disabled={resendSeconds > 0}
            hitSlop={8}
          >
            {resendSeconds > 0 ? (
              <Text style={[styles.linkText, { color: palette.textFaint }]}>
                {t("otp.resend_in")} 0:{String(resendSeconds).padStart(2, "0")}
              </Text>
            ) : (
              <Text
                style={[
                  styles.linkText,
                  styles.linkUnderline,
                  { color: palette.accent },
                ]}
              >
                {t("otp.button_resend")}
              </Text>
            )}
          </Pressable>
        </View>
        <Pressable
          onPress={handleVerify}
          style={({ pressed }) => [
            styles.primaryButton,
            styles.verifyButton,
            { backgroundColor: palette.accent, opacity: pressed ? 0.85 : 1 },
          ]}
        >
          <Text
            style={[styles.primaryButtonText, { color: palette.accentText }]}
          >
            {t("otp.button_verify")}
          </Text>
        </Pressable>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}
