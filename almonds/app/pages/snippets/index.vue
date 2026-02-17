<script setup lang="ts">
definePageMeta({ layout: false });

const snippets = [
  {
    title: "Vue Composable Template",
    language: "TypeScript",
    lines: 24,
    date: "Feb 16, 2026",
    preview:
      "export function useCounter() {\n  const count = ref(0)\n  return { count }\n}",
  },
  {
    title: "Tailwind Card Component",
    language: "Vue",
    lines: 18,
    date: "Feb 15, 2026",
    preview: '<div class="rounded-lg p-4 bg-white shadow">\n  <slot />\n</div>',
  },
  {
    title: "Fetch API Wrapper",
    language: "TypeScript",
    lines: 32,
    date: "Feb 14, 2026",
    preview:
      "async function fetchAPI<T>(url: string): Promise<T> {\n  const res = await fetch(url)\n  return res.json()\n}",
  },
  {
    title: "CSS Grid Layout",
    language: "CSS",
    lines: 12,
    date: "Feb 12, 2026",
    preview:
      ".grid-layout {\n  display: grid;\n  grid-template-columns: repeat(3, 1fr);\n}",
  },
];

const languages = ["All", "TypeScript", "Vue", "CSS", "JavaScript"];
const activeLanguage = ref("All");
</script>

<template>
  <Script> hljs.highlightAll(); </Script>
  <NuxtLayout name="default">
    <template #main_content>
     
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
          Snippets
        </h1>
        <button
          class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          New Snippet
        </button>
      </div>

      <div class="flex gap-2 mb-4">
        <button
          v-for="lang in languages"
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

      <div class="flex flex-col gap-3">
        <div
          v-for="snippet in snippets"
          :key="snippet.title"
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow overflow-hidden"
        >
          <div class="p-4">
            <div class="flex items-center justify-between mb-2">
              <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200">
                {{ snippet.title }}
              </h3>
              <div class="flex items-center gap-3">
                <span
                  class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-xs text-gray-500 dark:text-gray-400"
                  >{{ snippet.language }}</span
                >
                <span class="text-xs text-gray-400"
                  >{{ snippet.lines }} lines</span
                >
              </div>
            </div>
            <pre
              class="bg-gray-900 text-gray-100 rounded-md p-3 text-xs overflow-x-auto"
            ><code>{{ snippet.preview }}</code></pre>
          </div>
          <div
            class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
          >
            <p class="text-xs text-gray-400">{{ snippet.date }}</p>
            <div class="flex items-center gap-2">
              <button
                class="text-xs text-accent-600 dark:text-accent-400 hover:text-accent-700 font-medium"
              >
                Copy
              </button>
              <button
                class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              >
                Edit
              </button>
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Languages
      </h2>
      <div class="flex flex-col gap-1">
        <div
          v-for="lang in languages"
          :key="lang"
          class="flex items-center justify-between py-2 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
        >
          <span>{{ lang }}</span>
          <span class="text-xs text-gray-400">{{
            lang === "All"
              ? snippets.length
              : snippets.filter((s) => s.language === lang).length
          }}</span>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Recent
      </h2>
      <div class="flex flex-col gap-2">
        <div
          v-for="snippet in snippets.slice(0, 3)"
          :key="snippet.title"
          class="py-2 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
        >
          <p
            class="font-medium text-gray-700 dark:text-gray-300 truncate text-xs"
          >
            {{ snippet.title }}
          </p>
          <p class="text-xs text-gray-400">{{ snippet.date }}</p>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
