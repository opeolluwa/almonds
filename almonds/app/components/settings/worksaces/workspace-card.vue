<script setup lang="ts">
import type { Workspace } from "~/stores/workspaces";

defineProps<{ workspace: Workspace }>();

const emit = defineEmits<{
  delete: [identifier: string];
  edit: [identifier: string];
  toggleHidden: [identifier: string];
  setDefault: [identifier: string];
}>();

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
</script>

<template>
  <div
    class="group bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow flex items-center gap-4"
  >
    <UIcon name="heroicons:briefcase" class="size-5 text-accent-500 shrink-0" />
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <h3
          class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate"
        >
          {{ workspace.name }}
        </h3>
        <span
          v-if="workspace.isDefault"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-accent-100 dark:bg-accent-900 text-accent-600 dark:text-accent-300 shrink-0"
        >
          default
        </span>
        <span
          v-if="workspace.isHidden"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 shrink-0"
        >
          hidden
        </span>
      </div>
      <div class="text-xs text-gray-400 truncate block">
        {{ workspace.description }}
      </div>
    </div>
    <p class="text-xs text-gray-400 shrink-0 hidden sm:block">
      {{ formatDate(workspace.createdAt) }}
    </p>
    <div
      class="opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity flex items-center gap-1"
    >
      <UTooltip
        :text="workspace.isHidden ? 'Show workspace' : 'Hide workspace'"
      >
        <button
          class="text-gray-400 hover:text-accent-500 transition-colors"
          @click="emit('toggleHidden', workspace.identifier)"
        >
          <UIcon
            :name="workspace.isHidden ? 'heroicons:eye' : 'heroicons:eye-slash'"
            class="size-4"
          />
        </button>
      </UTooltip>
      <UTooltip text="Set as default workspace">
        <button
          v-if="!workspace.isDefault"
          class="text-gray-400 hover:text-accent-500 transition-colors"
          @click="emit('setDefault', workspace.identifier)"
        >
          <UIcon name="heroicons:star" class="size-4" />
        </button>
      </UTooltip>
      <MetaControls
        item-name="workspace"
        :show-edit="true"
        :show-duplicate="false"
        :show-transfer="false"
        @edit-record="emit('edit', workspace.identifier)"
        @delete-record="emit('delete', workspace.identifier)"
      />
    </div>
  </div>
</template>
