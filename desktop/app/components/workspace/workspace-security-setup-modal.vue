<script setup lang="ts">
import type { Workspace } from "~/stores/workspaces";

const props = defineProps<{
  open: boolean;
  workspaceId: string;
  workspaceName: string;
}>();

const emit = defineEmits<{
  close: [];
}>();

const workspaceStore = useWorkspacesStore();
const securityMode = ref<"new" | "reuse">("new");
const securityPassword = ref("");
const securityConfirm = ref("");
const reuseSourceId = ref("");
const securityError = ref("");
const securitySetupLoading = ref(false);

const securedWorkspaces = computed(() =>
  workspaceStore.workspaces.filter(
    (w) => w.isSecured && w.identifier !== props.workspaceId,
  ),
);

watch(
  () => props.open,
  (val) => {
    if (val) {
      securityMode.value = "new";
      securityPassword.value = "";
      securityConfirm.value = "";
      reuseSourceId.value = "";
      securityError.value = "";
    }
  },
);

function switchToNew() {
  securityMode.value = "new";
  securityPassword.value = "";
  securityConfirm.value = "";
  securityError.value = "";
}

function switchToReuse() {
  securityMode.value = "reuse";
  securityPassword.value = "";
  securityConfirm.value = "";
  reuseSourceId.value = securedWorkspaces.value[0]?.identifier ?? "";
  securityError.value = "";
}

async function submitSecuritySetup() {
  if (!props.workspaceId) return;
  securityError.value = "";

  if (securityMode.value === "new") {
    if (!securityPassword.value) {
      securityError.value = "Password is required.";
      return;
    }
    if (securityPassword.value !== securityConfirm.value) {
      securityError.value = "Passwords do not match.";
      return;
    }
  } else {
    if (!reuseSourceId.value || !securityPassword.value) {
      securityError.value = "Select a workspace and enter its password.";
      return;
    }
    const ok = await workspaceStore.verifyWorkspacePassword(
      reuseSourceId.value,
      securityPassword.value,
    );
    if (!ok) {
      securityError.value = "Incorrect password for the selected workspace.";
      return;
    }
  }

  securitySetupLoading.value = true;
  try {
    await workspaceStore.updateWorkspace(props.workspaceId, {
      isSecured: true,
      password: securityPassword.value,
    });
    workspaceStore.unlockWorkspace(props.workspaceId);
    emit("close");
  } catch (e) {
    securityError.value = (e as Error).message || "Failed to secure workspace.";
  } finally {
    securitySetupLoading.value = false;
  }
}
</script>

<template>
  <UModal :open="open" @close="$emit('close')">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Secure "{{ workspaceName }}"
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Optionally protect this workspace with a password.
        </p>
      </div>

      <div class="px-6 pb-6 mt-4 flex flex-col gap-4">
        <div class="flex gap-2">
          <button
            class="flex-1 rounded-lg border px-3 py-2 text-xs font-medium transition-colors"
            :class="
              securityMode === 'new'
                ? 'border-accent-500 bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300'
                : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            "
            @click="switchToNew"
          >
            New password
          </button>
          <button
            v-if="securedWorkspaces.length > 0"
            class="flex-1 rounded-lg border px-3 py-2 text-xs font-medium transition-colors"
            :class="
              securityMode === 'reuse'
                ? 'border-accent-500 bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300'
                : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            "
            @click="switchToReuse"
          >
            Reuse existing profile
          </button>
        </div>

        <template v-if="securityMode === 'new'">
          <AppInput
            v-model="securityPassword"
            label="Password"
            type="password"
            name="new-ws-password"
            placeholder="Enter password"
            :disabled="securitySetupLoading"
          />
          <AppInput
            v-model="securityConfirm"
            label="Confirm password"
            type="password"
            name="new-ws-password-confirm"
            placeholder="Confirm password"
            :disabled="securitySetupLoading"
          />
        </template>

        <template v-else>
          <UFormField label="Copy from workspace">
            <USelect
              v-model="reuseSourceId"
              :items="
                securedWorkspaces.map((w) => ({
                  label: w.name,
                  value: w.identifier,
                }))
              "
              class="w-full"
              :disabled="securitySetupLoading"
            />
          </UFormField>
          <AppInput
            v-model="securityPassword"
            :label="`Password for &quot;${securedWorkspaces.find((w) => w.identifier === reuseSourceId)?.name ?? 'workspace'}&quot;`"
            type="password"
            name="reuse-ws-password"
            placeholder="Enter that workspace's password"
            :disabled="securitySetupLoading"
          />
        </template>

        <p v-if="securityError" class="text-xs text-red-500 dark:text-red-400">
          {{ securityError }}
        </p>

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="securitySetupLoading"
            @click="$emit('close')"
          >
            Skip
          </UButton>
          <UButton
            type="button"
            color="primary"
            :loading="securitySetupLoading"
            :disabled="!securityPassword || securitySetupLoading"
            @click="submitSecuritySetup"
          >
            Secure workspace
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
