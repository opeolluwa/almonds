<template>
  <UButton
    :type="type"
    :color="color"
    :loading="loading"
    :size="size"
    :disabled="disabled || loading"
    :ui="{ base: 'py-3 pl-4' }"
  >
    <slot />
  </UButton>
</template>

<script setup lang="ts">
type Color =
  | "primary"
  | "error"
  | "secondary"
  | "success"
  | "info"
  | "warning"
  | "neutral"
  | undefined;
type Type = "button" | "submit" | "reset" | undefined;
defineProps({
  type: {
    type: String as PropType<"button" | "submit" | "reset">,
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
    default: "md",
  },
});
</script>
