<script setup lang="ts">
defineProps<{
  label: string;
  name: string;
  placeholder?: string;
  hint?: string;
  disabled?: boolean;
  items: any[];
}>();

const model = defineModel<string>();
</script>

<template>
  <UFormField
    v-slot="{ error }"
    :label="label"
    :hint="hint"
    :name="name"
    :ui="{

      error: 'text-red-500 mt-1',
      label: 'text-xs font-medium text-gray-600 dark:text-gray-400',
      hint: 'mr-auto text-gray-400 dark:text-gray-600 font-normal ml-1',
    }"
  >
    <USelectMenu
      v-model="model"
      :items="items"
      :disabled="disabled"
      :placeholder="placeholder"
      :ui="{ base: 'py-4 pl-4 lowercase' }"
      :class="[
        'w-full transition-colors',
        error
          ? 'border-red-500 focus:border-red-500'
          : 'border-gray-300 focus:border-black',
      ]"
    />
  </UFormField>
</template>
