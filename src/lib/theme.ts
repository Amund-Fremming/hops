export const fonts = {
  display: "SpaceGrotesk-SemiBold",
  displayBold: "SpaceGrotesk-Bold",
  body: "Manrope-Regular",
  bodySemiBold: "Manrope-SemiBold",
  bodyBold: "Manrope-Bold",
  bodyExtraBold: "Manrope-ExtraBold",
} as const;

export interface Palette {
  bg: string;
  surface: string;
  surfaceAlt: string;
  border: string;
  borderStrong: string;
  text: string;
  textMuted: string;
  textFaint: string;
  accent: string;
  accentText: string;
  error: string;
  errorBg: string;
  success: string;
  successBg: string;
  inputBg: string;
  shadow: string;
}

export const palettes: Record<"light" | "dark", Palette> = {
  dark: {
    bg: "#0a0d0a",
    surface: "#141814",
    surfaceAlt: "#191e19",
    border: "rgba(255,255,255,0.09)",
    borderStrong: "rgba(255,255,255,0.18)",
    text: "#eef2ea",
    textMuted: "rgba(238,242,234,0.56)",
    textFaint: "rgba(238,242,234,0.36)",
    accent: "#5FCB6E",
    accentText: "#08170a",
    error: "#F0716E",
    errorBg: "rgba(240,113,110,0.14)",
    success: "#5FCB6E",
    successBg: "rgba(95,203,110,0.14)",
    inputBg: "#181d18",
    shadow: "0 30px 80px -20px rgba(0,0,0,0.65)",
  },
  light: {
    bg: "#f6f7f2",
    surface: "#ffffff",
    surfaceAlt: "#eef0e7",
    border: "rgba(20,23,15,0.10)",
    borderStrong: "rgba(20,23,15,0.20)",
    text: "#171a12",
    textMuted: "rgba(23,26,18,0.58)",
    textFaint: "rgba(23,26,18,0.38)",
    accent: "#2F8C43",
    accentText: "#ffffff",
    error: "#C7433F",
    errorBg: "rgba(199,67,63,0.10)",
    success: "#2F8C43",
    successBg: "rgba(47,140,67,0.10)",
    inputBg: "#f1f2ec",
    shadow: "0 30px 80px -24px rgba(20,30,15,0.18)",
  },
};
