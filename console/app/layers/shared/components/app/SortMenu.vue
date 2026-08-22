<script setup lang="ts" generic="T extends string">
import type { DropdownMenuItem } from "@nuxt/ui";
import type { SortOption } from "@shared/utils/sorting";

const model = defineModel<T>({ required: true });

const props = defineProps<{
  options: SortOption<T>[][];
}>();

const items = computed<DropdownMenuItem[][]>(() =>
  props.options.map((group) =>
    group.map((option) => ({
      label: option.label,
      icon:
        model.value === option.value ? "heroicons:check" : (option.icon ?? ""),
      onSelect: () => {
        model.value = option.value;
      },
    })),
  ),
);
</script>

<template>
  <UDropdownMenu
    :items="items"
    size="sm"
    :ui="{
      content:
        'min-w-48 rounded-xl shadow-lg border border-gray-100 dark:border-gray-800 py-1',
      item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
      separator: 'my-1 mx-2',
    }"
  >
    <button
      class="flex shrink-0 items-center gap-1.5 px-2.5 py-1.5 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-xs font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
    >
      <UIcon name="heroicons:arrows-up-down" class="size-3.5" />
      Sort
    </button>
  </UDropdownMenu>
</template>
