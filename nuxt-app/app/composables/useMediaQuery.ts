export function useMediaQuery() {
  const isMobile = useState("media-mobile", () => false);
  const isTablet = useState("media-tablet", () => false);
  const isDesktop = useState("media-desktop", () => true);

  if (typeof window === "undefined") return { isMobile, isTablet, isDesktop };

  const mobile = window.matchMedia("(max-width: 639px)");
  const tablet = window.matchMedia("(min-width: 640px) and (max-width: 1023px)");
  const desktop = window.matchMedia("(min-width: 1024px)");

  function update() {
    isMobile.value = mobile.matches;
    isTablet.value = tablet.matches;
    isDesktop.value = desktop.matches;
  }

  update();

  mobile.addEventListener("change", update);
  tablet.addEventListener("change", update);
  desktop.addEventListener("change", update);

  onUnmounted(() => {
    mobile.removeEventListener("change", update);
    tablet.removeEventListener("change", update);
    desktop.removeEventListener("change", update);
  });

  return { isMobile, isTablet, isDesktop };
}
