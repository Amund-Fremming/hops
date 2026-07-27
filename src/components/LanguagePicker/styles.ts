import { StyleSheet } from "react-native";

import { horizontalScale, moderateScale, verticalScale } from "@/lib/scaling";
import { fonts } from "@/lib/theme";

export const styles = StyleSheet.create({
  wrapper: {
    alignSelf: "flex-end",
    zIndex: 50,
  },
  pill: {
    flexDirection: "row",
    alignItems: "center",
    gap: horizontalScale(8),
    paddingVertical: verticalScale(9),
    paddingHorizontal: horizontalScale(16),
    borderRadius: moderateScale(999),
    borderWidth: moderateScale(1),
  },
  pillLabel: {
    fontFamily: fonts.bodySemiBold,
    fontSize: moderateScale(17),
  },
  globe: {
    width: horizontalScale(17),
    height: verticalScale(17),
    borderRadius: moderateScale(8.5),
    borderWidth: moderateScale(1.3),
    alignItems: "center",
    justifyContent: "center",
  },
  globeMeridian: {
    width: horizontalScale(7),
    height: verticalScale(17),
    borderRadius: moderateScale(3.5),
    borderWidth: moderateScale(1.3),
  },
  globeEquator: {
    position: "absolute",
    width: horizontalScale(15),
    height: verticalScale(1.3),
  },
  menu: {
    position: "absolute",
    top: verticalScale(46),
    right: 0,
    minWidth: horizontalScale(168),
    borderRadius: moderateScale(20),
    borderWidth: moderateScale(1),
    paddingVertical: verticalScale(10),
    paddingHorizontal: horizontalScale(6),
    boxShadow: "0 16px 34px -10px rgba(0,0,0,0.4)",
    // keeps the menu above the step content on Android, where zIndex alone is unreliable
    elevation: 12,
  },
  menuItem: {
    paddingVertical: verticalScale(10),
    paddingHorizontal: horizontalScale(14),
    borderRadius: moderateScale(12),
  },
  menuItemText: {
    fontFamily: fonts.body,
    fontSize: moderateScale(19),
  },
});
