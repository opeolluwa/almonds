<script setup lang="ts">
defineProps<{
  label: string;
  name: string;
  placeholder?: string;
  hint?: string;
  disabled?: boolean;
  type?: string;
}>();

const model = defineModel<string>();
</script>

<template>
  <UFormField
    v-slot="{ error }"
    :name="name"
    :label="label"
    :hint="hint"
    :ui="{
      error: 'text-red-500 mt-1',
      label: 'text-xs font-medium text-gray-600 dark:text-gray-400',
      hint: ' mr-auto text-gray-400 dark:text-gray-600 font-normal ml-1',
    }"
  >
    <UInput
      v-model="model"
      :disabled="disabled"
      :placeholder="placeholder"
      :type="type"
      :ui="{ base: 'py-3 pl-4 bg-transparent' }"
      :class="[
        'w-full transition-colors',
        error
          ? 'border-red-500 focus-within:border-red-500'
          : 'border-gray-300 dark:border-gray-600 focus-within:border-black dark:focus-within:border-gray-400',
      ]"
    />
  </UFormField>
</template>
