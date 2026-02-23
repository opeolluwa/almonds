<script setup lang="ts">
import { useReminderStore } from "~/stores/reminder";

definePageMeta({ layout: false, name: "Create Reminder" });

const reminderStore = useReminderStore();
const router = useRouter();
const { notify } = useAppNotification();

const form = reactive({
  title: "",
  description: "",
  remindAt: "",
  recurring: false,
  recurrenceRule: "",
  alarmSound: "",
});

const submitting = ref(false);

async function handleSubmit() {
  if (!form.title.trim() || !form.remindAt) return;
  submitting.value = true;
  try {
    await reminderStore.createReminder({
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      remindAt: new Date(form.remindAt).toISOString(),
      recurring: form.recurring || undefined,
      recurrenceRule: form.recurrenceRule.trim() || undefined,
      alarmSound: form.alarmSound.trim() || undefined,
    });
    notify({ type: "success", message: "Reminder created" });
    router.push("/reminders");
  } catch {
    notify({ type: "error", message: "Failed to create reminder" });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-lg">
        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Title <span class="text-rose-500">*</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              placeholder="What should I remind you about?"
              autofocus
              class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500"
            >
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Description
            </label>
            <textarea
              v-model="form.description"
              placeholder="Add more details..."
              rows="3"
              class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500 resize-none"
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Remind at <span class="text-rose-500">*</span>
            </label>
            <div class="relative">
              <UIcon
                name="heroicons:clock"
                class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-gray-400 pointer-events-none"
              />
              <input
                v-model="form.remindAt"
                type="datetime-local"
                class="w-full bg-white dark:bg-gray-800 rounded-lg pl-9 pr-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
              >
            </div>
          </div>

          <div class="flex items-center gap-3">
            <button
              type="button"
              class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 focus:outline-none"
              :class="
                form.recurring
                  ? 'bg-accent-500'
                  : 'bg-gray-200 dark:bg-gray-700'
              "
              @click="form.recurring = !form.recurring"
            >
              <span
                class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow transform transition duration-200"
                :class="form.recurring ? 'translate-x-4' : 'translate-x-0'"
              />
            </button>
            <label
              class="text-sm text-gray-600 dark:text-gray-400 cursor-pointer select-none"
              @click="form.recurring = !form.recurring"
            >
              Recurring reminder
            </label>
          </div>

          <div v-if="form.recurring" class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Recurrence rule
            </label>
            <input
              v-model="form.recurrenceRule"
              type="text"
              placeholder="e.g. FREQ=DAILY;INTERVAL=1"
              class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500"
            >
          </div>

          <div class="flex justify-end gap-3 pt-2">
            <button
              type="button"
              class="px-4 py-2 rounded-lg text-sm text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              @click="router.back()"
            >
              Cancel
            </button>
            <button
              type="submit"
              :disabled="!form.title.trim() || !form.remindAt || submitting"
              class="px-4 py-2 rounded-lg text-sm font-medium bg-accent-500 text-white hover:bg-accent-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {{ submitting ? "Creating..." : "Create" }}
            </button>
          </div>
        </form>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tips
      </h2>
      <div class="flex flex-col gap-3">
        <div
          class="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <UIcon
            name="heroicons:clock"
            class="size-4 text-accent-500 shrink-0 mt-px"
          />
          <span>
            <strong class="text-gray-700 dark:text-gray-300">One-time</strong>
            — fires once at the scheduled time
          </span>
        </div>
        <div
          class="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <UIcon
            name="heroicons:arrow-path"
            class="size-4 text-violet-500 shrink-0 mt-px"
          />
          <span>
            <strong class="text-gray-700 dark:text-gray-300">Recurring</strong>
            — repeats on a schedule using an iCalendar recurrence rule
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
