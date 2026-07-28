import { useRouter } from "expo-router";
import { useEffect } from "react";
import {
  Animated,
  Pressable,
  Text,
  useAnimatedValue,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { usePalette, useTranslation } from "@/hooks";

import { styles } from "./styles";

export default function SignupSuccessScreen() {
  const palette = usePalette();
  const { t } = useTranslation();
  const router = useRouter();
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
    router.replace("/");
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
