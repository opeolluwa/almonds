<script setup lang="ts">
const props = defineProps<{
  open: boolean;
  workspaceId: string | null;
}>();

const emit = defineEmits<{
  close: [];
}>();

const workspaceStore = useWorkspacesStore();
const passwordInput = ref("");
const passwordError = ref("");
const verifyingPassword = ref(false);

watch(
  () => props.open,
  (val) => {
    if (val) {
      passwordInput.value = "";
      passwordError.value = "";
    }
  },
);

async function submitPassword() {
  if (!props.workspaceId || !passwordInput.value) return;
  verifyingPassword.value = true;
  passwordError.value = "";
  try {
    const ok = await workspaceStore.verifyWorkspacePassword(
      props.workspaceId,
      passwordInput.value,
    );
    if (ok) {
      workspaceStore.unlockWorkspace(props.workspaceId);
      await workspaceStore.setActiveWorkspace(props.workspaceId);
      emit("close");
    } else {
      passwordError.value = "Incorrect password. Please try again.";
    }
  } catch {
    passwordError.value = "Failed to verify password.";
  } finally {
    verifyingPassword.value = false;
  }
}
</script>

<template>
  <UModal :open="open" @close="$emit('close')">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Secured Workspace
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Enter the password to access this workspace.
        </p>
      </div>
      <form
        class="px-6 pb-6 mt-4 flex flex-col gap-4"
        @submit.prevent="submitPassword"
      >
        <AppInput
          v-model="passwordInput"
          label="Password"
          type="password"
          name="workspace-password"
          placeholder="Enter password"
          :error="passwordError"
          :disabled="verifyingPassword"
          autofocus
        />
        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="verifyingPassword"
            @click="$emit('close')"
          >
            Cancel
          </UButton>
          <UButton
            type="submit"
            color="primary"
            :loading="verifyingPassword"
            :disabled="!passwordInput || verifyingPassword"
          >
            Unlock
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
