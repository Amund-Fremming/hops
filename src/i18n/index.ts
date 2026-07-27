import translations from "./translations.json";

export type Language = "en" | "no";
type TranslationKey = keyof typeof translations;

/** Languages offered in the picker. Add an entry once its strings exist in translations.json. */
export const LANGUAGES: { code: Language; label: string; short: string }[] = [
  { code: "en", label: "English", short: "EN" },
  { code: "no", label: "Norsk", short: "NO" },
];

let currentLanguage: Language = "en";

export function setLanguage(lang: Language) {
  currentLanguage = lang;
}

export function getLanguage(): Language {
  return currentLanguage;
}

export function t(key: string, lang?: Language): string {
  const useLang = lang ?? currentLanguage;
  const entry = translations[key as TranslationKey];
  if (!entry) return key;
  return entry[useLang] ?? entry.en ?? key;
}
