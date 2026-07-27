import { useEffect } from "react";
import {
  Animated,
  Pressable,
  Text,
  useAnimatedValue,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { useSignup } from "@/features/auth/signup-context";
import { usePalette, useTranslation } from "@/hooks";

import { styles } from "./styles";

export default function SuccessScreen() {
  const palette = usePalette();
  const { t } = useTranslation();
  const signup = useSignup();
  const badgeAnim = useAnimatedValue(0);

  useEffect(() => {
    badgeAnim.setValue(0);
    Animated.spring(badgeAnim, {
      toValue: 1,
      friction: 5,
      tension: 140,
      useNativeDriver: true,
    }).start();
  }, [badgeAnim]);

  const handleComplete = () => {
    // TODO: create the account via API and start a session
    console.log("signup complete", signup.method, signup.identifier);
    signup.reset();
  };

  return (
    <SafeAreaView style={[styles.container, { backgroundColor: palette.bg }]}>
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
          {t("success.title")}
        </Text>
        <Text
          style={[
            styles.subtitle,
            styles.successSubtitle,
            { color: palette.textMuted },
          ]}
        >
          {t("success.subtitle")}
        </Text>
        <Pressable
          onPress={handleComplete}
          style={({ pressed }) => [
            styles.secondaryButton,
            { borderColor: palette.border, opacity: pressed ? 0.7 : 1 },
          ]}
        >
          <Text style={[styles.secondaryButtonText, { color: palette.text }]}>
            {t("success.button")}
          </Text>
        </Pressable>
      </View>
    </SafeAreaView>
  );
}
