<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { useBookmarkStore } from "~/stores/bookmarks";
import { useTodoStore } from "~/stores/todo";
import { useUserPreferenceStore } from "~/stores/user-preference";
import { useReminderStore } from "~/stores/reminder";
import { useSnippetStore } from "~/stores/snippets";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useMutation } from 'villus';


const preflightGraphQL = `
mutation Preflight {
  preflight(name: "alex")
}
`;


const { data, execute } = useMutation(preflightGraphQL);

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();
const reminderStore = useReminderStore();
const snippetStore = useSnippetStore();

const { setSearch, clearSearch, searchQuery } = useAppSearch();

onMounted(async () => {
  await Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
    reminderStore.fetchReminders(),
    snippetStore.fetchSnippets(),
  ]);
  setSearch({ placeholder: "Search everything…" });
});

onUnmounted(() => clearSearch());

const searchResults = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return null;

  const notes = noteStore.notes
    .filter(
      (n) =>
        n.title?.toLowerCase().includes(q) ||
        n.content?.toLowerCase().includes(q),
    )
    .map((n) => ({
      type: "note" as const,
      identifier: n.identifier,
      title: n.title,
      sub: n.updatedAt,
      href: "/notes",
    }));

  const bookmarks = bookmarkStore.bookmarks
    .filter(
      (b) =>
        b.title?.toLowerCase().includes(q) ||
        b.url?.toLowerCase().includes(q) ||
        b.tag?.toLowerCase().includes(q),
    )
    .map((b) => ({
      type: "bookmark" as const,
      identifier: b.identifier,
      title: b.title,
      sub: b.url,
      href: "/bookmarks",
      url: b.url,
    }));

  const snippets = snippetStore.snippets
    .filter(
      (s) =>
        s.title?.toLowerCase().includes(q) ||
        s.language?.toLowerCase().includes(q) ||
        s.description?.toLowerCase().includes(q) ||
        s.code?.toLowerCase().includes(q),
    )
    .map((s) => ({
      type: "snippet" as const,
      identifier: s.identifier,
      title: s.title || "Untitled",
      sub: s.language || "",
      href: "/snippets",
    }));

  const todos = todoStore.todos
    .filter(
      (t) =>
        t.title?.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    )
    .map((t) => ({
      type: "todo" as const,
      identifier: t.identifier,
      title: t.title,
      sub: t.priority,
      href: "/todo",
    }));

  const reminders = reminderStore.reminders
    .filter(
      (r) =>
        r.title?.toLowerCase().includes(q) ||
        r.description?.toLowerCase().includes(q),
    )
    .map((r) => ({
      type: "reminder" as const,
      identifier: r.identifier,
      title: r.title,
      sub: r.remindAt,
      href: "/reminders",
    }));

  return { notes, bookmarks, snippets, todos, reminders };
});

const totalSearchResults = computed(() => {
  if (!searchResults.value) return 0;
  const r = searchResults.value;
  return (
    r.notes.length +
    r.bookmarks.length +
    r.snippets.length +
    r.todos.length +
    r.reminders.length
  );
});

const searchSections = computed(() => {
  if (!searchResults.value) return [];
  const iconMap = {
    note: {
      icon: "heroicons:document-text",
      color: "text-violet-400",
      bg: "bg-violet-50 dark:bg-violet-950/60",
    },
    bookmark: {
      icon: "heroicons:bookmark-solid",
      color: "text-accent-400",
      bg: "bg-accent-50 dark:bg-accent-950/60",
    },
    snippet: {
      icon: "heroicons:code-bracket",
      color: "text-blue-400",
      bg: "bg-blue-50 dark:bg-blue-950/60",
    },
    todo: {
      icon: "heroicons:check-circle",
      color: "text-emerald-400",
      bg: "bg-emerald-50 dark:bg-emerald-950/60",
    },
    reminder: {
      icon: "heroicons:clock",
      color: "text-rose-400",
      bg: "bg-rose-50 dark:bg-rose-950/60",
    },
  };
  return [
    { label: "Notes", items: searchResults.value.notes, ...iconMap.note },
    {
      label: "Bookmarks",
      items: searchResults.value.bookmarks,
      ...iconMap.bookmark,
    },
    {
      label: "Snippets",
      items: searchResults.value.snippets,
      ...iconMap.snippet,
    },
    { label: "Todos", items: searchResults.value.todos, ...iconMap.todo },
    {
      label: "Reminders",
      items: searchResults.value.reminders,
      ...iconMap.reminder,
    },
  ].filter((s) => s.items.length > 0);
});

