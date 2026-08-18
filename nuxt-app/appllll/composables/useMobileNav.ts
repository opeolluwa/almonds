export function useMobileNav() {
  const mobileNavOpen = useState<boolean>("appMobileNavOpen", () => false);

  function toggleMobileNav() {
    mobileNavOpen.value = !mobileNavOpen.value;
  }

  return { mobileNavOpen, toggleMobileNav };
}
