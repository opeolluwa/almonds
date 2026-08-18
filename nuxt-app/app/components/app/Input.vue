<script setup lang="ts">
withDefaults(
  defineProps<{
    label: string;
    name: string;
    placeholder?: string;
    hint?: string;
    disabled?: boolean;
    type?: string;
    enablePasswordToggle?: boolean;
  }>(),
  {
    enablePasswordToggle: true,
  },
);

const model = defineModel<string>();
const show = ref(false);
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
    <template v-if="type == 'password'">
      <UInput
        v-model="model"
        :disabled="disabled"
        :placeholder="placeholder"
        :type="show ? 'text' : 'password'"
        :ui="{ base: 'py-3 pl-4 bg-transparent' }"
        :class="[
          'w-full transition-colors',
          error
            ? 'border-red-500 focus-within:border-red-500'
            : 'border-gray-300 dark:border-gray-600 focus-within:border-black dark:focus-within:border-gray-400',
        ]"
      >
        <template v-if="enablePasswordToggle" #trailing>
          <UButton
            color="neutral"
            variant="link"
            size="sm"
            :icon="show ? 'i-lucide-eye-off' : 'i-lucide-eye'"
            :aria-label="show ? 'Hide password' : 'Show password'"
            :aria-pressed="show"
            aria-controls="password"
            @click="show = !show"
          />
        </template>
      </UInput>
    </template>
    <template v-else>
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
    /></template>
  </UFormField>
</template>
