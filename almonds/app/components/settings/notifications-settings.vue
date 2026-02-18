<script setup lang="ts">
const notifTasksDue = ref(true);
const notifReminders = ref(true);
const notifSyncComplete = ref(false);
const notifAppUpdates = ref(true);

const items = computed(() => [
  { label: "Tasks due", desc: "Remind me when tasks are due", model: notifTasksDue },
  { label: "Reminders", desc: "Calendar event reminders", model: notifReminders },
  { label: "Sync complete", desc: "Notify when sync finishes", model: notifSyncComplete },
  { label: "App updates", desc: "Notify about new versions", model: notifAppUpdates },
]);
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Notifications
      </h2>
      <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700">
        <div
          v-for="item in items"
          :key="item.label"
          class="flex items-center justify-between py-3"
        >
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">
              {{ item.label }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ item.desc }}</p>
          </div>
          <button
            class="relative w-10 h-6 rounded-full transition-colors"
            :class="
              item.model.value
                ? 'bg-accent-500'
                : 'bg-gray-200 dark:bg-gray-600'
            "
            @click="item.model.value = !item.model.value"
          >
            <span
              class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
              :class="item.model.value ? 'translate-x-4' : 'translate-x-0'"
            />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
