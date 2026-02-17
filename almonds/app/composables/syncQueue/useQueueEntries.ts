import { invoke } from "@tauri-apps/api/core";

export interface SyncQueueEntry {
  identifier: string;
  tableName: string;
  recordIdentifier: string;
  operation: string;
  createdAt: string;
}

export function useQueueEntries() {
  const entries = useState<SyncQueueEntry[]>("syncQueueEntries", () => []);
  const count = useState<number>("syncQueueCount", () => 0);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchEntries() {
    loading.value = true;
    error.value = null;
    try {
      entries.value = await invoke<SyncQueueEntry[]>("run_sync");
      count.value = entries.value.length;
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function fetchCount() {
    try {
      count.value = await invoke<number>("count_sync_queue_entries");
    } catch (e) {
      error.value = String(e);
    }
  }

  return {
    entries: readonly(entries),
    count: readonly(count),
    loading,
    error,
    fetchEntries,
    fetchCount,
  };
}
