import { StyleSheet } from "react-native";

import { fonts } from "@/lib/theme";

export const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingTop: 32,
    paddingBottom: 24,
    paddingHorizontal: 28,
  },
  dotsRow: {
    flexDirection: "row",
    gap: 6,
    marginBottom: 36,
  },
  dot: {
    height: 6,
    borderRadius: 3,
  },
  stepContainer: {
    flex: 1,
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
  subtitleStrong: {
    fontFamily: fonts.bodyBold,
  },
  fieldLabel: {
    fontFamily: fonts.bodyBold,
    fontSize: 16.5,
    textTransform: "uppercase",
    letterSpacing: 0.5,
    marginBottom: 8,
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
  switchLink: {
    alignSelf: "flex-start",
    marginTop: 14,
  },
  linkText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 17,
  },
  linkUnderline: {
    textDecorationLine: "underline",
  },
  editLink: {
    alignSelf: "flex-start",
    marginBottom: 18,
  },
  editLinkText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 16.5,
  },
  otpRow: {
    flexDirection: "row",
    justifyContent: "space-between",
  },
  otpBox: {
    width: 46,
    height: 56,
    borderRadius: 14,
    borderWidth: 1,
    textAlign: "center",
    fontFamily: fonts.displayBold,
    fontSize: 26,
    paddingVertical: 0,
  },
  resendRow: {
    alignItems: "center",
    marginTop: 20,
  },
  nameRow: {
    flexDirection: "row",
    gap: 10,
    marginBottom: 12,
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
  checklist: {
    gap: 6,
    marginTop: 12,
    marginBottom: 22,
  },
  checkRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
  },
  checkMarker: {
    width: 19,
    height: 19,
    borderRadius: 9.5,
    borderWidth: 1.5,
    alignItems: "center",
    justifyContent: "center",
  },
  checkMarkerText: {
    fontFamily: fonts.bodyBold,
    fontSize: 13,
    lineHeight: 15,
  },
  checkLabel: {
    fontFamily: fonts.body,
    fontSize: 16.5,
  },
  primaryButton: {
    width: "100%",
    paddingVertical: 15,
    borderRadius: 14,
    alignItems: "center",
  },
  continueButton: {
    marginTop: 32,
  },
  verifyButton: {
    marginTop: 28,
  },
  primaryButtonText: {
    fontFamily: fonts.bodyBold,
    fontSize: 19,
  },
  successContainer: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  successBadge: {
    width: 64,
    height: 64,
    borderRadius: 32,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: 22,
  },
  successCheck: {
    fontSize: 34,
    fontWeight: "700",
    lineHeight: 40,
  },
  successTitle: {
    fontFamily: fonts.display,
    fontSize: 29,
    marginBottom: 8,
  },
  successSubtitle: {
    textAlign: "center",
  },
  secondaryButton: {
    paddingVertical: 14,
    paddingHorizontal: 36,
    borderRadius: 14,
    borderWidth: 1,
    alignItems: "center",
  },
  secondaryButtonText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 18,
  },
});
