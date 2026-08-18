const palette: Record<string, string> = {
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
};

function applyTheme() {
  const root = document.documentElement;
  for (const [shade, value] of Object.entries(palette)) {
    root.style.setProperty(`--color-surface-${shade}`, value);
  }
}

export function useDarkTheme() {
  function init() {
    applyTheme();
  }

  return { init };
}
