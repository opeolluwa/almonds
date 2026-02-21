import { useUserPreferenceStore } from "~/stores/user-preference";

export function useUserSetup() {
  const store = useUserPreferenceStore();
  const initialized = ref(false);

  const setupRequired = computed(
    () => initialized.value && store.preference === null,
  );

  async function checkSetup() {
    await store.fetchPreference();
    initialized.value = true;
  }

  return { setupRequired, checkSetup };
}
