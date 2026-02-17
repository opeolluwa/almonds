import { invoke } from "@tauri-apps/api/core";
import { useQueueEntries } from "./useQueueEntries";

export function usePopFromQueue() {
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function popFromQueue(identifier: string) {
    loading.value = true;
    error.value = null;
    try {
      await invoke("remove_sync_queue_entry", { identifier });
      const { fetchEntries } = useQueueEntries();
      await fetchEntries();
      return true;
    } catch (e) {
      error.value = String(e);
      return false;
    } finally {
      loading.value = false;
    }
  }

  return { popFromQueue, loading, error };
}
