const inset = ref(0);

let initialized = false;

export function useKeyboardInset() {
  if (import.meta.client && !initialized && window.visualViewport) {
    initialized = true;
    const vv = window.visualViewport;
    const update = () => {
      inset.value = Math.max(
        0,
        Math.round(window.innerHeight - vv.height - vv.offsetTop),
      );
      document.documentElement.style.setProperty(
        "--kb-inset",
        `${inset.value}px`,
      );
    };
    useEventListener(vv, "resize", update);
    useEventListener(vv, "scroll", update);
    update();
  }
  return inset;
}
