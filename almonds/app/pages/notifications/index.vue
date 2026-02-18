<script setup lang="ts">
definePageMeta({ layout: false });

type NotificationCategory = "system" | "activity" | "reminder" | "alert";

interface Notification {
  id: number;
  title: string;
  message: string;
  category: NotificationCategory;
  read: boolean;
  time: string; // ISO string
}

const notifications = ref<Notification[]>([
  {
    id: 1,
    title: "Snippet saved",
    message: "Your TypeScript utility snippet was saved successfully.",
    category: "activity",
    read: false,
    time: new Date(Date.now() - 1000 * 60 * 4).toISOString(),
  },
  {
    id: 2,
    title: "Sync complete",
    message: "All local data has been synced to the server.",
    category: "system",
    read: false,
    time: new Date(Date.now() - 1000 * 60 * 22).toISOString(),
  },
  {
    id: 3,
    title: "Daily reminder",
    message: "You have 3 tasks due today. Stay on top of it!",
    category: "reminder",
    read: false,
    time: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
  },
  {
    id: 4,
    title: "Storage warning",
    message: "You're using 85% of your available storage quota.",
    category: "alert",
    read: false,
    time: new Date(Date.now() - 1000 * 60 * 60 * 3).toISOString(),
  },
  {
    id: 5,
    title: "Note updated",
    message: '"Project Ideas" was modified from another session.',
    category: "activity",
    read: true,
    time: new Date(Date.now() - 1000 * 60 * 60 * 6).toISOString(),
  },
  {
    id: 6,
    title: "New bookmark added",
    message: '"Nuxt 4 Migration Guide" was bookmarked.',
    category: "activity",
    read: true,
    time: new Date(Date.now() - 1000 * 60 * 60 * 24).toISOString(),
  },
  {
    id: 7,
    title: "App updated",
    message: "Almonds was updated to version 1.4.0. See what's new.",
    category: "system",
    read: true,
    time: new Date(Date.now() - 1000 * 60 * 60 * 48).toISOString(),
  },
  {
    id: 8,
    title: "Weekly summary",
    message: "You completed 12 tasks and added 5 snippets this week.",
    category: "reminder",
    read: true,
    time: new Date(Date.now() - 1000 * 60 * 60 * 72).toISOString(),
  },
]);

type FilterTab = "all" | "unread";
const filter = ref<FilterTab>("all");
const activeCategory = ref<NotificationCategory | "all">("all");

const categoryConfig: Record<
  NotificationCategory,
  { icon: string; color: string; bg: string; label: string }
> = {
  activity: {
    icon: "heroicons:bolt",
    color: "text-surface-500 dark:text-surface-300",
    bg: "bg-surface-100 dark:bg-surface-900",
    label: "Activity",
  },
  system: {
    icon: "heroicons:cog-6-tooth",
    color: "text-gray-500 dark:text-gray-400",
    bg: "bg-gray-100 dark:bg-gray-800",
    label: "System",
  },
  reminder: {
    icon: "heroicons:bell",
    color: "text-amber-500 dark:text-amber-400",
    bg: "bg-amber-50 dark:bg-amber-950",
    label: "Reminder",
  },
  alert: {
    icon: "heroicons:exclamation-triangle",
    color: "text-accent-500 dark:text-accent-400",
    bg: "bg-accent-50 dark:bg-accent-950",
    label: "Alert",
  },
};

const filtered = computed(() => {
  let list = notifications.value;
  if (filter.value === "unread") list = list.filter((n) => !n.read);
  if (activeCategory.value !== "all")
    list = list.filter((n) => n.category === activeCategory.value);
  return list;
});

const unreadCount = computed(
  () => notifications.value.filter((n) => !n.read).length,
);

const categoryCounts = computed(() =>
  (["activity", "system", "reminder", "alert"] as NotificationCategory[]).map(
    (c) => ({
      key: c,
      count: notifications.value.filter((n) => n.category === c).length,
    }),
  ),
);

function markRead(id: number) {
  const n = notifications.value.find((n) => n.id === id);
  if (n) n.read = true;
}

function dismiss(id: number) {
  notifications.value = notifications.value.filter((n) => n.id !== id);
}

function markAllRead() {
  notifications.value.forEach((n) => (n.read = true));
}

function clearAll() {
  notifications.value = [];
}

