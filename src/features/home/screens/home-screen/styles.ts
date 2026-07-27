import { StyleSheet } from "react-native";

import { moderateScale } from "@/lib/scaling";

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
});
