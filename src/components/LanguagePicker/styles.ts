import { StyleSheet } from "react-native";

import { fonts } from "@/lib/theme";

export const styles = StyleSheet.create({
  wrapper: {
    alignSelf: "flex-end",
    zIndex: 50,
  },
  pill: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
    paddingVertical: 9,
    paddingHorizontal: 16,
    borderRadius: 999,
    borderWidth: 1,
  },
  pillLabel: {
    fontFamily: fonts.bodySemiBold,
    fontSize: 17,
  },
  globe: {
    width: 17,
    height: 17,
    borderRadius: 8.5,
    borderWidth: 1.3,
    alignItems: "center",
    justifyContent: "center",
  },
  globeMeridian: {
    width: 7,
    height: 17,
    borderRadius: 3.5,
    borderWidth: 1.3,
  },
  globeEquator: {
    position: "absolute",
    width: 15,
    height: 1.3,
  },
  menu: {
    position: "absolute",
    top: 46,
    right: 0,
    minWidth: 168,
    borderRadius: 20,
    borderWidth: 1,
    paddingVertical: 10,
    paddingHorizontal: 6,
    boxShadow: "0 16px 34px -10px rgba(0,0,0,0.4)",
    // keeps the menu above the step content on Android, where zIndex alone is unreliable
    elevation: 12,
  },
  menuItem: {
    paddingVertical: 10,
    paddingHorizontal: 14,
    borderRadius: 12,
  },
  menuItemText: {
    fontFamily: fonts.body,
    fontSize: 19,
  },
});
