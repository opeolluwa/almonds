<script setup lang="ts">
withDefaults(
  defineProps<{
    modelValue: string;
    disabled?: boolean;
  }>(),
  { disabled: false },
);

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

function handleInput(event: Event) {
  const target = event.target as HTMLTextAreaElement;
  target.style.height = "auto";
  target.style.height = `${target.scrollHeight}px`;
  emit("update:modelValue", target.value);
}
</script>

<template>
  <textarea
    :value="modelValue"
    placeholder="Untitled"
    rows="1"
    :disabled="disabled"
    class="w-full resize-none bg-transparent outline-none text-xl md:text-3xl font-bold text-gray-900 dark:text-gray-200 placeholder:text-gray-400 dark:placeholder:text-gray-500 leading-tight mb-0 overflow-hidden"
    @input="handleInput"
  />
</template>
