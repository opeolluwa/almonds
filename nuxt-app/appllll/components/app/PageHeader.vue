<script setup>
defineProps({
  title: {
    type: String,
    default: "",
  },
  subtitle: {
    type: String,
    default: "",
  },
  ctaText: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["cta"]);

function handleCta() {
  emit("cta");
}
</script>

<template>
  <div class="flex items-start justify-between mb-8">
    <div>
      <h2 class="text-lg font-semibold">{{ title }}</h2>
      <p class="text-sm text-muted mt-1">{{ subtitle }}</p>
    </div>
    <AppButton class="px-4 py-2 shrink-0" @click="handleCta()">
      {{ ctaText }}
    </AppButton>
  </div>
</template>
