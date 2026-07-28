import { StyleSheet } from "react-native";

import { horizontalScale, moderateScale, verticalScale } from "@/lib/scaling";

export const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
  title: {
    fontSize: moderateScale(24),
    fontWeight: "600",
  },
  logoutButton: {
    marginTop: verticalScale(24),
    paddingHorizontal: horizontalScale(24),
    paddingVertical: verticalScale(12),
    backgroundColor: "#ff4444",
    borderRadius: moderateScale(8),
  },
  logoutText: {
    color: "#fff",
    fontSize: moderateScale(16),
    fontWeight: "600",
  },
});
