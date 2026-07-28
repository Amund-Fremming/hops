import { StyleSheet } from "react-native";

import { horizontalScale, moderateScale, verticalScale } from "@/lib/scaling";
import { fonts } from "@/lib/theme";

export const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  inner: {
    flex: 1,
    paddingTop: verticalScale(32),
    paddingBottom: verticalScale(24),
    paddingHorizontal: horizontalScale(28),
  },
  headerRow: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: verticalScale(36),
  },
  dotsRow: {
    flexDirection: "row",
    gap: horizontalScale(6),
  },
  languagePickerAbsolute: {
    position: "absolute",
    top: 0,
    right: 0,
  },
  dot: {
    height: verticalScale(6),
    borderRadius: moderateScale(3),
  },
  title: {
    fontFamily: fonts.displayBold,
    fontSize: moderateScale(30),
    letterSpacing: -0.25,
    marginBottom: verticalScale(8),
  },
  subtitle: {
    fontFamily: fonts.body,
    fontSize: moderateScale(18.5),
    lineHeight: verticalScale(26),
    marginBottom: verticalScale(28),
  },
  subtitleStrong: {
    fontFamily: fonts.bodyBold,
  },
  fieldLabel: {
    fontFamily: fonts.bodyBold,
    fontSize: moderateScale(16.5),
    letterSpacing: 0.5,
    marginBottom: verticalScale(8),
  },
  phoneRow: {
    flexDirection: "row",
    gap: horizontalScale(8),
  },
  prefixPill: {
    paddingHorizontal: horizontalScale(14),
    borderRadius: moderateScale(14),
    borderWidth: moderateScale(1),
    alignItems: "center",
    justifyContent: "center",
  },
  prefixText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(19),
  },
  input: {
    borderRadius: moderateScale(14),
    borderWidth: moderateScale(1),
    fontFamily: fonts.body,
    fontSize: moderateScale(19),
    paddingVertical: verticalScale(14),
    paddingHorizontal: horizontalScale(16),
  },
  rowInput: {
    flex: 1,
    minWidth: 0,
  },
  switchLink: {
    alignSelf: "flex-start",
    marginTop: verticalScale(14),
  },
  linkText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(17),
  },
  linkUnderline: {
    textDecorationLine: "underline",
  },
  otpRow: {
    flexDirection: "row",
    justifyContent: "space-between",
  },
  otpBox: {
    width: horizontalScale(46),
    height: verticalScale(56),
    borderRadius: moderateScale(14),
    borderWidth: moderateScale(1),
    textAlign: "center",
    fontFamily: fonts.displayBold,
    fontSize: moderateScale(26),
    paddingVertical: 0,
  },
  resendRow: {
    alignItems: "flex-start",
    marginTop: verticalScale(20),
  },
  nameRow: {
    flexDirection: "row",
    gap: horizontalScale(10),
    marginBottom: verticalScale(12),
  },
  passwordInput: {
    paddingRight: horizontalScale(44),
  },
  eyeButton: {
    position: "absolute",
    right: horizontalScale(12),
    top: 0,
    bottom: 0,
    justifyContent: "center",
  },
  eyeIcon: {
    width: horizontalScale(19),
    height: verticalScale(19),
    alignItems: "center",
    justifyContent: "center",
  },
  eyeOutline: {
    width: horizontalScale(19),
    height: verticalScale(12),
    borderRadius: moderateScale(6),
    borderWidth: moderateScale(1.5),
  },
  eyePupil: {
    position: "absolute",
    width: horizontalScale(6),
    height: verticalScale(6),
    borderRadius: moderateScale(3),
    borderWidth: moderateScale(1.5),
  },
  eyeSlash: {
    position: "absolute",
    width: horizontalScale(22),
    height: verticalScale(1.5),
    borderRadius: moderateScale(1),
    transform: [{ rotate: "45deg" }],
  },
  checklist: {
    gap: verticalScale(6),
    marginTop: verticalScale(12),
    marginBottom: verticalScale(22),
  },
  checkRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: horizontalScale(8),
  },
  checkMarker: {
    width: horizontalScale(19),
    height: verticalScale(19),
    borderRadius: moderateScale(9.5),
    borderWidth: moderateScale(1.5),
    alignItems: "center",
    justifyContent: "center",
  },
  checkMarkerText: {
    fontFamily: fonts.bodyBold,
    fontSize: moderateScale(13),
    lineHeight: verticalScale(15),
  },
  checkLabel: {
    fontFamily: fonts.body,
    fontSize: moderateScale(16.5),
  },
  primaryButton: {
    width: "100%",
    paddingVertical: verticalScale(15),
    borderRadius: moderateScale(14),
    alignItems: "center",
  },
  continueButton: {
    marginTop: verticalScale(32),
  },
  toggleRow: {
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    marginTop: verticalScale(20),
  },
  toggleText: {
    fontFamily: fonts.body,
    fontSize: moderateScale(17),
  },
  verifyButton: {
    marginTop: verticalScale(28),
  },
  primaryButtonText: {
    fontFamily: fonts.bodyBold,
    fontSize: moderateScale(19),
  },
  successContainer: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  successBadge: {
    width: horizontalScale(64),
    height: verticalScale(64),
    borderRadius: moderateScale(32),
    alignItems: "center",
    justifyContent: "center",
    marginBottom: verticalScale(22),
  },
  successCheck: {
    fontSize: moderateScale(34),
    fontWeight: "700",
    lineHeight: verticalScale(40),
  },
  successTitle: {
    fontFamily: fonts.display,
    fontSize: moderateScale(29),
    marginBottom: verticalScale(8),
  },
  successSubtitle: {
    textAlign: "center",
  },
  secondaryButton: {
    paddingVertical: verticalScale(14),
    paddingHorizontal: horizontalScale(36),
    borderRadius: moderateScale(14),
    borderWidth: moderateScale(1),
    alignItems: "center",
  },
  secondaryButtonText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(18),
  },
});
