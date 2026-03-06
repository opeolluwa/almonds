import { useWorkspacesStore } from "~/stores/workspaces";

export function useWorkspaceSetup() {
  const store = useWorkspacesStore();
  const initializing = ref(true);
  const initialized = ref(false);

  const setupRequired = computed(
    () => initialized.value && store.workspaces.length === 0,
  );

  async function checkSetup() {
    try {
      await store.fetchWorkspaces();
    } finally {
      initialized.value = true;
      initializing.value = false;
    }
  }

  return { setupRequired, checkSetup, initializing };
}
