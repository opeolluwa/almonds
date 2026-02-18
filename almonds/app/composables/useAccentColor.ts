export type AccentKey = "rose" | "emerald" | "sky" | "amber";

const STORAGE_KEY = "accent-color";

const palettes: Record<AccentKey, Record<string, string>> = {
  rose: {
    "50": "#fdf2f4",
    "100": "#fce7eb",
    "200": "#f9d0d9",
    "300": "#f4a9b9",
    "400": "#ed7694",
    "500": "#d02752",
    "600": "#bf2249",
    "700": "#a11d3f",
    "800": "#871b3a",
    "900": "#741b36",
    "950": "#410a1a",
  },
  emerald: {
    "50": "#ecfdf5",
    "100": "#d1fae5",
    "200": "#a7f3d0",
    "300": "#6ee7b7",
    "400": "#34d399",
    "500": "#10b981",
    "600": "#059669",
    "700": "#047857",
    "800": "#065f46",
    "900": "#064e3b",
    "950": "#022c22",
  },
  sky: {
    "50": "#f0f9ff",
    "100": "#e0f2fe",
    "200": "#bae6fd",
    "300": "#7dd3fc",
    "400": "#38bdf8",
    "500": "#0ea5e9",
    "600": "#0284c7",
    "700": "#0369a1",
    "800": "#075985",
    "900": "#0c4a6e",
    "950": "#082f49",
  },
  amber: {
    "50": "#fffbeb",
    "100": "#fef3c7",
    "200": "#fde68a",
    "300": "#fcd34d",
    "400": "#fbbf24",
    "500": "#f59e0b",
    "600": "#d97706",
    "700": "#b45309",
    "800": "#92400e",
    "900": "#78350f",
    "950": "#451a03",
  },
};

function applyPalette(key: AccentKey) {
  const palette = palettes[key];
  const root = document.documentElement;
  for (const [shade, value] of Object.entries(palette)) {
    root.style.setProperty(`--color-accent-${shade}`, value);
  }
}

export function useAccentColor() {
  const accent = useState<AccentKey>("accent-color", () => "rose");

  function setAccent(key: AccentKey) {
    accent.value = key;
    localStorage.setItem(STORAGE_KEY, key);
    applyPalette(key);
  }

  function init() {
    const stored = localStorage.getItem(STORAGE_KEY) as AccentKey | null;
    if (stored && stored in palettes) {
      accent.value = stored;
      applyPalette(stored);
    }
  }

  return { accent, setAccent, init };
}
