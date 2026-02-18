interface SearchConfig {
  placeholder?: string;
  searchFn?: (query: string) => void;
  data?: unknown[];
}

export function useAppSearch() {
  const searchConfig = useState<SearchConfig | null>("appSearch", () => null);
  const searchQuery = useState<string>("appSearchQuery", () => "");

  function setSearch(config: SearchConfig) {
    searchQuery.value = "";
    searchConfig.value = config;
  }

  function clearSearch() {
    searchConfig.value = null;
    searchQuery.value = "";
  }

  return {
    searchConfig: readonly(searchConfig),
    searchQuery,
    setSearch,
    clearSearch,
  };
}
