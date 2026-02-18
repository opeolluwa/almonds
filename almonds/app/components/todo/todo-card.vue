<script setup lang="ts">
import type { Todo } from "~/stores/todo";

defineProps<{
  todo: Todo;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

const priorityColor: Record<string, string> = {
  high: "text-rose-500",
  medium: "text-amber-500",
  low: "text-emerald-500",
};

function formatDueDate(dateStr: string | null) {
  if (!dateStr) return null;
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 flex items-center gap-4 hover:shadow-sm transition-shadow group"
  >
    <button @click="emit('toggle', todo.identifier, !todo.done)">
      <UIcon
        :name="todo.done ? 'heroicons:check-circle-solid' : 'heroicons:circle'"
        class="size-5 transition-colors"
        :class="
          todo.done
            ? 'text-accent-500'
            : 'text-gray-300 dark:text-gray-600 hover:text-gray-400'
        "
      />
    </button>

    <div class="flex-1 min-w-0">
      <p
        class="text-sm transition-colors"
        :class="
          todo.done
            ? 'line-through text-gray-400'
            : 'text-gray-700 dark:text-gray-200'
        "
      >
        {{ todo.title }}
      </p>
      <p v-if="todo.description" class="text-xs text-gray-400 truncate mt-0.5">
        {{ todo.description }}
      </p>
    </div>

    <UIcon
      name="heroicons:flag"
      class="size-4 shrink-0"
      :class="priorityColor[todo.priority]"
    />

    <span v-if="todo.dueDate" class="text-xs text-gray-400 shrink-0">
      {{ formatDueDate(todo.dueDate) }}
    </span>

    <div
      class="flex items-center gap-1 opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity"
    >
      <button
        v-if="!todo.done"
        class="flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium text-emerald-600 dark:text-emerald-400 hover:bg-emerald-50 dark:hover:bg-emerald-950 transition-colors"
        @click="emit('toggle', todo.identifier, true)"
      >
        <UIcon name="heroicons:check" class="size-3.5" />
        Done
      </button>
      <button
        class="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
        @click="emit('edit', todo.identifier)"
      >
        <UIcon name="heroicons:pencil" class="size-3.5" />
      </button>
      <button
        class="p-1 text-gray-400 hover:text-rose-500 transition-colors"
        @click="emit('delete', todo.identifier)"
      >
        <UIcon name="heroicons:trash" class="size-3.5" />
      </button>
    </div>
  </div>
</template>
