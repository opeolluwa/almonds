export function useAppSearch() {
  const searchQuery = useState<string>("appSearchQuery", () => "");
  const isOpen = useState<boolean>("appSearchOpen", () => false);

  // Kept for backward compat — pages that register per-page search still call these;
  // now they're no-ops since search is global.
  function setSearch(_config?: unknown) {
    searchQuery.value = "";
  }

  function clearSearch() {
    searchQuery.value = "";
    isOpen.value = false;
  }

  return { searchQuery, isOpen, setSearch, clearSearch };
}
