export type FontSizeKey = "sm" | "md" | "lg";

const STORAGE_KEY = "font-size";

const sizes: Record<FontSizeKey, string> = {
  sm: "13px",
  md: "15px",
  lg: "17px",
};

function applySize(key: FontSizeKey) {
  document.documentElement.style.setProperty(
    "--font-size-base",
    sizes[key]
  );
}

const fontSize = useState<FontSizeKey>("font-size", () => "md");

export function useFontSize() {
  function setFontSize(key: FontSizeKey) {
    fontSize.value = key;
    localStorage.setItem(STORAGE_KEY, key);
    applySize(key);
  }

  function init() {
    const stored = localStorage.getItem(STORAGE_KEY) as FontSizeKey | null;
    if (stored && stored in sizes) {
      fontSize.value = stored;
      applySize(stored);
    }
  }

  return { fontSize, setFontSize, init };
}
