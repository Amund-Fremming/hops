import { useState } from "react";
import type { StyleProp, ViewStyle } from "react-native";
import {
  Animated,
  Easing,
  Pressable,
  Text,
  useAnimatedValue,
  View,
} from "react-native";

import { usePalette, useTranslation } from "@/hooks";
import type { Language } from "@/i18n";
import { LANGUAGES } from "@/i18n";

import { styles } from "./styles";

interface LanguagePickerProps {
  style?: StyleProp<ViewStyle>;
}

export function LanguagePicker({ style }: LanguagePickerProps) {
  const palette = usePalette();
  const { lang, setLanguage } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const menuAnim = useAnimatedValue(0);

  const active = LANGUAGES.find((l) => l.code === lang) ?? LANGUAGES[0];

  const toggleMenu = () => {
    const opening = !isOpen;
    setIsOpen(opening);
    Animated.timing(menuAnim, {
      toValue: opening ? 1 : 0,
      duration: 180,
      easing: Easing.out(Easing.ease),
      useNativeDriver: true,
    }).start();
  };

  const handleSelect = (code: Language) => {
    setLanguage(code);
    setIsOpen(false);
    Animated.timing(menuAnim, {
      toValue: 0,
      duration: 140,
      easing: Easing.out(Easing.ease),
      useNativeDriver: true,
    }).start();
  };

  return (
    <View style={[styles.wrapper, style]}>
      <Pressable
        onPress={toggleMenu}
        accessibilityRole="button"
        accessibilityLabel={`Language: ${active.label}`}
        style={({ pressed }) => [
          styles.pill,
          {
            backgroundColor: palette.surfaceAlt,
            borderColor: palette.border,
            opacity: pressed ? 0.7 : 1,
          },
        ]}
      >
        <GlobeIcon color={palette.textMuted} />
        <Text style={[styles.pillLabel, { color: palette.text }]}>
          {active.short}
        </Text>
      </Pressable>

      {isOpen && (
        <Animated.View
          style={[
            styles.menu,
            {
              backgroundColor: palette.surfaceAlt,
              borderColor: palette.border,
              opacity: menuAnim,
              transform: [
                {
                  translateY: menuAnim.interpolate({
                    inputRange: [0, 1],
                    outputRange: [-6, 0],
                  }),
                },
              ],
            },
          ]}
        >
          {LANGUAGES.map((language) => (
            <Pressable
              key={language.code}
              onPress={() => handleSelect(language.code)}
              style={({ pressed }) => [
                styles.menuItem,
                { opacity: pressed ? 0.6 : 1 },
              ]}
            >
              <Text
                style={[
                  styles.menuItemText,
                  {
                    color:
                      language.code === lang
                        ? palette.accent
                        : palette.textMuted,
                  },
                ]}
              >
                {language.label}
              </Text>
            </Pressable>
          ))}
        </Animated.View>
      )}
    </View>
  );
}

function GlobeIcon({ color }: { color: string }) {
  return (
    <View style={[styles.globe, { borderColor: color }]}>
      <View style={[styles.globeMeridian, { borderColor: color }]} />
      <View style={[styles.globeEquator, { backgroundColor: color }]} />
    </View>
  );
}
