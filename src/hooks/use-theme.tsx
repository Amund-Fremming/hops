import AsyncStorage from "@react-native-async-storage/async-storage";
import type { ReactNode } from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { useColorScheme as useSystemColorScheme } from "react-native";

type Theme = "light" | "dark" | "system";

interface ThemeContextValue {
  theme: Theme;
  colorScheme: "light" | "dark";
  setTheme: (theme: Theme) => void;
  toggleTheme: () => void;
}

const THEME_KEY = "app_theme";

const ThemeContext = createContext<ThemeContextValue | null>(null);

export function ThemeProvider({ children }: { children: ReactNode }) {
  const systemColorScheme =
    useSystemColorScheme() === "dark" ? "dark" : "light";
  const [theme, setThemeState] = useState<Theme>("dark");
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    AsyncStorage.getItem(THEME_KEY).then((stored) => {
      if (stored === "light" || stored === "dark" || stored === "system") {
        setThemeState(stored);
      }
      setIsLoaded(true);
    });
  }, []);

  const setTheme = (newTheme: Theme) => {
    setThemeState(newTheme);
    AsyncStorage.setItem(THEME_KEY, newTheme);
  };

  const toggleTheme = () => {
    const current = theme === "system" ? systemColorScheme : theme;
    setTheme(current === "dark" ? "light" : "dark");
  };

  const colorScheme = theme === "system" ? systemColorScheme : theme;

  const value = useMemo<ThemeContextValue>(
    () => ({ theme, colorScheme, setTheme, toggleTheme }),
    [theme, colorScheme],
  );

  if (!isLoaded) return null;

  return (
    <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
  );
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error("useTheme must be used within ThemeProvider");
  }
  return context;
}
