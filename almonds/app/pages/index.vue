<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { useBookmarkStore } from "~/stores/bookmarks";
import { useTodoStore } from "~/stores/todo";
import { useUserPreferenceStore } from "~/stores/user-preference";
import { useReminderStore } from "~/stores/reminder";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();
const reminderStore = useReminderStore();

onMounted(async () => {
  await Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
    reminderStore.fetchReminders(),
  ]);
});

// Greeting
const greeting = computed(() => {
  const h = new Date().getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  new Date().toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);

// Stats
const stats = computed(() => [
  {
    label: "Notes",
    value: noteStore.notes.length,
    icon: "heroicons:document-text-solid",
    color: "text-violet-500",
    bg: "bg-violet-50 dark:bg-violet-950",
    href: "/notes",
  },
  {
    label: "Bookmarks",
    value: bookmarkStore.bookmarks.length,
    icon: "heroicons:bookmark-solid",
    color: "text-accent-500",
    bg: "bg-accent-50 dark:bg-accent-950",
    href: "/bookmarks",
  },
  {
    label: "Active todos",
    value: todoStore.activeTodos.length,
    icon: "heroicons:check-circle-solid",
    color: "text-emerald-500",
    bg: "bg-emerald-50 dark:bg-emerald-950",
    href: "/todo",
  },
  {
    label: "Upcoming reminders",
    value: upcomingReminders.value.length,
    icon: "heroicons:clock-solid",
    color: "text-rose-500",
    bg: "bg-rose-50 dark:bg-rose-950",
    href: "/reminders",
  },
]);

// Next 3 upcoming reminders sorted by soonest first
const upcomingReminders = computed(() =>
  [...reminderStore.upcomingReminders]
    .sort(
      (a, b) => new Date(a.remindAt).getTime() - new Date(b.remindAt).getTime(),
    )
    .slice(0, 3),
);

function formatRemindAt(iso: string) {
  return new Date(iso).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

// Recent notes (latest 3)
const recentNotes = computed(() => noteStore.notes.slice(0, 3));

// Recent bookmarks (latest 3)
const recentBookmarks = computed(() => bookmarkStore.bookmarks.slice(0, 3));

// Todo filter / sort
const todoFilter = ref<"all" | "active" | "done">("active");
const todoSort = ref<"priority" | "date">("priority");

const priorityOrder: Record<"high" | "medium" | "low", number> = {
  high: 0,
  medium: 1,
  low: 2,
};

const filteredSortedTodos = computed(() => {
  let list = todoStore.todos;
  if (todoFilter.value === "active") list = list.filter((t) => !t.done);
  else if (todoFilter.value === "done") list = list.filter((t) => t.done);
  return [...list]
    .sort((a, b) => {
      if (todoSort.value === "priority") {
        return priorityOrder[a.priority] - priorityOrder[b.priority];
      }
      if (!a.dueDate && !b.dueDate) return 0;
      if (!a.dueDate) return 1;
      if (!b.dueDate) return -1;
      return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
    })
    .slice(0, 5);
});

// Progress
const todoProgress = computed(() => {
  const total = todoStore.todos.length;
  if (total === 0) return 0;
  return Math.round((todoStore.completedTodos.length / total) * 100);
});

const priorityColor: Record<string, string> = {
  high: "text-red-500 bg-red-50 dark:bg-red-950",
  medium: "text-amber-500 bg-amber-50 dark:bg-amber-950",
  low: "text-emerald-500 bg-emerald-50 dark:bg-emerald-950",
};

function stripHtml(html: string) {
  return html
    .replace(/<[^>]*>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}

const firstName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);
const quickActions = [
  {
    label: "New note",
    icon: "heroicons:document-plus",
    href: "/notes/create-notes",
  },
  {
    label: "Add bookmark",
    icon: "heroicons:bookmark-slash",
    href: "/bookmarks",
  },
  {
    label: "New todo",
    icon: "heroicons:plus-circle",
    href: "/todo/create-todo",
  },
  {
    label: "New snippet",
    icon: "heroicons:code-bracket-square",
    href: "/snippets/create-snippets",
  },
];
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <div>
        <p class="text-sm text-gray-400 dark:text-gray-500 mb-0.5">
          {{ today }}
        </p>
        <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
          {{ greeting }}, {{ firstName }} 👋
        </h1>
      </div>
    </template>

    <template #main_content>
      <!-- Stats row -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
        <NuxtLink
          v-for="stat in stats"
          :key="stat.label"
          :to="stat.href"
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow flex items-center gap-3"
        >
          <div
            class="size-9 rounded-lg flex items-center justify-center shrink-0"
            :class="stat.bg"
          >
            <UIcon :name="stat.icon" class="size-4.5" :class="stat.color" />
          </div>
          <div>
            <p
              class="text-xl font-bold text-gray-800 dark:text-gray-100 leading-none"
            >
              {{ stat.value }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ stat.label }}</p>
          </div>
        </NuxtLink>
      </div>

      <!-- Recent notes -->
      <section class="mb-8">
        <div class="flex items-center justify-between mb-3">
          <h2
            class="text-sm font-medium text-gray-500 dark:text-gray-400 flex items-center gap-2"
          >
            <UIcon name="heroicons:document-text" class="size-4" />
            Recent notes
          </h2>
          <NuxtLink
            to="/notes"
            class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
          >
            View all
          </NuxtLink>
        </div>

        <div
          v-if="noteStore.loading"
          class="flex items-center gap-2 py-6 text-gray-400 text-sm"
        >
          <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
          Loading…
        </div>

        <div
          v-else-if="recentNotes.length === 0"
          class="bg-white dark:bg-gray-800 rounded-xl border border-dashed border-gray-200 dark:border-gray-700 p-6 text-center"
        >
          <p class="text-sm text-gray-400">No notes yet.</p>
          <NuxtLink
            to="/notes/create-notes"
            class="text-xs text-accent-500 hover:underline mt-1 block"
          >
            Create your first note
          </NuxtLink>
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <NuxtLink
            v-for="note in recentNotes"
            :key="note.identifier"
            :to="`/notes`"
            class="group bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm hover:border-accent-200 dark:hover:border-accent-800 transition-all"
          >
            <h3
              class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate mb-1 group-hover:text-accent-600 dark:group-hover:text-accent-400 transition-colors"
            >
              {{ note.title }}
            </h3>
            <p class="text-xs text-gray-400 line-clamp-2 mb-2">
              {{ stripHtml(note.content) || "No content" }}
            </p>
            <p class="text-xs text-gray-300 dark:text-gray-600">
              {{ formatDate(note.updatedAt) }}
            </p>
          </NuxtLink>
        </div>
      </section>

      <!-- Upcoming reminders -->
      <section class="mb-8">
        <div class="flex items-center justify-between mb-3">
          <h2
            class="text-sm font-medium text-gray-500 dark:text-gray-400 flex items-center gap-2"
          >
            <UIcon name="heroicons:clock" class="size-4" />
            Upcoming reminders
          </h2>
          <NuxtLink
            to="/reminders"
            class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
          >
            View all
          </NuxtLink>
        </div>

        <div
          v-if="upcomingReminders.length === 0"
          class="bg-white dark:bg-gray-800 rounded-xl border border-dashed border-gray-200 dark:border-gray-700 p-6 text-center"
        >
          <p class="text-sm text-gray-400">No upcoming reminders.</p>
          <NuxtLink
            to="/reminders/create-reminder"
            class="text-xs text-accent-500 hover:underline mt-1 block"
          >
            Create a reminder
          </NuxtLink>
        </div>

        <div v-else class="flex flex-col gap-2">
          <NuxtLink
            v-for="reminder in upcomingReminders"
            :key="reminder.identifier"
            to="/reminders"
            class="group bg-white dark:bg-gray-800 rounded-xl px-4 py-3 border border-gray-100 dark:border-gray-700 hover:shadow-sm hover:border-rose-200 dark:hover:border-rose-800 transition-all flex items-center gap-3"
          >
            <div class="p-1.5 rounded-md bg-rose-50 dark:bg-rose-950 shrink-0">
              <UIcon
                :name="
                  reminder.recurring
                    ? 'heroicons:arrow-path'
                    : 'heroicons:clock'
                "
                class="size-4 text-rose-400"
              />
            </div>
            <div class="flex-1 min-w-0">
              <p
                class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-rose-600 dark:group-hover:text-rose-400 transition-colors"
              >
                {{ reminder.title }}
              </p>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ formatRemindAt(reminder.remindAt) }}
              </p>
            </div>
            <span
              v-if="reminder.recurring"
              class="text-xs px-2 py-0.5 rounded-full bg-violet-50 dark:bg-violet-950 text-violet-500 dark:text-violet-400 shrink-0"
            >
              Recurring
            </span>
          </NuxtLink>
        </div>
      </section>

      <!-- Recent bookmarks -->
      <section>
        <div class="flex items-center justify-between mb-3">
          <h2
            class="text-sm font-medium text-gray-500 dark:text-gray-400 flex items-center gap-2"
          >
            <UIcon name="heroicons:bookmark" class="size-4" />
            Recent bookmarks
          </h2>
          <NuxtLink
            to="/bookmarks"
            class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
          >
            View all
          </NuxtLink>
        </div>

        <div
          v-if="bookmarkStore.loading"
          class="flex items-center gap-2 py-6 text-gray-400 text-sm"
        >
          <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
          Loading…
        </div>

        <div
          v-else-if="recentBookmarks.length === 0"
          class="bg-white dark:bg-gray-800 rounded-xl border border-dashed border-gray-200 dark:border-gray-700 p-6 text-center"
        >
          <p class="text-sm text-gray-400">No bookmarks saved yet.</p>
          <NuxtLink
            to="/bookmarks"
            class="text-xs text-accent-500 hover:underline mt-1 block"
          >
            Add your first bookmark
          </NuxtLink>
        </div>

        <div v-else class="flex flex-col gap-2">
          <NuxtLink
            v-for="bm in recentBookmarks"
            :key="bm.identifier"
            :to="bm.url"
            class="group bg-white dark:bg-gray-800 rounded-xl px-4 py-3 border border-gray-100 dark:border-gray-700 hover:shadow-sm hover:border-accent-200 dark:hover:border-accent-800 transition-all flex items-center gap-3"
          >
            <UIcon
              name="heroicons:bookmark-solid"
              class="size-4 text-accent-400 shrink-0"
            />
            <div class="flex-1 min-w-0">
              <p
                class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-accent-600 dark:group-hover:text-accent-400 transition-colors"
              >
                {{ bm.title }}
              </p>
              <p class="text-xs text-gray-400 truncate">{{ bm.url }}</p>
            </div>
            <span
              class="text-xs px-2 py-0.5 rounded-full bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 capitalize shrink-0"
            >
              {{ bm.tag }}
            </span>
          </NuxtLink>
        </div>
      </section>
    </template>

    <template #side_content>
      <!-- Quick actions -->
      <section class="mb-6">
        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Quick actions
        </h2>
        <div class="grid grid-cols-2 gap-2">
          <NuxtLink
            v-for="action in quickActions"
            :key="action.label"
            :to="action.href"
            class="flex flex-col items-center gap-1.5 py-3 px-2 rounded-xl bg-gray-50 dark:bg-gray-800 hover:bg-accent-50 dark:hover:bg-accent-950 hover:text-accent-600 dark:hover:text-accent-400 text-gray-500 dark:text-gray-400 transition-colors border border-transparent hover:border-accent-200 dark:hover:border-accent-800"
          >
            <UIcon :name="action.icon" class="size-5" />
            <span class="text-xs font-medium text-center leading-tight">{{
              action.label
            }}</span>
          </NuxtLink>
        </div>
      </section>

      <USeparator class="my-4" />

      <!-- Active todos -->
      <section class="mb-4">
        <div class="flex items-center justify-between mb-3">
          <h2
            class="text-sm font-medium text-gray-500 dark:text-gray-400 flex items-center gap-2"
          >
            <UIcon name="heroicons:clipboard-document-check" class="size-4" />
            Todos
          </h2>
          <NuxtLink
            to="/todo"
            class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
          >
            See all
          </NuxtLink>
        </div>

        <!-- Filter / Sort / Delete completed controls -->
        <div class="flex items-center gap-1.5 mb-3 flex-wrap">
          <div
            class="flex rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700 text-xs"
          >
            <button
              v-for="f in ['all', 'active', 'done'] as const"
              :key="f"
              class="px-2 py-0.5 capitalize transition-colors"
              :class="
                todoFilter === f
                  ? 'bg-accent-500 text-white'
                  : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'
              "
              @click="todoFilter = f"
            >
              {{ f }}
            </button>
          </div>
          <div
            class="flex rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700 text-xs"
          >
            <button
              class="px-2 py-0.5 transition-colors"
              :class="
                todoSort === 'priority'
                  ? 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
                  : 'text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
              "
              @click="todoSort = 'priority'"
            >
              Priority
            </button>
            <button
              class="px-2 py-0.5 transition-colors"
              :class="
                todoSort === 'date'
                  ? 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
                  : 'text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
              "
              @click="todoSort = 'date'"
            >
              Date
            </button>
          </div>
        </div>

        <!-- Progress bar -->
        <div v-if="todoStore.todos.length > 0" class="mb-3">
          <div class="flex justify-between text-xs text-gray-400 mb-1">
            <span
              >{{ todoStore.completedTodos.length }} /
              {{ todoStore.todos.length }} complete</span
            >
            <span>{{ todoProgress }}%</span>
          </div>
          <div
            class="h-1.5 rounded-full bg-gray-100 dark:bg-gray-800 overflow-hidden"
          >
            <div
              class="h-full rounded-full bg-accent-500 transition-all duration-500"
              :style="{ width: `${todoProgress}%` }"
            />
          </div>
        </div>

        <div
          v-if="todoStore.loading"
          class="flex items-center gap-2 py-4 text-gray-400 text-xs"
        >
          <UIcon name="heroicons:arrow-path" class="size-3.5 animate-spin" />
          Loading…
        </div>

        <div
          v-else-if="filteredSortedTodos.length === 0"
          class="py-4 text-center"
        >
          <UIcon
            name="heroicons:check-badge"
            class="size-8 text-emerald-400 mx-auto mb-1"
          />
          <p class="text-xs text-gray-400">
            {{ todoFilter === "active" ? "All caught up!" : "No todos found." }}
          </p>
        </div>

        <div v-else class="flex flex-col gap-1.5">
          <div
            v-for="todo in filteredSortedTodos"
            :key="todo.identifier"
            class="group flex items-start gap-2.5 py-1.5"
          >
            <UIcon
              name="heroicons:circle-stack"
              class="size-4 text-gray-300 dark:text-gray-600 shrink-0 mt-0.5"
            />
            <div class="flex-1 min-w-0">
              <p class="text-xs text-gray-700 dark:text-gray-300 truncate">
                {{ todo.title }}
              </p>
              <p v-if="todo.dueDate" class="text-xs text-gray-400">
                Due {{ formatDate(todo.dueDate) }}
              </p>
            </div>
            <span
              class="text-xs px-1.5 py-0.5 rounded-md font-medium capitalize shrink-0"
              :class="priorityColor[todo.priority]"
            >
              {{ todo.priority }}
            </span>
            <button
              class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0 mt-0.5 text-gray-300 hover:text-red-500 dark:text-gray-600 dark:hover:text-red-400"
              @click="todoStore.deleteTodo(todo.identifier)"
            >
              <UIcon name="heroicons:trash" class="size-3.5" />
            </button>
          </div>
        </div>
      </section>
    </template>
  </NuxtLayout>
</template>
