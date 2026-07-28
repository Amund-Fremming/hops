import { StyleSheet } from "react-native";

import { horizontalScale, moderateScale, verticalScale } from "@/lib/scaling";
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
    paddingTop: verticalScale(32),
    paddingBottom: verticalScale(24),
    paddingHorizontal: horizontalScale(28),
  },
  headerRow: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: verticalScale(36),
  },
  languagePickerAbsolute: {
    position: "absolute",
    top: verticalScale(-20),
    right: 0,
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
  labelRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: verticalScale(8),
  },
  fieldLabel: {
    fontFamily: fonts.bodyBold,
    fontSize: moderateScale(16.5),
    letterSpacing: 0.5,
    marginBottom: verticalScale(10),
  },
  switchText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(16),
  },
  linkUnderline: {
    textDecorationLine: "underline",
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
  passwordWrapper: {
    marginTop: verticalScale(20),
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
  forgotLink: {
    alignSelf: "flex-start",
    marginTop: verticalScale(14),
  },
  linkText: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(17),
  },
  primaryButton: {
    marginTop: verticalScale(32),
    width: "100%",
    paddingVertical: verticalScale(15),
    borderRadius: moderateScale(14),
    alignItems: "center",
  },
  primaryButtonText: {
    fontFamily: fonts.bodyBold,
    fontSize: moderateScale(19),
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
});
