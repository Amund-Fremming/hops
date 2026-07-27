import { useEffect, useRef, useState } from "react";
import type { StyleProp, ViewStyle } from "react-native";
import {
  Animated,
  Easing,
  Pressable,
  Text,
  TextInput,
  useAnimatedValue,
  View,
} from "react-native";

import { LanguagePicker } from "@/components";
import { usePalette, useToast } from "@/hooks";

import { styles } from "./styles";

type Step = "start" | "otp" | "signup" | "success";
type Method = "phone" | "email";

const OTP_LENGTH = 6;
const RESEND_SECONDS = 30;
const PHONE_PREFIX = "+47";
// TODO: verify against the OTP API instead of a fixed code
const DEMO_CODE = "123456";

const emptyOtp = () => Array<string>(OTP_LENGTH).fill("");

export interface OtpFlowResult {
  method: Method;
  identifier: string;
  firstName: string;
  lastName: string;
  password: string;
}

interface OtpFlowProps {
  onComplete: (result: OtpFlowResult) => void;
}

export function OtpFlow({ onComplete }: OtpFlowProps) {
  const palette = usePalette();
  const { showError, showSuccess } = useToast();

  const [step, setStep] = useState<Step>("start");
  const [method, setMethod] = useState<Method>("phone");
  const [value, setValue] = useState("");
  const [otp, setOtp] = useState<string[]>(emptyOtp);
  const [resendSeconds, setResendSeconds] = useState(RESEND_SECONDS);
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const otpRefs = useRef<(TextInput | null)[]>([]);
  const stepAnim = useAnimatedValue(0);
  const shakeAnim = useAnimatedValue(0);
  const badgeAnim = useAnimatedValue(0);

  useEffect(() => {
    stepAnim.setValue(0);
    Animated.timing(stepAnim, {
      toValue: 1,
      duration: 350,
      easing: Easing.out(Easing.ease),
      useNativeDriver: true,
    }).start();
  }, [step, stepAnim]);

  useEffect(() => {
    if (step !== "otp" || resendSeconds <= 0) return;
    const timer = setTimeout(() => setResendSeconds((s) => s - 1), 1000);
    return () => clearTimeout(timer);
  }, [step, resendSeconds]);

  useEffect(() => {
    if (step !== "success") return;
    badgeAnim.setValue(0);
    Animated.spring(badgeAnim, {
      toValue: 1,
      friction: 5,
      tension: 140,
      useNativeDriver: true,
    }).start();
  }, [step, badgeAnim]);

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

  const handleSwitchMethod = () => {
    setMethod((m) => (m === "phone" ? "email" : "phone"));
    setValue("");
  };

  const handleContinue = () => {
    const trimmed = value.trim();
    const isValid =
      method === "phone"
        ? trimmed.replace(/\D/g, "").length >= 8
        : /^\S+@\S+\.\S+$/.test(trimmed);

    if (!isValid) {
      showError(
        method === "phone"
          ? "Enter a valid phone number."
          : "Enter a valid email address.",
      );
      return;
    }

    // TODO: request an OTP from the API; surface "already registered" errors here
    setOtp(emptyOtp());
    setResendSeconds(RESEND_SECONDS);
    setStep("otp");
    showSuccess("Verification code sent.");
    setTimeout(() => focusOtpBox(0), 400);
  };

  const handleOtpChange = (index: number, text: string) => {
    const digits = text.replace(/\D/g, "");
    setOtp((current) => {
      const next = [...current];
      if (!digits) {
        next[index] = "";
        return next;
      }
      // autofill/paste can deliver several digits at once — spread them out
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
      showError("Enter all 6 digits.");
      return;
    }
    if (code !== DEMO_CODE) {
      showError("Incorrect code. Try again.");
      setOtp(emptyOtp());
      runShake();
      focusOtpBox(0);
      return;
    }
    setStep("signup");
  };

  const handleResend = () => {
    if (resendSeconds > 0) return;
    // TODO: request a new code from the API
    setResendSeconds(RESEND_SECONDS);
    showSuccess("A new code was sent.");
  };

  const passwordChecks = [
    { label: "At least 8 characters", pass: password.length >= 8 },
    { label: "One uppercase letter", pass: /[A-Z]/.test(password) },
    { label: "One number", pass: /[0-9]/.test(password) },
  ];

  const handleSignUp = () => {
    if (!firstName.trim() || !lastName.trim()) {
      showError("Please enter your first and last name.");
      return;
    }
    if (password.length < 8) {
      showError("Password must be at least 8 characters.");
      return;
    }
    if (!/[A-Z]/.test(password) || !/[0-9]/.test(password)) {
      showError("Password needs an uppercase letter and a number.");
      return;
    }
    setStep("success");
    showSuccess("Welcome to hops!");
  };

  const handleComplete = () => {
    onComplete({
      method,
      identifier: value.trim(),
      firstName: firstName.trim(),
      lastName: lastName.trim(),
      password,
    });
  };

  const progressStep = step === "start" ? 0 : step === "otp" ? 1 : 2;
  const inputTheme = {
    backgroundColor: palette.inputBg,
    borderColor: palette.border,
    color: palette.text,
  };

  return (
    <View style={styles.container}>
      {step !== "success" && (
        <View style={styles.dotsRow}>
          {[0, 1, 2].map((i) => (
            <View
              key={i}
              style={[
                styles.dot,
                {
                  backgroundColor:
                    i <= progressStep ? palette.accent : palette.border,
                  width: i === progressStep ? 22 : 6,
                },
              ]}
            />
          ))}
        </View>
      )}

      {step !== "success" && <LanguagePicker style={styles.languagePicker} />}

      <Animated.View
        style={[
          styles.stepContainer,
          {
            opacity: stepAnim,
            transform: [
              {
                scale: stepAnim.interpolate({
                  inputRange: [0, 1],
                  outputRange: [0.92, 1],
                }),
              },
              {
                translateY: stepAnim.interpolate({
                  inputRange: [0, 1],
                  outputRange: [6, 0],
                }),
              },
            ],
          },
        ]}
      >
        {step === "start" && (
          <>
            <Text style={[styles.title, { color: palette.text }]}>
              Verify it&apos;s you
            </Text>
            <Text style={[styles.subtitle, { color: palette.textMuted }]}>
              We&apos;ll send a one-time code to confirm your identity.
            </Text>
            <Text style={[styles.fieldLabel, { color: palette.textFaint }]}>
              {method === "phone" ? "Phone number" : "Email address"}
            </Text>
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
                  <Text
                    style={[styles.prefixText, { color: palette.textMuted }]}
                  >
                    {PHONE_PREFIX}
                  </Text>
                </View>
                <TextInput
                  style={[styles.input, styles.rowInput, inputTheme]}
                  value={value}
                  onChangeText={setValue}
                  placeholder="555 55 555"
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
                placeholder="you@hops.com"
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
                {method === "phone" ? "Use email instead" : "Use phone instead"}
              </Text>
            </Pressable>
            <PrimaryButton
              label="Continue"
              onPress={handleContinue}
              style={styles.continueButton}
            />
          </>
        )}

        {step === "otp" && (
          <>
            <Pressable
              onPress={() => setStep("start")}
              hitSlop={8}
              style={styles.editLink}
            >
              <Text style={[styles.editLinkText, { color: palette.textMuted }]}>
                ← Edit
              </Text>
            </Pressable>
            <Text style={[styles.title, { color: palette.text }]}>
              Enter the code
            </Text>
            <Text style={[styles.subtitle, { color: palette.textMuted }]}>
              We sent a 6-digit code to{" "}
              <Text style={[styles.subtitleStrong, { color: palette.text }]}>
                {method === "phone"
                  ? `${PHONE_PREFIX} ${value.trim()}`
                  : value.trim()}
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
                  style={[
                    styles.otpBox,
                    { ...inputTheme, borderColor: palette.borderStrong },
                  ]}
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
                    Resend code in 0:
                    {String(resendSeconds).padStart(2, "0")}
                  </Text>
                ) : (
                  <Text
                    style={[
                      styles.linkText,
                      styles.linkUnderline,
                      { color: palette.accent },
                    ]}
                  >
                    Resend code
                  </Text>
                )}
              </Pressable>
            </View>
            <PrimaryButton
              label="Verify"
              onPress={handleVerify}
              style={styles.verifyButton}
            />
          </>
        )}

        {step === "signup" && (
          <>
            <Text style={[styles.title, { color: palette.text }]}>
              Create your account
            </Text>
            <Text style={[styles.subtitle, { color: palette.textMuted }]}>
              Just a few details and you&apos;re in.
            </Text>
            <View style={styles.nameRow}>
              <TextInput
                style={[styles.input, styles.rowInput, inputTheme]}
                value={firstName}
                onChangeText={setFirstName}
                placeholder="First name"
                placeholderTextColor={palette.textFaint}
                autoCapitalize="words"
                autoComplete="given-name"
              />
              <TextInput
                style={[styles.input, styles.rowInput, inputTheme]}
                value={lastName}
                onChangeText={setLastName}
                placeholder="Last name"
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
                placeholder="Password"
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
                <View key={check.label} style={styles.checkRow}>
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
                        style={[
                          styles.checkMarkerText,
                          { color: palette.success },
                        ]}
                      >
                        ✓
                      </Text>
                    )}
                  </View>
                  <Text
                    style={[
                      styles.checkLabel,
                      {
                        color: check.pass
                          ? palette.success
                          : palette.textFaint,
                      },
                    ]}
                  >
                    {check.label}
                  </Text>
                </View>
              ))}
            </View>
            <PrimaryButton label="Sign up" onPress={handleSignUp} />
          </>
        )}

        {step === "success" && (
          <View style={styles.successContainer}>
            <Animated.View
              style={[
                styles.successBadge,
                {
                  backgroundColor: palette.successBg,
                  transform: [{ scale: badgeAnim }],
                },
              ]}
            >
              <Text style={[styles.successCheck, { color: palette.success }]}>
                ✓
              </Text>
            </Animated.View>
            <Text style={[styles.successTitle, { color: palette.text }]}>
              Welcome to hops
            </Text>
            <Text
              style={[
                styles.subtitle,
                styles.successSubtitle,
                { color: palette.textMuted },
              ]}
            >
              Your account is ready to go.
            </Text>
            <Pressable
              onPress={handleComplete}
              style={({ pressed }) => [
                styles.secondaryButton,
                { borderColor: palette.border, opacity: pressed ? 0.7 : 1 },
              ]}
            >
              <Text
                style={[styles.secondaryButtonText, { color: palette.text }]}
              >
                Enter app
              </Text>
            </Pressable>
          </View>
        )}
      </Animated.View>
    </View>
  );
}

function PrimaryButton({
  label,
  onPress,
  style,
}: {
  label: string;
  onPress: () => void;
  style?: StyleProp<ViewStyle>;
}) {
  const palette = usePalette();
  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.primaryButton,
        style,
        { backgroundColor: palette.accent, opacity: pressed ? 0.85 : 1 },
      ]}
    >
      <Text style={[styles.primaryButtonText, { color: palette.accentText }]}>
        {label}
      </Text>
    </Pressable>
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
