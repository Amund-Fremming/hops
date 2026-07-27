import type { ReactNode } from "react";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  Animated,
  Easing,
  Pressable,
  StyleSheet,
  Text,
  useAnimatedValue,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { fonts } from "@/lib/theme";

import { usePalette } from "./use-palette";

type ToastType = "error" | "success";

interface Toast {
  type: ToastType;
  message: string;
}

interface ToastContextValue {
  showToast: (type: ToastType, message: string) => void;
  showError: (message: string) => void;
  showSuccess: (message: string) => void;
  dismissToast: () => void;
}

const TOAST_DURATION_MS = 3800;

const ToastContext = createContext<ToastContextValue | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toast, setToast] = useState<Toast | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const dismissToast = useCallback(() => {
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = null;
    setToast(null);
  }, []);

  const showToast = useCallback((type: ToastType, message: string) => {
    if (timerRef.current) clearTimeout(timerRef.current);
    setToast({ type, message });
    timerRef.current = setTimeout(() => setToast(null), TOAST_DURATION_MS);
  }, []);

  const showError = useCallback(
    (message: string) => showToast("error", message),
    [showToast],
  );

  const showSuccess = useCallback(
    (message: string) => showToast("success", message),
    [showToast],
  );

  useEffect(
    () => () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    },
    [],
  );

  const value = useMemo<ToastContextValue>(
    () => ({ showToast, showError, showSuccess, dismissToast }),
    [showToast, showError, showSuccess, dismissToast],
  );

  return (
    <ToastContext.Provider value={value}>
      {children}
      {toast && <ToastBanner toast={toast} onDismiss={dismissToast} />}
    </ToastContext.Provider>
  );
}

export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error("useToast must be used within ToastProvider");
  }
  return context;
}

function ToastBanner({
  toast,
  onDismiss,
}: {
  toast: Toast;
  onDismiss: () => void;
}) {
  const palette = usePalette();
  const insets = useSafeAreaInsets();
  const anim = useAnimatedValue(0);

  useEffect(() => {
    anim.setValue(0);
    Animated.timing(anim, {
      toValue: 1,
      duration: 300,
      easing: Easing.out(Easing.ease),
      useNativeDriver: true,
    }).start();
  }, [toast, anim]);

  const tint = toast.type === "error" ? palette.error : palette.success;
  const tintBg = toast.type === "error" ? palette.errorBg : palette.successBg;

  return (
    <Animated.View
      style={[
        styles.toast,
        {
          top: insets.top + 12,
          backgroundColor: palette.surface,
          opacity: anim,
          transform: [
            {
              translateY: anim.interpolate({
                inputRange: [0, 1],
                outputRange: [-14, 0],
              }),
            },
          ],
        },
      ]}
    >
      <View
        style={[styles.inner, { backgroundColor: tintBg, borderColor: tint }]}
      >
        <View style={[styles.glyph, { backgroundColor: tint }]}>
          <Text style={[styles.glyphText, { color: palette.surface }]}>
            {toast.type === "error" ? "!" : "✓"}
          </Text>
        </View>
        <Text style={[styles.message, { color: tint }]}>{toast.message}</Text>
        <Pressable onPress={onDismiss} hitSlop={8}>
          <Text style={[styles.close, { color: tint }]}>×</Text>
        </Pressable>
      </View>
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  toast: {
    position: "absolute",
    left: 16,
    right: 16,
    zIndex: 100,
    borderRadius: 14,
    boxShadow: "0 16px 34px -10px rgba(0,0,0,0.4)",
  },
  inner: {
    flexDirection: "row",
    alignItems: "flex-start",
    gap: 10,
    paddingVertical: 13,
    paddingHorizontal: 14,
    borderRadius: 14,
    borderWidth: 1,
  },
  glyph: {
    width: 20,
    height: 20,
    borderRadius: 10,
    alignItems: "center",
    justifyContent: "center",
  },
  glyphText: {
    fontFamily: fonts.bodyExtraBold,
    fontSize: 12,
  },
  message: {
    flex: 1,
    fontFamily: fonts.bodySemiBold,
    fontSize: 13.5,
    lineHeight: 19,
    paddingTop: 1,
  },
  close: {
    fontSize: 15,
    lineHeight: 17,
    opacity: 0.6,
  },
});
