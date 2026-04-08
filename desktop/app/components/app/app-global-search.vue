<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { onClickOutside } from "@vueuse/core";

const emit = defineEmits<{ close: [] }>();

const router = useRouter();
const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const snippetStore = useSnippetStore();
const todoStore = useTodoStore();
const reminderStore = useReminderStore();
const { searchQuery } = useAppSearch();

const dropdownRef = ref<HTMLElement | null>(null);
const activeIndex = ref(-1);

// Lazily fetch stores that haven't loaded yet
onMounted(async () => {
  await Promise.all(
    [
      noteStore.notes.length === 0 ? noteStore.fetchNotes() : null,
      bookmarkStore.bookmarks.length === 0
        ? bookmarkStore.fetchBookmarks()
        : null,
      snippetStore.snippets.length === 0 ? snippetStore.fetchSnippets() : null,
      todoStore.todos.length === 0 ? todoStore.fetchTodos() : null,
      reminderStore.reminders.length === 0
        ? reminderStore.fetchReminders()
        : null,
    ].filter(Boolean),
  );
});

onClickOutside(dropdownRef, () => emit("close"));

const typeConfig = {
  note: {
    icon: "heroicons:document-text",
    color: "text-violet-500 dark:text-violet-400",
    bg: "bg-violet-50 dark:bg-violet-950/60",
    label: "Notes",
  },
  bookmark: {
    icon: "heroicons:bookmark-solid",
    color: "text-accent-500 dark:text-accent-400",
    bg: "bg-accent-50 dark:bg-accent-950/60",
    label: "Bookmarks",
  },
  snippet: {
    icon: "heroicons:code-bracket",
    color: "text-blue-500 dark:text-blue-400",
    bg: "bg-blue-50 dark:bg-blue-950/60",
    label: "Snippets",
  },
  todo: {
    icon: "heroicons:check-circle",
    color: "text-emerald-500 dark:text-emerald-400",
    bg: "bg-emerald-50 dark:bg-emerald-950/60",
    label: "Todos",
  },
  reminder: {
    icon: "heroicons:bell",
    color: "text-amber-500 dark:text-amber-400",
    bg: "bg-amber-50 dark:bg-amber-950/60",
    label: "Reminders",
  },
} as const;

type ResultType = keyof typeof typeConfig;

interface SearchResult {
  id: string;
  type: ResultType;
  title: string;
  sub: string;
  href: string;
  externalUrl?: string;
}

const sections = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return [];

  const out: { label: string; type: ResultType; items: SearchResult[] }[] = [];

  const notes = noteStore.notes
    .filter(
      (n) =>
        n.title?.toLowerCase().includes(q) ||
        n.content?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((n) => ({
      id: n.identifier,
      type: "note",
      title: n.title || "Untitled",
      sub: n.categories?.length ? n.categories.join(", ") : "Note",
      href: `/notes/edit-notes?id=${n.identifier}`,
    }));
  if (notes.length) out.push({ label: "Notes", type: "note", items: notes });

  const bookmarks = bookmarkStore.bookmarks
    .filter(
      (b) =>
        b.title?.toLowerCase().includes(q) ||
        b.url?.toLowerCase().includes(q) ||
        b.tag?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((b) => ({
      id: b.identifier,
      type: "bookmark",
      title: b.title,
      sub: b.url,
      href: "/bookmarks",
      externalUrl: b.url,
    }));
  if (bookmarks.length)
    out.push({ label: "Bookmarks", type: "bookmark", items: bookmarks });

  const snippets = snippetStore.snippets
    .filter(
      (s) =>
        s.title?.toLowerCase().includes(q) ||
        s.language?.toLowerCase().includes(q) ||
        s.description?.toLowerCase().includes(q) ||
        s.code?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((s) => ({
      id: s.identifier,
      type: "snippet",
      title: s.title || "Untitled",
      sub: s.language || "snippet",
      href: `/snippets/view-snippet?id=${s.identifier}`,
    }));
  if (snippets.length)
    out.push({ label: "Snippets", type: "snippet", items: snippets });

  const todos = todoStore.todos
    .filter(
      (t) =>
        t.title?.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((t) => ({
      id: t.identifier,
      type: "todo",
      title: t.title,
      sub: t.priority,
      href: `/todo/edit-todo?id=${t.identifier}`,
    }));
  if (todos.length) out.push({ label: "Todos", type: "todo", items: todos });

  const reminders = reminderStore.reminders
    .filter(
      (r) =>
        r.title?.toLowerCase().includes(q) ||
        r.description?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((r) => ({
      id: r.identifier,
      type: "reminder",
      title: r.title,
      sub: new Date(r.remindAt).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
      }),
      href: `/reminders/edit-reminder?id=${r.identifier}`,
    }));
  if (reminders.length)
    out.push({ label: "Reminders", type: "reminder", items: reminders });

  return out;
});

const flatResults = computed(() => sections.value.flatMap((s) => s.items));
const hasResults = computed(() => flatResults.value.length > 0);

// Reset active index when query changes
watch(searchQuery, () => {
  activeIndex.value = -1;
});

function navigate(item: SearchResult) {
  if (item.externalUrl) {
    openUrl(item.externalUrl);
  } else {
    router.push(item.href);
  }
  emit("close");
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "ArrowDown") {
    e.preventDefault();
    activeIndex.value = Math.min(
      activeIndex.value + 1,
      flatResults.value.length - 1,
    );
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    activeIndex.value = Math.max(activeIndex.value - 1, 0);
  } else if (e.key === "Enter" && activeIndex.value >= 0) {
    e.preventDefault();
    navigate(flatResults.value[activeIndex.value]);
  } else if (e.key === "Escape") {
    emit("close");
  }
}

