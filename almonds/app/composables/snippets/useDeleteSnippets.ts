import { invoke } from "@tauri-apps/api/core";
import { useGetSnippets } from "./useGetSnippets";

export function useDeleteSnippet() {
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function deleteSnippet(identifier: string) {
    loading.value = true;
    error.value = null;
    try {
      await invoke("delete_snippet", { identifier });
      const { fetchSnippets } = useGetSnippets();
      await fetchSnippets();
      return true;
    } catch (e) {
      error.value = String(e);
      return false;
    } finally {
      loading.value = false;
    }
  }

  return { deleteSnippet, loading, error };
}
