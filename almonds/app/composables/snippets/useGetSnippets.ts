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

export function useGetSnippets() {
  const snippets = useState<Snippet[]>("snippets", () => []);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchSnippets() {
    loading.value = true;
    error.value = null;
    try {
      snippets.value = await invoke<Snippet[]>("get_all_snippets");
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function fetchSnippet(identifier: string) {
    try {
      return await invoke<Snippet | null>("get_snippet", { identifier });
    } catch (e) {
      error.value = String(e);
      return null;
    }
  }

  return {
    snippets: readonly(snippets),
    loading,
    error,
    fetchSnippets,
    fetchSnippet,
  };
}