// Expose keyboard handler so parent (titlebar) can attach it to the input
defineExpose({ handleKeydown });

// Build a flat index map for active highlighting across sections
const flatIndexByItem = computed(() => {
  const map = new Map<string, number>();
  flatResults.value.forEach((item, i) => map.set(item.id, i));
  return map;
});
</script>

<template>
  <div
    ref="dropdownRef"
    class="absolute top-full mt-2 left-1/2 -translate-x-1/2 w-[520px] max-h-[70vh] overflow-y-auto bg-white dark:bg-gray-900 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 z-50 py-2"
  >
    <!-- No results -->
    <div
      v-if="!hasResults"
      class="flex flex-col items-center justify-center py-10 gap-2 text-gray-400 dark:text-gray-500"
    >
      <UIcon name="heroicons:magnifying-glass" class="size-6" />
      <p class="text-sm">No results for "{{ searchQuery }}"</p>
    </div>

    <!-- Results grouped by type -->
    <template v-else>
      <div
        v-for="section in sections"
        :key="section.type"
        class="mb-1 last:mb-0"
      >
        <!-- Section header -->
        <div class="flex items-center gap-2 px-4 py-1.5">
          <span
            class="inline-flex items-center justify-center size-4 rounded"
            :class="typeConfig[section.type].bg"
          >
            <UIcon
              :name="typeConfig[section.type].icon"
              class="size-2.5"
              :class="typeConfig[section.type].color"
            />
          </span>
          <span
            class="text-[10px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500"
          >
            {{ section.label }}
          </span>
        </div>

        <!-- Items -->
        <button
          v-for="item in section.items"
          :key="item.id"
          class="w-full flex items-center gap-3 px-4 py-2 transition-colors text-left"
          :class="
            flatIndexByItem.get(item.id) === activeIndex
              ? 'bg-accent-50 dark:bg-accent-950/60'
              : 'hover:bg-gray-50 dark:hover:bg-gray-800'
          "
          @click="navigate(item)"
          @mouseenter="activeIndex = flatIndexByItem.get(item.id) ?? -1"
        >
          <span
            class="shrink-0 inline-flex items-center justify-center size-7 rounded-lg"
            :class="typeConfig[item.type].bg"
          >
            <UIcon
              :name="typeConfig[item.type].icon"
              class="size-3.5"
              :class="typeConfig[item.type].color"
            />
          </span>
          <span class="flex-1 min-w-0">
            <span
              class="block text-sm font-medium text-gray-800 dark:text-gray-100 truncate"
            >
              {{ item.title }}
            </span>
            <span
              class="block text-xs text-gray-400 dark:text-gray-500 truncate"
            >
              {{ item.sub }}
            </span>
          </span>
          <UIcon
            name="heroicons:arrow-right"
            class="size-3.5 shrink-0 text-gray-300 dark:text-gray-600"
          />
        </button>
      </div>
    </template>
  </div>
</template>
