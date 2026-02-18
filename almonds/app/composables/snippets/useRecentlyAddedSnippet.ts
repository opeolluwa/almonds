import { invoke } from "@tauri-apps/api/core";

export interface Snippet {
  identifier: string;
  title: string | null;
  language: string | null;
  code: string;
  description: string | null;
  isPinned: boolean;
  createdAt: string;
  updatedAt: string;
}

export function useRecentlyAddedSnippet() {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const snippetStore = useSnippetStore();

  async function fetchRecentSnippets() {
    loading.value = true;
    error.value = null;
    try {
      const snippets = await invoke<Snippet[]>("get_recently_added_snippet");
      snippetStore.recent = snippets;
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  return {
    loading,
    error,
    fetchRecentSnippets,
  };
}
