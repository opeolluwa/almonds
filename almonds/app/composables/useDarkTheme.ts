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
    swatch: "#141418",
    palette: {
      "50": "#eaeaf0",
      "100": "#d1d1dc",
      "200": "#a8a8b8",
      "300": "#7c7c8f",
      "400": "#555563",
      "500": "#3a3a44",
      "600": "#28282f",
      "700": "#1c1c22",
      "800": "#141418",
      "900": "#0d0d0f",
      "950": "#050507",
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
