import { invoke } from "@tauri-apps/api/core";
import { useGetSnippets, type Snippet } from "./useGetSnippets";

export interface CreateSnippetPayload {
  title: string | null;
  language: string | null;
  code: string;
  description: string | null;
}

export function useCreateSnippet() {
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function createSnippet(payload: CreateSnippetPayload) {
    loading.value = true;
    error.value = null;
    try {
      const created = await invoke<Snippet>("create_snippet", { snippet: payload });
      const { fetchSnippets } = useGetSnippets();
      await fetchSnippets();
      return created;
    } catch (e) {
      error.value = String(e);
      return null;
    } finally {
      loading.value = false;
    }
  }

  return { createSnippet, loading, error };
}
