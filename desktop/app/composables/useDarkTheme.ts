export type DarkThemeKey = "charcoal" | "grape" | "onyx" | "plum";

const STORAGE_KEY = "dark-theme";

const themes: Record<
  DarkThemeKey,
  { label: string; swatch: string; palette: Record<string, string> }
> = {
  charcoal: {
    label: "Charcoal",
    swatch: "#222831",
    palette: {
      "50": "#f0f3f5",
      "100": "#e2e7eb",
      "200": "#c8d0d8",
      "300": "#a6b4bf",
      "400": "#8496a5",
      "500": "#677a8c",
      "600": "#506070",
      "700": "#3d4759",
      "800": "#2d3542",
      "900": "#222831",
      "950": "#161b20",
    },
  },
  grape: {
    label: "Grape",
    swatch: "#210F37",
    palette: {
      "50": "#f6eefa",
      "100": "#ecdef4",
      "200": "#d6bfe6",
      "300": "#ba96d5",
      "400": "#9c6ac2",
      "500": "#7a44a8",
      "600": "#5d3188",
      "700": "#45236a",
      "800": "#31174f",
      "900": "#210F37",
      "950": "#150922",
    },
  },
  onyx: {
    label: "Onyx",
    swatch: "#0d1220",
    palette: {
      "50": "#e0e6f2",
      "100": "#b8c4e0",
      "200": "#8a9dc6",
      "300": "#5e7aab",
      "400": "#375990",
      "500": "#0d1220",
      "600": "#0a0f1c",
      "700": "#070b14",
      "800": "#04070d",
      "900": "#020407",
      "950": "#010203",
    },
  },
  plum: {
    label: "Plum",
    swatch: "#211832",
    palette: {
      "50": "#f3f0f8",
      "100": "#e7e2f0",
      "200": "#cfc5e0",
      "300": "#af9fcb",
      "400": "#9079b0",
      "500": "#715c94",
      "600": "#584578",
      "700": "#42325e",
      "800": "#302347",
      "900": "#211832",
      "950": "#150f20",
    },
  },
};

function applyTheme(key: DarkThemeKey) {
  const theme = themes[key];
  const root = document.documentElement;
  for (const [shade, value] of Object.entries(theme.palette)) {
    root.style.setProperty(`--color-surface-${shade}`, value);
  }
}

export function useDarkTheme() {
  const darkTheme = useState<DarkThemeKey>("dark-theme", () => "onyx");

  function setDarkTheme(key: DarkThemeKey) {
    darkTheme.value = key;
    localStorage.setItem(STORAGE_KEY, key);
    applyTheme(key);
  }

  function init() {
    const stored = localStorage.getItem(STORAGE_KEY) as DarkThemeKey | null;
    if (stored && stored in themes) {
      darkTheme.value = stored;
      applyTheme(stored);
    }
  }

  return { darkTheme, themes, setDarkTheme, init };
}
