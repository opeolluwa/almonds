<script setup lang="ts">
defineProps<{
  title: string;
  language: string;
  lines: number;
  date: string;
  preview: string;
}>();

defineEmits<{
  copy: [];
  edit: [];
}>();
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow overflow-hidden"
  >
    <div class="p-4">
      <div class="flex items-center justify-between mb-2">
        <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200">
          {{ title }}
        </h3>
        <div class="flex items-center gap-3">
          <span
            class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-xs text-gray-500 dark:text-gray-400"
            >{{ language }}</span
          >
          <span class="text-xs text-gray-400">{{ lines }} lines</span>
        </div>
      </div>
      <pre
        class="bg-gray-900 text-gray-100 rounded-md p-3 text-xs overflow-x-auto"
      ><code>{{ preview }}</code></pre>
    </div>
    <div
      class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
    >
      <p class="text-xs text-gray-400">{{ date }}</p>
      <div class="flex items-center gap-2">
        <button
          class="text-xs text-accent-600 dark:text-accent-400 hover:text-accent-700 font-medium"
          @click="$emit('copy')"
        >
          Copy
        </button>
        <button
          class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          @click="$emit('edit')"
        >
          Edit
        </button>
      </div>
    </div>
  </div>
</template>
