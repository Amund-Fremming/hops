import type { Palette } from "@/lib/theme";
import { palettes } from "@/lib/theme";

import { useTheme } from "./use-theme";

export function usePalette(): Palette {
  const { colorScheme } = useTheme();
  return palettes[colorScheme];
}
