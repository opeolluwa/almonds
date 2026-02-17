import { invoke } from "@tauri-apps/api/core";
import { useQueueEntries } from "./useQueueEntries";

export interface AddSyncQueueEntryPayload {
  tableName: string;
  recordIdentifier: string;
  operation: string;
  createdAt: string;
}

export function usePushToQueue() {
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function pushToQueue(entry: AddSyncQueueEntryPayload) {
    loading.value = true;
    error.value = null;
    try {
      await invoke("add_sync_queue_entry", { entry });
      const { fetchCount } = useQueueEntries();
      await fetchCount();
      return true;
    } catch (e) {
      error.value = String(e);
      return false;
    } finally {
      loading.value = false;
    }
  }

  return { pushToQueue, loading, error };
}
