<script setup lang="ts">
import { useReminderStore } from "~/stores/reminder";

definePageMeta({ layout: false });

const reminderStore = useReminderStore();
const router = useRouter();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "upcoming" | "recurring">("all");

const filteredReminders = computed(() => {
  let list = reminderStore.reminders;

  if (filter.value === "upcoming") list = reminderStore.upcomingReminders;
  if (filter.value === "recurring") list = reminderStore.recurringReminders;

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (r) =>
        r.title.toLowerCase().includes(q) ||
        r.description?.toLowerCase().includes(q),
    );
  }

  return list;
});

function formatRemindAt(remindAt: string): string {
  return new Date(remindAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

async function handleDelete(identifier: string) {
  await reminderStore.deleteReminder(identifier);
}

function handleEdit(identifier: string) {
  router.push(`/reminders/edit-reminder?id=${identifier}`);
}

onMounted(async () => {
  setSearch({ placeholder: "Search reminders..." });
  await reminderStore.fetchReminders();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <PrimaryCta
        v-if="reminderStore.reminders.length !== 0"
        label="New Reminder"
        icon="heroicons:plus"
        to="/reminders/create-reminder"
      />
    </template>

    <template #main_content>
      <!-- Filter tabs -->
      <div
        v-if="!reminderStore.loading && reminderStore.reminders.length > 0"
        class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5 mb-4 w-fit"
      >
        <button
          v-for="f in ['all', 'upcoming', 'recurring'] as const"
          :key="f"
          class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
          :class="
            filter === f
              ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
              : 'text-gray-500 dark:text-gray-400'
          "
          @click="filter = f"
        >
          {{ f }}
        </button>
      </div>

      <!-- Loading -->
      <div v-if="reminderStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <!-- Empty state: no reminders at all -->
      <div
        v-else-if="reminderStore.reminders.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div
          class="mb-4 p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:clock"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No reminders yet
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Create your first reminder to stay on track.
        </p>
        <NuxtLink
          to="/reminders/create-reminder"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
        >
          Create reminder
        </NuxtLink>
      </div>

      <!-- Empty state: search or filter yields no results -->
      <div
        v-else-if="filteredReminders.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-4 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon
            name="heroicons:magnifying-glass"
            class="w-8 h-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No results found
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Try a different search or filter.
        </p>
        <div class="flex gap-3">
          <button
            v-if="searchQuery"
            class="text-xs text-accent-500 hover:text-accent-600 font-medium"
            @click="searchQuery = ''"
          >
            Clear search
          </button>
          <button
            v-if="filter !== 'all'"
            class="text-xs text-gray-400 hover:text-gray-600 font-medium"
            @click="filter = 'all'"
          >
            Clear filter
          </button>
        </div>
      </div>

      <!-- Reminder list -->
      <div v-else class="flex flex-col gap-2">
        <div
          v-for="reminder in filteredReminders"
          :key="reminder.identifier"
          class="flex items-start gap-3 bg-white dark:bg-gray-800 rounded-lg px-4 py-3 border border-gray-100 dark:border-gray-700"
        >
          <div
            class="mt-0.5 p-1.5 rounded-md"
            :class="
              reminder.recurring
                ? 'bg-accent-50 dark:bg-accent-950'
                : 'bg-gray-100 dark:bg-gray-700'
            "
          >
            <UIcon
              :name="reminder.recurring ? 'heroicons:arrow-path' : 'heroicons:clock'"
              class="size-4"
              :class="
                reminder.recurring
                  ? 'text-accent-500'
                  : 'text-gray-400 dark:text-gray-500'
              "
            />
          </div>

          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">
              {{ reminder.title }}
            </p>
            <p
              v-if="reminder.description"
              class="text-xs text-gray-400 dark:text-gray-500 mt-0.5 truncate"
            >
              {{ reminder.description }}
            </p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
              {{ formatRemindAt(reminder.remindAt) }}
            </p>
          </div>

          <div class="flex items-center gap-1 shrink-0">
            <button
              class="p-1.5 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              @click="handleEdit(reminder.identifier)"
            >
              <UIcon name="heroicons:pencil" class="size-4" />
            </button>
            <button
              class="p-1.5 rounded-md text-gray-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors"
              @click="handleDelete(reminder.identifier)"
            >
              <UIcon name="heroicons:trash" class="size-4" />
            </button>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Summary
      </h2>
      <div class="flex flex-col gap-3 mb-6">
        <div class="bg-accent-50 dark:bg-accent-950 rounded-lg p-3">
          <p class="text-2xl font-semibold text-accent-700 dark:text-accent-300">
            {{ reminderStore.upcomingReminders.length }}
          </p>
          <p class="text-xs text-accent-500 dark:text-accent-400">
            Upcoming
          </p>
        </div>
        <div class="bg-violet-50 dark:bg-violet-950 rounded-lg p-3">
          <p class="text-2xl font-semibold text-violet-700 dark:text-violet-300">
            {{ reminderStore.recurringReminders.length }}
          </p>
          <p class="text-xs text-violet-500 dark:text-violet-400">
            Recurring
          </p>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Total
      </h2>
      <p class="text-2xl font-semibold text-gray-700 dark:text-gray-200">
        {{ reminderStore.reminders.length }}
      </p>
      <p class="text-xs text-gray-400 dark:text-gray-500">reminders</p>
    </template>
  </NuxtLayout>
</template>
