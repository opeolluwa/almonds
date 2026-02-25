<script setup lang="ts">
import { useWorkspacesStore } from "~/stores/workspaces";

const store = useWorkspacesStore();

const form = reactive({
  name: "",
  description: "",
});

const loading = ref(false);
const errors = reactive({
  name: "",
  description: "",
});

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    await store.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
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
          Welcome to Almonds
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Create your first workspace to get started. You can have multiple
          workspaces for different projects or contexts, and switch between them
          seamlessly.
        </p>
      </div>
    </template>

    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
              >Name</label
            >
            <UInput
              v-model="form.name"
              placeholder="My Workspace"
              :disabled="loading"
              :ui="{ root: errors.name ? 'ring-1 ring-red-500' : '' }"
            />
            <p v-if="errors.name" class="text-xs text-red-500">
              {{ errors.name }}
            </p>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
              >Description</label
            >
            <UInput
              v-model="form.description"
              placeholder="My Workspace Description"
              :disabled="loading"
              :ui="{ root: errors.description ? 'ring-1 ring-red-500' : '' }"
            />
            <p v-if="errors.description" class="text-xs text-red-500">
              {{ errors.description }}
            </p>
          </div>
        </div>

        <div class="flex justify-end pt-1">
          <UButton
            type="submit"
            color="primary"
            :loading="loading"
            :disabled="loading"
          >
            Save and continue
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
