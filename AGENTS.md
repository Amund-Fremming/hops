# Expo HAS CHANGED

Read the exact versioned docs at https://docs.expo.dev/versions/v57.0.0/ before writing any code.

# File Organization

- **types.ts** — All TypeScript interfaces and types live here (request/response types, enums, etc.)
- **api.ts** — Only API functions and error classes. Import types from `types.ts`

Example structure for a feature:

```
features/auth/
├── api.ts          # API calls only
├── types.ts        # All interfaces, types, enums
├── index.ts        # Public exports
└── components/     # UI components
```

# Style Scaling

All numeric style values must use scaling functions from `@/lib/scaling`:

- `horizontalScale(size)` — horizontal values: width, paddingHorizontal, marginHorizontal, gap, left, right, minWidth, maxWidth
- `verticalScale(size)` — vertical values: height, paddingVertical, marginVertical, top, bottom, lineHeight, minHeight, maxHeight
- `moderateScale(size, factor?)` — font sizes, border radii, border widths (scales more gently)

**Do not use raw numeric values in StyleSheet.create().**

Example:

```ts
import { horizontalScale, verticalScale, moderateScale } from "@/lib/scaling";

const styles = StyleSheet.create({
  container: {
    paddingHorizontal: horizontalScale(16),
    paddingVertical: verticalScale(12),
    borderRadius: moderateScale(8),
    fontSize: moderateScale(16),
  },
});
```
