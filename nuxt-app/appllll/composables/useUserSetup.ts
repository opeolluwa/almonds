import { useUserPreferenceStore } from "~/stores/workspace-preferences";

export function useUserSetup() {
  const store = useUserPreferenceStore();
  const initializing = ref(true);
  const initialized = ref(false);

  const setupRequired = computed(
    () => initialized.value && store.preference === null,
  );

  async function checkSetup() {
    try {
      await store.fetchPreference();
    } finally {
      initialized.value = true;
      initializing.value = false;
    }
  }

  return { setupRequired, checkSetup, initializing };
}
