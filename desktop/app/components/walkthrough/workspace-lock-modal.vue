<script setup lang="ts">
const emit = defineEmits<{ unlocked: [] }>();

const workspaceStore = useWorkspacesStore();

const password = ref("");
const error = ref("");
const loading = ref(false);

const workspaceName = computed(
  () => workspaceStore.currentWorkspace?.name ?? "this workspace",
);

async function submit() {
  const id = workspaceStore.activeWorkspaceId;
  if (!password.value || !id) return;
  loading.value = true;
  error.value = "";
  try {
    const ok = await workspaceStore.verifyWorkspacePassword(id, password.value);
    if (ok) {
      workspaceStore.unlockWorkspace(id);
      emit("unlocked");
    } else {
      error.value = "Incorrect password. Please try again.";
    }
  } catch {
    error.value = "Failed to verify password.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UModal
    :open="true"
    :close="false"
    :dismissible="false"
    @update:open="() => {}"
  >
    <template #header>
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Workspace locked
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Enter the password to access
          <span class="font-medium text-gray-700 dark:text-gray-200">{{
            workspaceName
          }}</span
          >.
        </p>
      </div>
    </template>

    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="submit">
        <AppInput
          v-model="password"
          label="Password"
          type="password"
          name="lock-password"
          placeholder="Enter password"
          :error="error"
          :disabled="loading"
          autofocus
        />
        <div class="flex justify-end pt-1">
          <AppButton
            type="submit"
            :loading="loading"
            :disabled="!password || loading"
          >
            Unlock
          </AppButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
