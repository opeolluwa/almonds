<script setup lang="ts">
import SnippetCard from "~/components/snippets/snippet-card.vue";
import { useSnippetStore } from "~/stores/snippets";

definePageMeta({ layout: false });

const snippetStore = useSnippetStore();
const activeLanguage = ref("All");
const { searchQuery, setSearch, clearSearch } = useAppSearch();

const allLanguages = computed(() => ["All", ...snippetStore.languages]);

const filteredSnippets = computed(() => {
  let list = snippetStore.snippets;

  if (activeLanguage.value !== "All") {
    list = list.filter((s) => s.language === activeLanguage.value);
  }

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (s) =>
        s.title?.toLowerCase().includes(q) ||
        s.language?.toLowerCase().includes(q) ||
        s.description?.toLowerCase().includes(q) ||
        s.code.toLowerCase().includes(q),
    );
  }
  return list;
});

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function lineCount(code: string) {
  return code.split("\n").length;
}

onMounted(async () => {
  setSearch({ placeholder: "Search snippets..." });
  await Promise.all([
    snippetStore.fetchSnippets(),
    snippetStore.fetchRecentSnippets(),
  ]);
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <PrimaryCta
        v-if="snippetStore.snippets.length !== 0"
        label="New Snippet"
        icon="heroicons:plus"
        to="/snippets/create-snippets"
      />
    </template>

    <template #main_content>
      <!-- Language filter tabs -->
      <div
        v-if="!snippetStore.loading && allLanguages.length > 1"
        class="flex gap-2 mb-4 flex-wrap"
      >
        <button
          v-for="lang in allLanguages"
          :key="lang"
          class="px-3 py-1 rounded-full text-xs font-medium transition-colors"
          :class="
            activeLanguage === lang
              ? 'bg-accent-500 text-white'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
          "
          @click="activeLanguage = lang"
        >
          {{ lang }}
        </button>
      </div>

      <!-- Loading skeletons -->
      <div v-if="snippetStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-32 rounded-lg" />
      </div>

      <!-- Empty state: no snippets at all -->
      <div
        v-else-if="snippetStore.snippets.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div
          class="mb-4 p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:code-bracket"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No snippets yet
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Save your first code snippet to get started.
        </p>
        <NuxtLink
          to="/snippets/create-snippets"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
        >
          Create snippet
        </NuxtLink>
      </div>

      <!-- Empty state: filtered language has no results -->
      <div
        v-else-if="filteredSnippets.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-4 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon
            name="heroicons:funnel"
            class="w-8 h-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No results found
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Try a different search or language filter.
        </p>
        <div class="flex items-center gap-3">
          <button
            v-if="searchQuery"
            class="text-xs text-accent-500 hover:text-accent-600 font-medium"
            @click="searchQuery = ''"
          >
            Clear search
          </button>
          <button
            v-if="activeLanguage !== 'All'"
            class="text-xs text-gray-400 hover:text-gray-600 font-medium"
            @click="activeLanguage = 'All'"
          >
            Clear filter
          </button>
        </div>
      </div>

      <!-- Snippet list -->
      <div v-else class="flex flex-col gap-3">
        <SnippetCard
          v-for="snippet in filteredSnippets"
          :key="snippet.identifier"
          :identifier="snippet.identifier"
          :title="snippet.title ?? 'Untitled'"
          :language="snippet.language ?? 'Unknown'"
          :lines="lineCount(snippet.code)"
          :date="formatDate(snippet.createdAt)"
          :preview="snippet.code"
          :search-query="searchQuery"
        />
      </div>
    </template>

    <template #side_content>
      <!-- Languages section -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Languages
      </h2>

      <div v-if="snippetStore.loading" class="flex flex-col gap-1">
        <USkeleton v-for="i in 4" :key="i" class="h-8 rounded-lg" />
      </div>

      <div
        v-else-if="snippetStore.languages.length === 0"
        class="flex flex-col items-center py-6 text-center"
      >
        <UIcon
          name="heroicons:tag"
          class="w-5 h-5 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">No languages yet</p>
      </div>

      <div v-else class="flex flex-col gap-1">
        <div
          v-for="lang in allLanguages"
          :key="lang"
          class="flex items-center justify-between py-2 px-3 rounded-lg text-sm cursor-pointer transition-colors"
          :class="
            activeLanguage === lang
              ? 'bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
          "
          @click="activeLanguage = lang"
        >
          <span>{{ lang }}</span>
          <span class="text-xs text-gray-400">
            {{
              lang === "All"
                ? snippetStore.snippets.length
                : snippetStore.snippets.filter((s) => s.language === lang)
                    .length
            }}
          </span>
        </div>
      </div>

      <USeparator class="my-4" />

      <!-- Recent section -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Recent
      </h2>

      <div v-if="snippetStore.recentLoading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 3" :key="i" class="h-12 rounded-lg" />
      </div>

      <div
        v-else-if="snippetStore.recent.length === 0"
        class="flex flex-col items-center py-6 text-center"
      >
        <UIcon
          name="heroicons:clock"
          class="w-5 h-5 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">
          No recent snippets
        </p>
      </div>

      <div v-else class="flex flex-col gap-2">
        <div
          v-for="snippet in snippetStore.recent"
          :key="snippet.identifier"
          class="py-2 px-3 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
        >
          <p
            class="font-medium text-gray-700 dark:text-gray-300 truncate text-xs"
          >
            {{ snippet.title ?? "Untitled" }}
          </p>
          <p class="text-xs text-gray-400 mt-0.5">
            {{ formatDate(snippet.createdAt) }}
          </p>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
