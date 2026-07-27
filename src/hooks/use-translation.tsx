import AsyncStorage from "@react-native-async-storage/async-storage";
import type { ReactNode } from "react";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

import type { Language } from "@/i18n";
import { getLanguage, setLanguage as setGlobalLanguage, t } from "@/i18n";

interface LanguageContextValue {
  t: (key: string) => string;
  lang: Language;
  setLanguage: (lang: Language) => void;
}

const LANGUAGE_KEY = "app_language";

const LanguageContext = createContext<LanguageContextValue | null>(null);

export function LanguageProvider({ children }: { children: ReactNode }) {
  const [lang, setLang] = useState<Language>(getLanguage());

  useEffect(() => {
    AsyncStorage.getItem(LANGUAGE_KEY).then((stored) => {
      if (stored === "en" || stored === "no") {
        setGlobalLanguage(stored);
        setLang(stored);
      }
    });
  }, []);

  const setLanguage = useCallback((newLang: Language) => {
    setGlobalLanguage(newLang);
    setLang(newLang);
    AsyncStorage.setItem(LANGUAGE_KEY, newLang);
  }, []);

  const translate = useCallback((key: string) => t(key, lang), [lang]);

  const value = useMemo<LanguageContextValue>(
    () => ({ t: translate, lang, setLanguage }),
    [translate, lang, setLanguage],
  );

  return (
    <LanguageContext.Provider value={value}>{children}</LanguageContext.Provider>
  );
}

export function useTranslation() {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error("useTranslation must be used within LanguageProvider");
  }
  return context;
}
