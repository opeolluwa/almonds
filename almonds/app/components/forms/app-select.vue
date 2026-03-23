<script setup lang="ts">
defineProps<{
  label: string;
  icon?: string;
  className?: string;
  name: string;
  trailingIcon?: string;
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
    :ui="{
      error: 'text-red-500 mt-1',
      label: 'text-xs font-medium text-gray-600 dark:text-gray-400',
      hint: 'mr-auto text-gray-400 dark:text-gray-600 font-normal ml-1',
      root: 'lowercase border-2 border-red-500',
    }"
  >
    <USelectMenu
      v-model="model"
      :items="items"
      :icon="icon"
      :trailing-icon="trailingIcon"
      value-key="value"
      :disabled="disabled"
      :placeholder="placeholder"
      :ui="{
        base: 'lowercase',
        trailingIcon: 'size-10',
      }"
      :class="[
        'w-full transition-colors first:capitalize bg-transparent ' + className,
        error
          ? 'border-red-500 focus:border-red-500'
          : 'border-gray-300 focus:border-black',
      ]"
    />
  </UFormField>
</template>
