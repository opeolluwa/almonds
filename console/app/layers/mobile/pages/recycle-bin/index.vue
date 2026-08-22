<script setup lang="ts">
import {
  useRecycleBinStore,
  type RecycleBinItemType,
} from "@shared/stores/recycle-bin";
import EmptyState from "@shared/components/app/EmptyState.vue";

const recycleBinStore = useRecycleBinStore();
const { searchQuery, clearSearch } = useAppSearch();
const { notify } = useAppNotification();

const typeFilter = ref<RecycleBinItemType | "all">("all");
const purgeTargetId = ref<string | null>(null);
const confirmEmptyBin = ref(false);
const purging = ref(false);

const itemTypeOptions: Array<{
  label: string;
  value: RecycleBinItemType | "all";
}> = [
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

const purgeTarget = computed(
  () =>
    recycleBinStore.entries.find((e) => e.identifier === purgeTargetId.value) ??
    null,
);

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

function formatDeletedAt(deletedAt: string): string {
  return new Date(deletedAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

function closePurgeConfirm() {
  if (!purging.value) purgeTargetId.value = null;
}

function closeEmptyBinConfirm() {
  if (!purging.value) confirmEmptyBin.value = false;
}

async function handlePurge() {
  if (!purgeTargetId.value) return;

  purging.value = true;

  try {
    await recycleBinStore.purgeEntry(purgeTargetId.value);
    notify({ type: "success", message: "Item permanently deleted" });
    purgeTargetId.value = null;
  } catch {
    notify({ type: "error", message: "Failed to delete item" });
  } finally {
    purging.value = false;
  }
}

async function handleEmptyBin() {
  purging.value = true;

  try {
    await recycleBinStore.purgeAll();
    notify({ type: "success", message: "Recycle bin emptied" });
    confirmEmptyBin.value = false;
  } catch {
    notify({ type: "error", message: "Failed to empty recycle bin" });
  } finally {
    purging.value = false;
  }
}

onMounted(async () => {
  await recycleBinStore.fetchEntries();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <div>
    <!-- Loading -->
    <div v-if="recycleBinStore.loading" class="flex flex-col gap-2">
      <USkeleton v-for="i in 4" :key="i" class="h-14 rounded-lg" />
    </div>

    <!-- Empty state: bin is empty -->
    <div v-else-if="recycleBinStore.entries.length === 0">
      <EmptyState
        title="Recycle bin is empty"
        description="Deleted items will appear here."
        icon="ri:delete-bin-line"
      />
    </div>

    <template v-else>
      <!-- Search + type filter -->
      <div class="mb-3">
        <AppInput
          v-model="searchQuery"
          name="search"
          icon="heroicons:magnifying-glass"
          placeholder="Search recycle bin..."
          size="sm"
        />

        <div class="flex gap-1 overflow-x-auto mt-2 -mx-1 px-1">
          <button
            v-for="opt in itemTypeOptions"
            :key="opt.value"
            class="shrink-0 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors capitalize"
            :class="
              typeFilter === opt.value
                ? 'bg-gray-800 dark:bg-gray-100 text-white dark:text-gray-900'
                : 'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400'
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
      </div>

      <!-- Summary bar -->
      <div class="flex items-center justify-between mb-3 px-1">
        <p class="text-xs text-gray-400 dark:text-gray-500">
          {{ filteredEntries.length }}
          {{ filteredEntries.length === 1 ? "item" : "items" }}
        </p>
        <button
          class="text-xs font-medium text-rose-500 hover:text-rose-600 transition-colors"
          @click="confirmEmptyBin = true"
        >
          Empty bin
        </button>
      </div>

      <!-- Empty state: filter/search yields no results -->
      <EmptyState
        v-if="filteredEntries.length === 0"
        title="No results found"
        description="Try a different search term or filter."
        icon="heroicons:magnifying-glass"
        action-label="clear filters"
        @action="
          () => {
            searchQuery = '';
            typeFilter = 'all';
          }
        "
      />

      <!-- Entry list -->
      <div v-else class="flex flex-col gap-2">
        <div
          v-for="entry in filteredEntries"
          :key="entry.identifier"
          class="flex items-center gap-3 bg-white dark:bg-gray-800 rounded-lg px-4 py-3 border border-gray-100 dark:border-gray-700"
        >
          <div
            class="p-2 rounded-md bg-gray-100 dark:bg-gray-700 shrink-0 self-start"
          >
            <UIcon
              :name="itemTypeIcon(entry.itemType)"
              class="size-4 text-gray-400 dark:text-gray-500"
            />
          </div>

          <div class="flex-1 min-w-0">
            <p
              class="text-sm font-medium text-gray-600 dark:text-gray-300 capitalize"
            >
              {{ entry.itemType }}
            </p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
              Deleted {{ formatDeletedAt(entry.deletedAt) }}
            </p>
          </div>

          <button
            class="p-2 rounded-md text-gray-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors shrink-0"
            aria-label="Permanently delete"
            @click="purgeTargetId = entry.identifier"
          >
            <UIcon name="heroicons:trash" class="size-4" />
          </button>
        </div>
      </div>
    </template>

    <!-- Purge single item confirm -->
    <UModal :open="!!purgeTargetId" @close="closePurgeConfirm">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            Delete permanently?
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            This
            <span class="capitalize font-medium">{{
              purgeTarget?.itemType ?? "item"
            }}</span>
            will be permanently deleted. This action cannot be undone.
          </p>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              color="error"
              :loading="purging"
              @click="handlePurge"
            >
              Delete
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="purging"
              @click="closePurgeConfirm"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>

    <!-- Empty bin confirm -->
    <UModal :open="confirmEmptyBin" @close="closeEmptyBinConfirm">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            Empty recycle bin?
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            All {{ recycleBinStore.entries.length }} items will be permanently
            deleted. This action cannot be undone.
          </p>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              color="error"
              :loading="purging"
              @click="handleEmptyBin"
            >
              Empty bin
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="purging"
              @click="closeEmptyBinConfirm"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
