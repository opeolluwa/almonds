<script setup lang="ts"></script>

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
      <AppCreateWorkspaceForm />
    </template>
  </UModal>
</template>