function relativeTime(iso: string) {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60000);
  if (mins < 1) return "just now";
  if (mins < 60) return `${mins}m ago`;
  const hrs = Math.floor(mins / 60);
  if (hrs < 24) return `${hrs}h ago`;
  const days = Math.floor(hrs / 24);
  return `${days}d ago`;
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Toolbar -->
      <div class="flex items-center justify-between mb-5">
        <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5">
          <button
            v-for="f in ['all', 'unread'] as const"
            :key="f"
            class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
            :class="
              filter === f
                ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
                : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
            "
            @click="filter = f"
          >
            {{ f }}
            <span
              v-if="f === 'unread' && unreadCount > 0"
              class="ml-1 px-1.5 py-0.5 rounded-full text-[10px] bg-accent-500 text-white"
            >
              {{ unreadCount }}
            </span>
          </button>
        </div>

        <button
          v-if="unreadCount > 0"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
          @click="markAllRead"
        >
          Mark all as read
        </button>
      </div>

      <!-- Empty state -->
      <div
        v-if="filtered.length === 0"
        class="flex flex-col items-center justify-center py-24 text-center"
      >
        <div class="mb-4 p-4 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon
            name="heroicons:bell-slash"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          {{ filter === "unread" ? "All caught up" : "No notifications" }}
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500">
          {{
            filter === "unread"
              ? "You have no unread notifications."
              : "Nothing here yet."
          }}
        </p>
      </div>

      <!-- Notification list -->
      <div v-else class="flex flex-col gap-2">
        <div
          v-for="item in filtered"
          :key="item.id"
          class="group relative flex items-start gap-3 bg-white dark:bg-gray-800 rounded-xl p-4 border transition-all cursor-pointer"
          :class="
            item.read
              ? 'border-gray-100 dark:border-gray-700'
              : 'border-accent-100 dark:border-accent-900 hover:border-accent-200 dark:hover:border-accent-800'
          "
          @click="markRead(item.id)"
        >
          <!-- unread dot -->
          <span
            v-if="!item.read"
            class="absolute top-4 right-4 size-2 rounded-full bg-accent-500"
          />

          <!-- category icon -->
          <div
            class="shrink-0 size-8 rounded-lg flex items-center justify-center mt-0.5"
            :class="categoryConfig[item.category].bg"
          >
            <UIcon
              :name="categoryConfig[item.category].icon"
              class="size-4"
              :class="categoryConfig[item.category].color"
            />
          </div>

          <!-- content -->
          <div class="flex-1 min-w-0 pr-6">
            <div class="flex items-baseline gap-2">
              <p
                class="text-sm truncate"
                :class="
                  item.read
                    ? 'font-normal text-gray-600 dark:text-gray-400'
                    : 'font-medium text-gray-800 dark:text-gray-100'
                "
              >
                {{ item.title }}
              </p>
              <span class="text-[11px] text-gray-400 shrink-0">
                {{ relativeTime(item.time) }}
              </span>
            </div>
            <p
              class="text-xs text-gray-400 dark:text-gray-500 mt-0.5 leading-snug"
            >
              {{ item.message }}
            </p>
          </div>

          <!-- dismiss button (hover) -->
          <button
            class="absolute bottom-3.5 right-3.5 opacity-0 group-hover:opacity-100 transition-opacity text-gray-300 hover:text-gray-500 dark:hover:text-gray-300"
            aria-label="Dismiss"
            @click.stop="dismiss(item.id)"
          >
            <UIcon name="heroicons:x-mark" class="size-3.5" />
          </button>
        </div>
      </div>
    </template>

    <!-- Side panel -->
    <template #side_content>
      <!-- Summary -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Summary
      </h2>
      <div class="grid grid-cols-2 gap-2 mb-5">
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 text-center">
          <p class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
            {{ notifications.length }}
          </p>
          <p class="text-[11px] text-gray-400 mt-0.5">Total</p>
        </div>
        <div class="bg-accent-50 dark:bg-accent-950 rounded-lg p-3 text-center">
          <p
            class="text-2xl font-semibold text-accent-600 dark:text-accent-300"
          >
            {{ unreadCount }}
          </p>
          <p class="text-[11px] text-accent-400 mt-0.5">Unread</p>
        </div>
      </div>

      <USeparator class="mb-4" />

      <!-- Category filter -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Category
      </h2>
      <div class="flex flex-col gap-1 mb-5">
        <button
          class="flex items-center justify-between py-2 px-3 rounded-lg text-sm transition-colors w-full"
          :class="
            activeCategory === 'all'
              ? 'bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200'
              : 'text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
          "
          @click="activeCategory = 'all'"
        >
          <span>All</span>
          <span class="text-xs text-gray-400">{{ notifications.length }}</span>
        </button>

        <button
          v-for="cat in categoryCounts"
          :key="cat.key"
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm transition-colors w-full"
          :class="
            activeCategory === cat.key
              ? 'bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200'
              : 'text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
          "
          @click="activeCategory = cat.key"
        >
          <UIcon
            :name="categoryConfig[cat.key].icon"
            class="size-3.5 shrink-0"
            :class="categoryConfig[cat.key].color"
          />
          <span class="flex-1 text-left">{{
            categoryConfig[cat.key].label
          }}</span>
          <span class="text-xs text-gray-400">{{ cat.count }}</span>
        </button>
      </div>

      <USeparator class="mb-4" />

      <!-- Actions -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Actions
      </h2>
      <div class="flex flex-col gap-2">
        <button
          :disabled="unreadCount === 0"
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm w-full transition-colors"
          :class="
            unreadCount > 0
              ? 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800'
              : 'text-gray-300 dark:text-gray-600 cursor-not-allowed'
          "
          @click="markAllRead"
        >
          <UIcon name="heroicons:check-circle" class="size-4 shrink-0" />
          Mark all as read
        </button>
        <button
          :disabled="notifications.length === 0"
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm w-full transition-colors"
          :class="
            notifications.length > 0
              ? 'text-accent-500 hover:bg-accent-50 dark:hover:bg-accent-950'
              : 'text-gray-300 dark:text-gray-600 cursor-not-allowed'
          "
          @click="clearAll"
        >
          <UIcon name="heroicons:trash" class="size-4 shrink-0" />
          Clear all
        </button>
      </div>
    </template>
  </NuxtLayout>
</template>
