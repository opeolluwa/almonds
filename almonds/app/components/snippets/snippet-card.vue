<script setup lang="ts">
import hljs from "highlight.js/lib/common";

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

const props = defineProps<{
  identifier: string;
  title: string;
  language: string;
  lines: number;
  date: string;
  preview: string;
  searchQuery?: string;
}>();

const router = useRouter();
const snippetStore = useSnippetStore();
const copied = ref(false);
const deleting = ref(false);
const showDeleteConfirm = ref(false);

async function confirmDelete() {
  deleting.value = true;
  await snippetStore.deleteSnippet(props.identifier);
}

const previewCode = computed(() =>
  props.preview.split("\n").slice(0, 10).join("\n"),
);

const highlighted = computed(() => {
  const lang = hlLanguageMap[props.language];
  if (lang && hljs.getLanguage(lang)) {
    return hljs.highlight(previewCode.value, { language: lang }).value;
  }
  return hljs.highlightAuto(previewCode.value).value;
});

// Find matching lines in code when a search query is active
const codeMatchInfo = computed(() => {
  const q = props.searchQuery?.trim().toLowerCase();
  if (!q || !props.preview.toLowerCase().includes(q)) return null;

  const codeLines = props.preview.split("\n");
  const matchIdx = codeLines.findIndex((l) => l.toLowerCase().includes(q));
  if (matchIdx === -1) return null;

  const context = 2;
  const start = Math.max(0, matchIdx - context);
  const end = Math.min(codeLines.length - 1, matchIdx + context);

  return { excerpt: codeLines.slice(start, end + 1), startLine: start + 1 };
});

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

const codeMatchHtml = computed(() => {
  if (!codeMatchInfo.value) return "";
  const rawQ = props.searchQuery!.trim();
  // escape both the HTML and the query so angle-bracket searches work correctly
  const escapedQ = escapeHtml(rawQ).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`(${escapedQ})`, "gi");
  return codeMatchInfo.value.excerpt
    .map((line) => escapeHtml(line).replace(re, "<mark>$1</mark>"))
    .join("\n");
});

async function copyCode() {
  await navigator.clipboard.writeText(props.preview);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
}
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow overflow-hidden cursor-pointer"
    @click="router.push(`/snippets/view-snippet?id=${identifier}`)"
  >
    <div class="p-4">
      <div class="flex items-center justify-between mb-2">
        <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200">
          {{ title }}
        </h3>
        <div class="flex items-center gap-3">
          <span
            v-if="codeMatchInfo"
            class="px-2 py-0.5 rounded bg-accent-100 dark:bg-accent-900 text-xs text-accent-600 dark:text-accent-400"
          >code match</span>
          <span
            class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-xs text-gray-500 dark:text-gray-400"
            >{{ language }}</span
          >
          <span class="text-xs text-gray-400">{{ lines }} lines</span>
        </div>
      </div>

      <!-- Match excerpt: centered on the first matching line -->
      <div v-if="codeMatchInfo" class="bg-gray-900 rounded-md text-xs overflow-x-auto">
        <div class="px-3 py-1 text-gray-500 border-b border-gray-700 text-xs font-mono">
          line {{ codeMatchInfo.startLine }}
        </div>
        <pre class="p-3"><code v-html="codeMatchHtml" /></pre>
      </div>

      <!-- Normal syntax-highlighted preview -->
      <pre
        v-else
        class="bg-gray-900 rounded-md p-3 text-xs overflow-x-auto"
      ><code v-html="highlighted"/></pre>
    </div>
    <div
      class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
    >
      <p class="text-xs text-gray-400">{{ date }}</p>
      <div class="flex items-center gap-2" @click.stop>
        <button
          class="text-xs font-medium transition-colors"
          :class="
            copied
              ? 'text-green-500'
              : 'text-accent-600 dark:text-accent-400 hover:text-accent-700'
          "
          @click="copyCode"
        >
          {{ copied ? "Copied!" : "Copy" }}
        </button>
        <button
          class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          @click="router.push(`/snippets/edit-snippet?id=${identifier}`)"
        >
          Edit
        </button>
        <button
          class="text-xs text-red-400 hover:text-red-600 dark:hover:text-red-300"
          @click="showDeleteConfirm = true"
        >
          Delete
        </button>
      </div>
    </div>

    <UModal v-model:open="showDeleteConfirm" title="Delete snippet">
      <template #body>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Are you sure you want to delete
          <span class="font-medium text-gray-800 dark:text-gray-200">{{
            title
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
  </div>
</template>

<style scoped>
pre :deep(code) {
  background: transparent;
  padding: 0;
}

pre :deep(mark) {
  background: #facc15;
  color: #111;
  border-radius: 2px;
  padding: 0 2px;
}
</style>
