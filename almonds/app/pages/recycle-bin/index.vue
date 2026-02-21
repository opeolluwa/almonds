<script setup lang="ts">
import { useRecycleBinStore, type RecycleBinItemType } from "~/stores/recycle-bin";

definePageMeta({ layout: false });

const recycleBinStore = useRecycleBinStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const { notify } = useAppNotification();

const typeFilter = ref<RecycleBinItemType | "all">("all");
const confirmPurgeAll = ref(false);

const itemTypeOptions: Array<{ label: string; value: RecycleBinItemType | "all" }> = [
  { label: "All", value: "all" },
  { label: "Notes", value: "note" },
  { label: "Todos", value: "todo" },
  { label: "Bookmarks", value: "bookmark" },
  { label: "Reminders", value: "reminder" },
  { label: "Snippets", value: "snippet" },
];

const filteredEntries = computed(() => {
  let list = recycleBinStore.entries;

  if (typeFilter.value !== "all") {
    list = list.filter((e) => e.itemType === typeFilter.value);
  }

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter((e) => e.itemType.toLowerCase().includes(q));
  }

  return list;
});

function formatDeletedAt(deletedAt: string): string {
  return new Date(deletedAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

function itemTypeIcon(type: RecycleBinItemType): string {
  const icons: Record<RecycleBinItemType, string> = {
    note: "heroicons:document-text",
    todo: "heroicons:check-circle",
    bookmark: "heroicons:bookmark",
    reminder: "heroicons:clock",
    snippet: "heroicons:code-bracket",
  };
  return icons[type] ?? "heroicons:document";
}

async function handlePurge(identifier: string) {
  try {
    await recycleBinStore.purgeEntry(identifier);
    notify({ type: "success", message: "Item permanently deleted" });
  } catch {
    notify({ type: "error", message: "Failed to delete item" });
  }
}

async function handlePurgeAll() {
  if (!confirmPurgeAll.value) {
    confirmPurgeAll.value = true;
    return;
  }
  try {
    await recycleBinStore.purgeAll();
    notify({ type: "success", message: "Recycle bin emptied" });
  } catch {
    notify({ type: "error", message: "Failed to empty recycle bin" });
  } finally {
    confirmPurgeAll.value = false;
  }
}

onMounted(async () => {
  setSearch({ placeholder: "Search recycle bin..." });
  await recycleBinStore.fetchEntries();
});

onUnmounted(() => {
  clearSearch();
  confirmPurgeAll.value = false;
});
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <button
        v-if="recycleBinStore.entries.length !== 0"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors"
        :class="
          confirmPurgeAll
            ? 'bg-rose-500 text-white hover:bg-rose-600'
            : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700'
        "
        @click="handlePurgeAll"
      >
        <UIcon name="heroicons:trash" class="size-4" />
        {{ confirmPurgeAll ? "Confirm empty" : "Empty bin" }}
      </button>
    </template>

    <template #main_content>
      <!-- Type filter tabs -->
      <div
        v-if="!recycleBinStore.loading && recycleBinStore.entries.length > 0"
        class="flex gap-1 flex-wrap mb-4"
      >
        <button
          v-for="opt in itemTypeOptions"
          :key="opt.value"
          class="px-3 py-1.5 rounded-lg text-xs font-medium transition-colors capitalize"
          :class="
            typeFilter === opt.value
              ? 'bg-gray-800 dark:bg-gray-100 text-white dark:text-gray-900'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
          "
          @click="typeFilter = opt.value"
        >
          {{ opt.label }}
          <span
            v-if="opt.value !== 'all' && recycleBinStore.typeCounts[opt.value]"
            class="ml-1 opacity-60"
          >
            {{ recycleBinStore.typeCounts[opt.value] }}
          </span>
        </button>
      </div>

      <!-- Loading -->
      <div v-if="recycleBinStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-14 rounded-lg" />
      </div>

      <!-- Empty state: bin is empty -->
      <div
        v-else-if="recycleBinStore.entries.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div
          class="mb-4 p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:trash"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Recycle bin is empty
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500">
          Deleted items will appear here.
        </p>
      </div>

      <!-- Empty state: filter yields no results -->
      <div
        v-else-if="filteredEntries.length === 0"
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
        <div class="flex gap-3 mt-3">
          <button
            v-if="searchQuery"
            class="text-xs text-accent-500 hover:text-accent-600 font-medium"
            @click="searchQuery = ''"
          >
            Clear search
          </button>
          <button
            v-if="typeFilter !== 'all'"
            class="text-xs text-gray-400 hover:text-gray-600 font-medium"
            @click="typeFilter = 'all'"
          >
            Clear filter
          </button>
        </div>
      </div>

      <!-- Entry list -->
      <div v-else class="flex flex-col gap-2">
        <div
          v-for="entry in filteredEntries"
          :key="entry.identifier"
          class="flex items-center gap-3 bg-white dark:bg-gray-800 rounded-lg px-4 py-3 border border-gray-100 dark:border-gray-700"
        >
          <div class="p-1.5 rounded-md bg-gray-100 dark:bg-gray-700">
            <UIcon
              :name="itemTypeIcon(entry.itemType)"
              class="size-4 text-gray-400 dark:text-gray-500"
            />
          </div>

          <div class="flex-1 min-w-0">
            <p class="text-xs font-medium text-gray-500 dark:text-gray-400 capitalize">
              {{ entry.itemType }}
            </p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
              Deleted {{ formatDeletedAt(entry.deletedAt) }}
            </p>
          </div>

          <button
            class="p-1.5 rounded-md text-gray-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors shrink-0"
            title="Permanently delete"
            @click="handlePurge(entry.identifier)"
          >
            <UIcon name="heroicons:trash" class="size-4" />
          </button>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Summary
      </h2>
      <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 mb-6">
        <p class="text-2xl font-semibold text-gray-700 dark:text-gray-200">
          {{ recycleBinStore.entries.length }}
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500">deleted items</p>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        By type
      </h2>
      <div class="flex flex-col gap-2">
        <div
          v-for="opt in itemTypeOptions.filter((o) => o.value !== 'all')"
          :key="opt.value"
          class="flex items-center justify-between text-sm"
        >
          <div class="flex items-center gap-2">
            <UIcon
              :name="itemTypeIcon(opt.value as RecycleBinItemType)"
              class="size-4 text-gray-400"
            />
            <span class="text-gray-600 dark:text-gray-400 capitalize">{{ opt.label }}</span>
          </div>
          <span class="text-xs text-gray-400">
            {{ recycleBinStore.typeCounts[opt.value] ?? 0 }}
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
