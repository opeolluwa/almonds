<script setup lang="ts">
import hljs from "highlight.js/lib/common";

definePageMeta({ layout: false, name: "Snippets" });

const route = useRoute();
const router = useRouter();
const snippetStore = useSnippetStore();

const id = computed(() => route.query.id as string | undefined);
const snippet = computed(
  () => snippetStore.snippets.find((s) => s.identifier === id.value) ?? null,
);

const copied = ref(false);
const deleting = ref(false);
const showDeleteConfirm = ref(false);

const hlLanguageMap: Record<string, string> = {
  C: "c",
  "C++": "cpp",
  "C#": "csharp",
  Rust: "rust",
  Go: "go",
  Python: "python",
  Ruby: "ruby",
  PHP: "php",
  JavaScript: "javascript",
  TypeScript: "typescript",
  JSX: "javascript",
  TSX: "typescript",
  HTML: "xml",
  CSS: "css",
  SCSS: "scss",
  Less: "less",
  Bash: "bash",
  Zsh: "bash",
  PowerShell: "powershell",
  SQL: "sql",
  JSON: "json",
  YAML: "yaml",
  XML: "xml",
  Markdown: "markdown",
  Java: "java",
  Swift: "swift",
  Kotlin: "kotlin",
  Scala: "scala",
  Haskell: "haskell",
  Erlang: "erlang",
  Elixir: "elixir",
  R: "r",
  "Objective-C": "objectivec",
  GraphQL: "graphql",
  Docker: "dockerfile",
  "Docker Compose": "yaml",
  Makefile: "makefile",
  Vue: "xml",
  Svelte: "xml",
  React: "javascript",
  "Node.js": "javascript",
  Deno: "javascript",
  Bun: "javascript",
  Angular: "typescript",
};

const highlighted = computed(() => {
  if (!snippet.value) return "";
  const lang = hlLanguageMap[snippet.value.language ?? ""];
  if (lang && hljs.getLanguage(lang)) {
    return hljs.highlight(snippet.value.code, { language: lang }).value;
  }
  return hljs.highlightAuto(snippet.value.code).value;
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

async function copyCode() {
  if (!snippet.value) return;
  await navigator.clipboard.writeText(snippet.value.code);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
}

async function confirmDelete() {
  if (!snippet.value) return;
  deleting.value = true;
  await snippetStore.deleteSnippet(snippet.value.identifier);
  router.push("/snippets");
}

onMounted(async () => {
  if (snippetStore.snippets.length === 0) {
    await snippetStore.fetchSnippets();
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Not found -->
      <div
        v-if="!snippet"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon name="heroicons:code-bracket" class="size-7 text-gray-400" />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Snippet not found
        </h3>
        <button
          class="text-xs text-accent-500 hover:text-accent-600 font-medium mt-2"
          @click="router.push('/snippets')"
        >
          Back to snippets
        </button>
      </div>

      <template v-else>
        <!-- Header actions -->
        <div class="flex items-center justify-between mb-5">
          <div class="flex items-center gap-2">
            <UButton
              size="xs"
              variant="ghost"
              icon="heroicons:pencil-square"
              @click="
                router.push(`/snippets/edit-snippet?id=${snippet.identifier}`)
              "
            >
              Edit
            </UButton>
            <UButton
              size="xs"
              color="error"
              variant="ghost"
              icon="heroicons:trash"
              :loading="deleting"
              @click="showDeleteConfirm = true"
            >
              Delete
            </UButton>
          </div>
        </div>

        <!-- Title & meta -->
        <div class="mb-4">
          <h1
            class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-1"
          >
            {{ snippet.title ?? "Untitled" }}
          </h1>
          <div class="flex items-center gap-3">
            <span
              v-if="snippet.language"
              class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-xs text-gray-500 dark:text-gray-400"
            >
              {{ snippet.language }}
            </span>
            <span class="text-xs text-gray-400">
              {{ lineCount(snippet.code) }} lines
            </span>
          </div>
        </div>

        <!-- Description -->
        <p
          v-if="snippet.description"
          class="text-sm text-gray-500 dark:text-gray-400 mb-4"
        >
          {{ snippet.description }}
        </p>

        <!-- Code block -->
        <div class="relative group">
          <!-- eslint-disable-next-line vue/no-v-html — safe: content is hljs-escaped -->
          <pre
            class="bg-gray-900 rounded-lg p-4 text-xs overflow-x-auto"
          ><code v-html="highlighted"/></pre>
          <button
            class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity px-2 py-1 rounded text-xs font-medium"
            :class="
              copied
                ? 'bg-green-600 text-white'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            "
            @click="copyCode"
          >
            {{ copied ? "Copied!" : "Copy" }}
          </button>
        </div>

        <!-- Delete confirm modal -->
        <UModal v-model:open="showDeleteConfirm" title="Delete snippet">
          <template #body>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Are you sure you want to delete
              <span class="font-medium text-gray-800 dark:text-gray-200">{{
                snippet.title ?? "this snippet"
              }}</span
              >? This action cannot be undone.
            </p>
          </template>
          <template #footer>
            <div class="flex justify-end gap-2">
              <UButton
                size="xs"
                variant="ghost"
                :disabled="deleting"
                @click="showDeleteConfirm = false"
              >
                Cancel
              </UButton>
              <UButton
                size="xs"
                color="error"
                :loading="deleting"
                @click="confirmDelete"
              >
                Delete
              </UButton>
            </div>
          </template>
        </UModal>
      </template>
    </template>

    <template #side_content>
      <template v-if="snippet">
        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Details
        </h2>
        <div class="flex flex-col gap-3">
          <div>
            <p class="text-xs text-gray-400 mb-0.5">Language</p>
            <p class="text-xs text-gray-700 dark:text-gray-300">
              {{ snippet.language ?? "—" }}
            </p>
          </div>
          <div>
            <p class="text-xs text-gray-400 mb-0.5">Lines</p>
            <p class="text-xs text-gray-700 dark:text-gray-300">
              {{ lineCount(snippet.code) }}
            </p>
          </div>
          <div>
            <p class="text-xs text-gray-400 mb-0.5">Created</p>
            <p class="text-xs text-gray-700 dark:text-gray-300">
              {{ formatDate(snippet.createdAt) }}
            </p>
          </div>
          <div>
            <p class="text-xs text-gray-400 mb-0.5">Updated</p>
            <p class="text-xs text-gray-700 dark:text-gray-300">
              {{ formatDate(snippet.updatedAt) }}
            </p>
          </div>
        </div>

        <USeparator class="my-4" />

        <div class="flex flex-col gap-2">
          <UButton
            size="xs"
            variant="outline"
            icon="heroicons:clipboard"
            block
            @click="copyCode"
          >
            {{ copied ? "Copied!" : "Copy code" }}
          </UButton>
          <UButton
            size="xs"
            variant="outline"
            icon="heroicons:pencil-square"
            block
            @click="
              router.push(`/snippets/edit-snippet?id=${snippet.identifier}`)
            "
          >
            Edit snippet
          </UButton>
        </div>
      </template>
    </template>
  </NuxtLayout>
</template>

<style scoped>
pre :deep(code) {
  background: transparent;
  padding: 0;
}
</style>
