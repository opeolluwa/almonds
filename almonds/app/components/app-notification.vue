<script setup lang="ts">
import type { NotificationType } from "~/composables/useAppNotification";

const { notification, dismiss } = useAppNotification();

const visible = computed(() => !!notification.value);

const config: Record<
  NotificationType,
  { icon: string; bar: string; text: string; bg: string }
> = {
  success: {
    icon: "heroicons:check-circle",
    bar: "bg-green-500",
    text: "text-green-600 dark:text-green-400",
    bg: "bg-white dark:bg-gray-900 border border-green-200 dark:border-green-800",
  },
  error: {
    icon: "heroicons:x-circle",
    bar: "bg-accent-500",
    text: "text-accent-600 dark:text-accent-400",
    bg: "bg-white dark:bg-gray-900 border border-accent-200 dark:border-accent-800",
  },
  warning: {
    icon: "heroicons:exclamation-triangle",
    bar: "bg-amber-400",
    text: "text-amber-600 dark:text-amber-400",
    bg: "bg-white dark:bg-gray-900 border border-amber-200 dark:border-amber-800",
  },
  info: {
    icon: "heroicons:information-circle",
    bar: "bg-surface-400",
    text: "text-surface-600 dark:text-surface-300",
    bg: "bg-white dark:bg-gray-900 border border-surface-200 dark:border-surface-700",
  },
};

const current = computed(() => config[notification.value?.type ?? "info"]);
</script>

<template>
  <Teleport to="body">
    <Transition name="toast">
      <div
        v-if="visible && notification"
        class="fixed bottom-6 right-6 z-[9999] w-80 max-w-[calc(100vw-3rem)]"
        role="alert"
        aria-live="assertive"
      >
        <div
          class="relative flex items-start gap-3 rounded-xl shadow-lg overflow-hidden px-4 py-3"
          :class="current.bg"
        >
          <!-- coloured left bar -->
          <span
            class="absolute inset-y-0 left-0 w-1 rounded-l-xl"
            :class="current.bar"
          />

          <!-- icon -->
          <UIcon
            :name="current.icon"
            class="size-5 mt-px shrink-0"
            :class="current.text"
          />

          <!-- message -->
          <p class="flex-1 text-sm text-gray-800 dark:text-gray-100 leading-snug">
            {{ notification.message }}
          </p>

          <!-- dismiss -->
          <button
            class="shrink-0 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Dismiss"
            @click="dismiss"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition:
    opacity 0.25s ease,
    transform 0.25s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(0.75rem) scale(0.97);
}
.toast-enter-to,
.toast-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}
</style>
