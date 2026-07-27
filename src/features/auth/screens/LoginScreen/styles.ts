import { StyleSheet } from "react-native";

import { fonts } from "@/lib/theme";

export const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  keyboardView: {
    flex: 1,
  },
  content: {
    flex: 1,
    paddingTop: 32,
    paddingBottom: 24,
    paddingHorizontal: 28,
  },
  languagePicker: {
    marginBottom: 20,
  },
  title: {
    fontFamily: fonts.displayBold,
    fontSize: 30,
    letterSpacing: -0.25,
    marginBottom: 8,
  },
  subtitle: {
    fontFamily: fonts.body,
    fontSize: 18.5,
    lineHeight: 26,
    marginBottom: 28,
  },
  labelRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: 8,
  },
  fieldLabel: {
    fontFamily: fonts.bodyBold,
    fontSize: 16.5,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  switchText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 16,
  },
  linkUnderline: {
    textDecorationLine: "underline",
  },
  phoneRow: {
    flexDirection: "row",
    gap: 8,
  },
  prefixPill: {
    paddingHorizontal: 14,
    borderRadius: 14,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  prefixText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 19,
  },
  input: {
    borderRadius: 14,
    borderWidth: 1,
    fontFamily: fonts.body,
    fontSize: 19,
    paddingVertical: 14,
    paddingHorizontal: 16,
  },
  rowInput: {
    flex: 1,
    minWidth: 0,
  },
  passwordWrapper: {
    marginTop: 16,
  },
  passwordInput: {
    paddingRight: 44,
  },
  eyeButton: {
    position: "absolute",
    right: 12,
    top: 0,
    bottom: 0,
    justifyContent: "center",
  },
  eyeIcon: {
    width: 19,
    height: 19,
    alignItems: "center",
    justifyContent: "center",
  },
  eyeOutline: {
    width: 19,
    height: 12,
    borderRadius: 6,
    borderWidth: 1.5,
  },
  eyePupil: {
    position: "absolute",
    width: 6,
    height: 6,
    borderRadius: 3,
    borderWidth: 1.5,
  },
  eyeSlash: {
    position: "absolute",
    width: 22,
    height: 1.5,
    borderRadius: 1,
    transform: [{ rotate: "45deg" }],
  },
  forgotLink: {
    alignSelf: "flex-start",
    marginTop: 14,
  },
  linkText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 17,
  },
  demoTip: {
    fontFamily: fonts.body,
    fontSize: 15,
    marginTop: 12,
    marginBottom: 32,
  },
  primaryButton: {
    width: "100%",
    paddingVertical: 15,
    borderRadius: 14,
    alignItems: "center",
  },
  primaryButtonText: {
    fontFamily: fonts.bodyBold,
    fontSize: 19,
  },
  toggleRow: {
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    marginTop: 20,
  },
  toggleText: {
    fontFamily: fonts.body,
    fontSize: 17,
  },
});
