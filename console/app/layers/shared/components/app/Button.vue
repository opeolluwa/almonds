<template>
  <UButton
    :type="type"
    :color="color"
    :loading="loading"
    :size="size"
    :disabled="disabled || loading"
    class="w-full py-3 bg-accent-500 text-center hover:bg-accent-600 cursor-pointer text-white font-medium disabled:opacity-50"
  >
    <slot />
  </UButton>
</template>

<script setup lang="ts">
defineProps({
  type: {
    type: String as PropType<"button" | "submit" | "reset" | "link">,
    default: "button",
  },
  color: {
    type: String as PropType<
      | "primary"
      | "error"
      | "secondary"
      | "success"
      | "info"
      | "warning"
      | "neutral"
    >,
    default: "primary",
  },
  loading: {
    type: Boolean,
    default: false,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  error: {
    type: Boolean,
    default: false,
  },
  size: {
    type: String as PropType<"xs" | "sm" | "md" | "lg" | "xl">,
    default: "lg",
  },
});
</script>
