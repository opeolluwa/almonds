<script setup lang="ts">
interface Props {
  title: string;
  description: string;
  icon?: string;
  actionLabel?: string;
}

withDefaults(defineProps<Props>(), {
  icon: "heroicons:users",
});

const emit = defineEmits<{
  action: [];
}>();
</script>

<template>
  <div class="flex h-[60vh] flex-col items-center justify-center text-center">
    <div
      class="flex size-12 items-center justify-center rounded-2xl bg-primary-50/50 p-0.5 dark:bg-white/5"
    >
      <UIcon :name="icon" class="size-8 text-gray-3 dark:text-white/20" />
    </div>

    <div>
      <p class="mt-3 text-base font-medium">
        {{ title }}
      </p>

      <p class="mt-1 text-sm text-muted">
        {{ description }}
      </p>
    </div>

    <AppButton
      v-if="actionLabel"
      :size="'lg'"
      class="mt-6"
      @click="emit('action')"
    >
      {{ actionLabel }}
    </AppButton>
  </div>
</template>