// Live clock
const now = ref(new Date());
let clockTimer: ReturnType<typeof setInterval>;
onMounted(() => {
  clockTimer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(clockTimer));



const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);

const firstName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);

// Todos
const priorityOrder: Record<"high" | "medium" | "low", number> = {
  high: 0,
  medium: 1,
  low: 2,
};

const activeTodos = computed(() =>
  [...todoStore.todos]
    .filter((t) => !t.done)
    .sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority])
    .slice(0, 7),
);

const todoProgress = computed(() => {
  const total = todoStore.todos.length;
  if (total === 0) return 0;
  return Math.round((todoStore.completedTodos.length / total) * 100);
});

// SVG progress ring
const RING_R = 26;
const RING_C = computed(() => 2 * Math.PI * RING_R);
const ringOffset = computed(
  () => RING_C.value * (1 - todoProgress.value / 100),
);

const priorityDot: Record<string, string> = {
  high: "bg-red-400",
  medium: "bg-amber-400",
  low: "bg-emerald-400",
};

// Reminders
const upcomingReminders = computed(() =>
  [...reminderStore.upcomingReminders]
    .sort(
      (a, b) => new Date(a.remindAt).getTime() - new Date(b.remindAt).getTime(),
    )
    .slice(0, 3),
);

const nextReminder = computed(() => upcomingReminders.value[0] ?? null);

function timeUntil(iso: string) {
  const diff = new Date(iso).getTime() - Date.now();
  if (diff < 0) return "overdue";
  const mins = Math.floor(diff / 60_000);
  if (mins < 60) return `in ${mins}m`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `in ${hours}h`;
  return `in ${Math.floor(hours / 24)}d`;
}

function formatRemindAt(iso: string) {
  return new Date(iso).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

// Notes & bookmarks
const recentNotes = computed(() => noteStore.notes.slice(0, 4));
const recentBookmarks = computed(() => bookmarkStore.bookmarks.slice(0, 4));

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

// Stats pills
const statPills = computed(() => [
  {
    label: "Notes",
    value: noteStore.notes.length,
    icon: "heroicons:document-text-solid",
    color: "text-violet-500",
    href: "/notes",
  },
  {
    label: "Bookmarks",
    value: bookmarkStore.bookmarks.length,
    icon: "heroicons:bookmark-solid",
    color: "text-accent-500",
    href: "/bookmarks",
  },
  {
    label: "Active todos",
    value: todoStore.activeTodos.length,
    icon: "heroicons:check-circle-solid",
    color: "text-emerald-500",
    href: "/todo",
  },
  {
    label: "Reminders",
    value: upcomingReminders.value.length,
    icon: "heroicons:clock-solid",
    color: "text-rose-500",
    href: "/reminders",
  },
]);

// Quick actions
const quickActions = [
  {
    label: "New note",
    icon: "heroicons:document-plus",
    href: "/notes/create-notes",
    color:
      "text-violet-600 dark:text-violet-400 bg-violet-50 dark:bg-violet-950/60 hover:bg-violet-100 dark:hover:bg-violet-900/60 border-violet-100 dark:border-violet-900",
  },
  {
    label: "Add bookmark",
    icon: "heroicons:bookmark-plus",
    href: "/bookmarks",
    color:
      "text-accent-600 dark:text-accent-400 bg-accent-50 dark:bg-accent-950/60 hover:bg-accent-100 dark:hover:bg-accent-900/60 border-accent-100 dark:border-accent-900",
  },
  {
    label: "New todo",
    icon: "heroicons:plus-circle",
    href: "/todo/create-todo",
    color:
      "text-emerald-600 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/60 hover:bg-emerald-100 dark:hover:bg-emerald-900/60 border-emerald-100 dark:border-emerald-900",
  },
  {
    label: "New snippet",
    icon: "heroicons:code-bracket-square",
    href: "/snippets/create-snippets",
    color:
      "text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-950/60 hover:bg-blue-100 dark:hover:bg-blue-900/60 border-blue-100 dark:border-blue-900",
  },
];
</script>

<template>
  <NuxtLayout name="default">
    <!-- ── Hero banner ─────────────────────────────────────────────── -->
    <template #page_title>
      <div
        class="relative -mx-6 -mt-6 px-6 pt-7 pb-6 overflow-hidden bg-linear-to-br from-accent-500/10 via-violet-400/5 to-transparent dark:from-accent-500/12 dark:via-violet-500/6 dark:to-transparent border-b border-gray-100 dark:border-gray-800"
      >
      {{ data }}
      <button @click="execute">Test GraphQL</button>
        <!-- Soft blobs -->
        <div
          class="pointer-events-none absolute -top-10 right-0 size-52 rounded-full bg-accent-300/20 dark:bg-accent-500/10 blur-3xl"
        />
        <div
          class="pointer-events-none absolute bottom-0 left-1/2 size-36 rounded-full bg-violet-300/15 dark:bg-violet-500/8 blur-2xl translate-y-1/2"
        />

        <div class="relative flex items-end justify-between gap-4">
          <div>
            <p
              class="text-xs font-semibold tracking-widest uppercase text-gray-400 dark:text-gray-500 mb-1.5"
            >
              {{ today }}
            </p>
            <h1
              class="text-3xl font-bold tracking-tight text-gray-900 dark:text-white"
            >
              {{ greeting }}, {{ firstName }}
            </h1>
            <p class="mt-1.5 text-sm text-gray-500 dark:text-gray-400">
              <template v-if="todoStore.activeTodos.length > 0">
                <strong class="text-gray-700 dark:text-gray-300">{{
                  todoStore.activeTodos.length
                }}</strong>
                active
                {{ todoStore.activeTodos.length === 1 ? "todo" : "todos" }}
                today<span v-if="nextReminder"
                  >, next reminder
                  <strong class="text-rose-500">{{
                    timeUntil(nextReminder.remindAt)
                  }}</strong></span
                >.
              </template>
              <template v-else>
                You're all caught up
                <span v-if="nextReminder"
                  >— next reminder
                  <strong class="text-rose-500">{{
                    timeUntil(nextReminder.remindAt)
                  }}</strong></span
                >.
              </template>
            </p>
          </div>

          <!-- Live clock -->
          <!-- <p
            class="shrink-0 text-5xl font-thin tabular-nums leading-none text-gray-200 dark:text-gray-700 select-none"
          >
            {{ currentTime }}
          </p> -->
        </div>

        <!-- Stat pills -->
        <div class="relative flex flex-wrap items-center gap-2 mt-5">
          <NuxtLink
            v-for="s in statPills"
            :key="s.label"
            :to="s.href"
            class="flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium bg-white/70 dark:bg-gray-900/60 border border-gray-200/80 dark:border-gray-700/60 backdrop-blur-sm hover:border-accent-300 dark:hover:border-accent-700 transition-colors"
          >
            <UIcon :name="s.icon" class="size-3.5 shrink-0" :class="s.color" />
            <span class="text-gray-800 dark:text-gray-200 tabular-nums">{{
              s.value
            }}</span>
            <span class="text-gray-400 dark:text-gray-500">{{ s.label }}</span>
          </NuxtLink>
        </div>
      </div>
    </template>

    <!-- ── Main content ────────────────────────────────────────────── -->
    <template #main_content>
      <!-- Search results -->
      <template v-if="searchQuery.trim()">
        <div
          v-if="totalSearchResults === 0"
          class="flex flex-col items-center justify-center py-20 text-center"
        >
          <UIcon
            name="heroicons:magnifying-glass"
            class="size-10 text-gray-300 dark:text-gray-600 mb-3"
          />
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">
            No results for "{{ searchQuery }}"
          </p>
        </div>
        <div v-else class="flex flex-col gap-4">
          <p class="text-xs text-gray-400">
            {{ totalSearchResults }} result{{
              totalSearchResults === 1 ? "" : "s"
            }}
            for "{{ searchQuery }}"
          </p>
          <div
            v-for="section in searchSections"
            :key="section.label"
            class="bg-white dark:bg-gray-900 rounded-2xl border border-gray-200 dark:border-gray-700/60 overflow-hidden"
          >
            <div
              class="flex items-center gap-2 px-4 py-3 border-b border-gray-100 dark:border-gray-700/50"
            >
              <UIcon
                :name="section.icon"
                class="size-4 shrink-0"
                :class="section.color"
              />
              <h2
                class="text-sm font-semibold text-gray-700 dark:text-gray-300"
              >
                {{ section.label }}
              </h2>
              <span class="ml-auto text-xs text-gray-400">{{
                section.items.length
              }}</span>
            </div>
            <div class="divide-y divide-gray-100 dark:divide-gray-700/40">
              <NuxtLink
                v-for="item in section.items"
                :key="item.identifier"
                :to="item.href"
                class="flex items-center gap-3 px-4 py-3 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors"
              >
                <div
                  class="size-7 rounded-lg flex items-center justify-center shrink-0"
                  :class="section.bg"
                >
                  <UIcon
                    :name="section.icon"
                    class="size-3.5"
                    :class="section.color"
                  />
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-sm text-gray-800 dark:text-gray-200 truncate">
                    {{ item.title }}
                  </p>
                  <p
                    v-if="item.sub"
                    class="text-xs text-gray-400 truncate mt-0.5"
                  >
                    {{ item.sub }}
                  </p>
                </div>
              </NuxtLink>
            </div>
          </div>
        </div>
      </template>

      <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <!-- Todos: 2-col card with SVG ring -->
        <div
          class="lg:col-span-2 bg-white dark:bg-gray-900 rounded-2xl border border-gray-200 dark:border-gray-700/60 overflow-hidden flex flex-col"
        >
          <!-- Card header -->
          <div
            class="flex items-center gap-3 px-5 py-4 border-b border-gray-100 dark:border-gray-700/50"
          >
            <!-- Progress ring -->
            <div class="relative size-15 shrink-0">
              <svg class="size-15 -rotate-90" viewBox="0 0 68 68">
                <circle
                  cx="34"
                  cy="34"
                  :r="RING_R"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="5"
                  class="text-gray-100 dark:text-gray-800"
                />
                <circle
                  cx="34"
                  cy="34"
                  :r="RING_R"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="5"
                  stroke-linecap="round"
                  class="text-accent-500 transition-all duration-700"
                  :stroke-dasharray="RING_C"
                  :stroke-dashoffset="ringOffset"
                />
              </svg>
              <span
                class="absolute inset-0 flex items-center justify-center text-xs font-bold text-gray-700 dark:text-gray-300"
              >
                {{ todoProgress }}%
              </span>
            </div>

            <div class="flex-1 min-w-0">
              <h2
                class="text-sm font-semibold text-gray-800 dark:text-gray-200"
              >
                Active Todos
              </h2>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ todoStore.completedTodos.length }} of
                {{ todoStore.todos.length }} complete
              </p>
            </div>

            <NuxtLink
              to="/todo"
              class="shrink-0 text-xs font-medium text-accent-500 hover:text-accent-600 transition-colors"
            >
              See all →
            </NuxtLink>
          </div>

          <!-- Todo list -->
          <div class="flex-1">
            <div
              v-if="todoStore.loading"
              class="flex items-center gap-2 px-5 py-6 text-gray-400 text-sm"
            >
              <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
              Loading…
            </div>

            <div
              v-else-if="activeTodos.length === 0"
              class="flex flex-col items-center justify-center py-12 text-center"
            >
              <UIcon
                name="heroicons:check-badge"
                class="size-10 text-emerald-400 mb-2"
              />
              <p class="text-sm font-medium text-gray-500 dark:text-gray-400">
                All caught up!
              </p>
              <NuxtLink
                to="/todo/create-todo"
                class="text-xs text-accent-500 hover:underline mt-1"
                >Add a new todo</NuxtLink
              >
            </div>

            <div
              v-else
              class="divide-y divide-gray-100 dark:divide-gray-700/40"
            >
              <div
                v-for="todo in activeTodos"
                :key="todo.identifier"
                class="group flex items-center gap-3 px-5 py-3 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors"
              >
                <span
                  class="size-1.5 rounded-full shrink-0"
                  :class="priorityDot[todo.priority]"
                />
                <div class="flex-1 min-w-0">
                  <p class="text-sm text-gray-800 dark:text-gray-200 truncate">
                    {{ todo.title }}
                  </p>
                  <p v-if="todo.dueDate" class="text-xs text-gray-400 mt-0.5">
                    Due {{ formatDate(todo.dueDate) }}
                  </p>
                </div>
                <span
                  class="text-xs text-gray-400 dark:text-gray-500 capitalize shrink-0"
                >
                  {{ todo.priority }}
                </span>
                <button
                  class="opacity-0 group-hover:opacity-100 transition-opacity text-gray-300 hover:text-red-400 dark:text-gray-700 dark:hover:text-red-400 shrink-0"
                  @click="todoStore.deleteTodo(todo.identifier)"
                >
                  <UIcon name="heroicons:trash" class="size-3.5" />
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Right column -->
        <div class="flex flex-col gap-4">
          <!-- Next reminder -->
          <div
            v-if="nextReminder"
            class="bg-linear-to-br from-rose-50 to-orange-50/60 dark:from-rose-950/40 dark:to-orange-950/20 rounded-2xl border border-rose-100 dark:border-rose-900/40 p-5"
          >
            <div class="flex items-start justify-between mb-3">
              <div class="flex items-center gap-1.5 text-rose-500">
                <UIcon
                  :name="
                    nextReminder.recurring
                      ? 'heroicons:arrow-path'
                      : 'heroicons:bell'
                  "
                  class="size-3.5"
                />
                <span class="text-xs font-semibold uppercase tracking-wide"
                  >Next reminder</span
                >
              </div>
              <span
                class="text-xs font-bold px-2 py-0.5 rounded-full bg-rose-100 dark:bg-rose-900/50 text-rose-600 dark:text-rose-400 shrink-0"
              >
                {{ timeUntil(nextReminder.remindAt) }}
              </span>
            </div>
            <p class="text-sm font-semibold text-gray-800 dark:text-gray-200">
              {{ nextReminder.title }}
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              {{ formatRemindAt(nextReminder.remindAt) }}
            </p>
            <span
              v-if="nextReminder.recurring"
              class="inline-flex items-center gap-1 mt-2.5 text-xs px-2 py-0.5 rounded-full bg-violet-100 dark:bg-violet-900/40 text-violet-600 dark:text-violet-400"
            >
              <UIcon name="heroicons:arrow-path" class="size-3" />
              Recurring
            </span>
          </div>

          <div
            v-else
            class="rounded-2xl border border-dashed border-gray-200 dark:border-gray-700 p-5 flex flex-col items-center justify-center text-center bg-gray-50/50 dark:bg-gray-900/30"
          >
            <UIcon
              name="heroicons:bell-slash"
              class="size-7 text-gray-300 dark:text-gray-600 mb-2"
            />
            <p class="text-xs text-gray-400">No upcoming reminders</p>
            <NuxtLink
              to="/reminders/create-reminder"
              class="text-xs text-accent-500 hover:underline mt-1"
              >Set one up</NuxtLink
            >
          </div>

          <!-- Recent notes -->
          <div
            class="flex-1 bg-white dark:bg-gray-900 rounded-2xl border border-gray-200 dark:border-gray-700/60 overflow-hidden flex flex-col"
          >
            <div
              class="flex items-center justify-between px-4 py-3.5 border-b border-gray-100 dark:border-gray-700/50"
            >
              <h2
                class="text-sm font-semibold text-gray-700 dark:text-gray-300 flex items-center gap-1.5"
              >
                <UIcon
                  name="heroicons:document-text"
                  class="size-4 text-violet-400"
                />
                Recent notes
              </h2>
              <NuxtLink
                to="/notes"
                class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
                >View all →</NuxtLink
              >
            </div>

            <div
              v-if="noteStore.loading"
              class="flex items-center gap-2 p-4 text-gray-400 text-xs"
            >
              <UIcon
                name="heroicons:arrow-path"
                class="size-3.5 animate-spin"
              />
              Loading…
            </div>
            <div
              v-else-if="recentNotes.length === 0"
              class="p-5 text-center flex-1 flex flex-col items-center justify-center"
            >
              <p class="text-xs text-gray-400">No notes yet</p>
              <NuxtLink
                to="/notes/create-notes"
                class="text-xs text-accent-500 hover:underline mt-1"
                >Create one</NuxtLink
              >
            </div>
            <div
              v-else
              class="divide-y divide-gray-100 dark:divide-gray-700/40"
            >
              <NuxtLink
                v-for="note in recentNotes"
                :key="note.identifier"
                to="/notes"
                class="group flex items-start gap-2.5 px-4 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors"
              >
                <div
                  class="size-6 rounded-md bg-violet-50 dark:bg-violet-950/60 flex items-center justify-center shrink-0 mt-0.5"
                >
                  <UIcon
                    name="heroicons:document-text"
                    class="size-3 text-violet-400"
                  />
                </div>
                <div class="flex-1 min-w-0">
                  <p
                    class="text-xs font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-violet-600 dark:group-hover:text-violet-400 transition-colors"
                  >
                    {{ note.title }}
                  </p>
                  <p class="text-xs text-gray-400 truncate mt-0.5">
                    {{ stripHtml(note.content) || "No content" }}
                  </p>
                </div>
                <span class="text-xs text-gray-300 dark:text-gray-600 shrink-0">
                  {{ formatDate(note.updatedAt) }}
                </span>
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Bookmarks: full-width row -->
        <div
          class="lg:col-span-3 bg-white dark:bg-gray-900 rounded-2xl border border-gray-200 dark:border-gray-700/60 overflow-hidden"
        >
          <div
            class="flex items-center justify-between px-5 py-3.5 border-b border-gray-100 dark:border-gray-700/50"
          >
            <h2
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 flex items-center gap-1.5"
            >
              <UIcon name="heroicons:bookmark" class="size-4 text-accent-400" />
              Recent bookmarks
            </h2>
            <NuxtLink
              to="/bookmarks"
              class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
              >View all →</NuxtLink
            >
          </div>

          <div
            v-if="bookmarkStore.loading"
            class="flex items-center gap-2 p-5 text-gray-400 text-sm"
          >
            <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
            Loading…
          </div>
          <div v-else-if="recentBookmarks.length === 0" class="p-6 text-center">
            <p class="text-sm text-gray-400">No bookmarks saved yet.</p>
            <NuxtLink
              to="/bookmarks"
              class="text-xs text-accent-500 hover:underline mt-1 block"
              >Add your first bookmark</NuxtLink
            >
          </div>
          <div
            v-else
            class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 divide-y sm:divide-y-0 sm:divide-x divide-gray-100 dark:divide-gray-700/40"
          >
            <div
              v-for="bm in recentBookmarks"
              :key="bm.identifier"
              class="group flex items-start gap-3 p-4 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors cursor-pointer"
              @click="openUrl(bm.url)"
            >
              <div
                class="size-7 rounded-lg bg-accent-50 dark:bg-accent-950/60 flex items-center justify-center shrink-0 mt-0.5"
              >
                <UIcon
                  name="heroicons:bookmark-solid"
                  class="size-3.5 text-accent-400"
                />
              </div>
              <div class="flex-1 min-w-0">
                <p
                  class="text-xs font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-accent-600 dark:group-hover:text-accent-400 transition-colors"
                >
                  {{ bm.title }}
                </p>
                <p class="text-xs text-gray-400 truncate mt-0.5">
                  {{ bm.url }}
                </p>
                <span
                  v-if="bm.tag"
                  class="inline-block mt-1.5 text-xs px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 capitalize"
                  >{{ bm.tag }}</span
                >
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- ── Side panel ──────────────────────────────────────────────── -->
    <template #side_content>
      <!-- Quick actions -->
      <section class="mb-5">
        <h2
          class="text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 mb-3"
        >
          Quick actions
        </h2>
        <div class="flex flex-col gap-1.5">
          <NuxtLink
            v-for="action in quickActions"
            :key="action.label"
            :to="action.href"
            class="flex items-center gap-2.5 px-3 py-2.5 rounded-xl border text-sm font-medium transition-colors"
            :class="action.color"
          >
            <UIcon :name="action.icon" class="size-4 shrink-0" />
            {{ action.label }}
          </NuxtLink>
        </div>
      </section>

      <USeparator class="my-4" />

      <!-- Reminders list -->
      <section v-if="upcomingReminders.length > 0">
        <div class="flex items-center justify-between mb-3">
          <h2
            class="text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500"
          >
            Reminders
          </h2>
          <NuxtLink
            to="/reminders"
            class="text-xs text-accent-500 hover:text-accent-600 transition-colors"
            >All →</NuxtLink
          >
        </div>
        <div class="flex flex-col gap-1.5">
          <NuxtLink
            v-for="reminder in upcomingReminders"
            :key="reminder.identifier"
            to="/reminders"
            class="group flex items-start gap-2.5 px-3 py-2.5 rounded-xl bg-gray-50 dark:bg-gray-800/60 hover:bg-rose-50 dark:hover:bg-rose-950/30 border border-transparent hover:border-rose-100 dark:hover:border-rose-900/40 transition-all"
          >
            <UIcon
              :name="
                reminder.recurring ? 'heroicons:arrow-path' : 'heroicons:clock'
              "
              class="size-3.5 text-rose-400 shrink-0 mt-0.5"
            />
            <div class="flex-1 min-w-0">
              <p
                class="text-xs font-medium text-gray-700 dark:text-gray-300 truncate group-hover:text-rose-600 dark:group-hover:text-rose-400 transition-colors"
              >
                {{ reminder.title }}
              </p>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ formatRemindAt(reminder.remindAt) }}
              </p>
            </div>
            <span
              class="text-xs font-semibold text-rose-400 dark:text-rose-500 shrink-0 mt-0.5"
            >
              {{ timeUntil(reminder.remindAt) }}
            </span>
          </NuxtLink>
        </div>
      </section>

      <section v-else>
        <h2
          class="text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 mb-3"
        >
          Reminders
        </h2>
        <div class="flex flex-col items-center py-6 text-center">
          <UIcon
            name="heroicons:bell-slash"
            class="size-7 text-gray-300 dark:text-gray-700 mb-2"
          />
          <p class="text-xs text-gray-400">No upcoming reminders</p>
          <NuxtLink
            to="/reminders/create-reminder"
            class="text-xs text-accent-500 hover:underline mt-1"
            >Set one up</NuxtLink
          >
        </div>
      </section>
    </template>
  </NuxtLayout>
</template>
