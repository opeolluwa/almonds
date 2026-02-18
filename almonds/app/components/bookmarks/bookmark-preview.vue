<script setup lang="ts">
import type { Bookmark } from "~/stores/bookmarks";
import { useClipboard } from "@vueuse/core";

const props = defineProps<{ bookmark: Bookmark | null }>();
const open = defineModel<boolean>("open", { required: true });

const iframeRef = ref<HTMLIFrameElement | null>(null);
const loading = ref(true);
const blocked = ref(false);
const copied = ref(false);

const { copy } = useClipboard();

const faviconUrl = computed(() => {
  if (!props.bookmark?.url) return null;
  try {
    const { hostname } = new URL(props.bookmark.url);
    return `https://www.google.com/s2/favicons?domain=${hostname}&sz=32`;
  } catch {
    return null;
  }
});

const displayUrl = computed(() => {
  if (!props.bookmark?.url) return "";
  try {
    const u = new URL(props.bookmark.url);
    return u.hostname + (u.pathname !== "/" ? u.pathname : "");
  } catch {
    return props.bookmark.url;
  }
});

watch(
  () => props.bookmark,
  () => {
    loading.value = true;
    blocked.value = false;
  },
);

function onLoad() {
  loading.value = false;
  // Try to detect if the iframe was blocked (e.g. X-Frame-Options)
  try {
    // If the content is empty the browser likely blocked it
    const doc = iframeRef.value?.contentDocument;
    if (doc && doc.body && doc.body.innerHTML === "") {
      blocked.value = true;
    }
  } catch {
    // Cross-origin access denied — page loaded fine
  }
}

function onError() {
  loading.value = false;
  blocked.value = true;
}

function reload() {
  if (!iframeRef.value || !props.bookmark?.url) return;
  loading.value = true;
  blocked.value = false;
  iframeRef.value.src = props.bookmark.url;
}

async function copyUrl() {
  if (!props.bookmark?.url) return;
  await copy(props.bookmark.url);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1800);
}
</script>

<template>
  <USlideover
    v-model:open="open"
    side="right"
    :ui="{
      content: 'w-full max-w-3xl flex flex-col',
      overlay: 'bg-black/40 backdrop-blur-sm',
    }"
  >
    <template #content>
      <div class="flex flex-col h-full bg-white dark:bg-gray-900">
        <!-- Browser chrome header -->
        <div
          class="shrink-0 flex items-center gap-2 px-3 py-2.5 border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900"
        >
          <!-- Traffic-light dots -->
          <div class="flex items-center gap-1.5 shrink-0">
            <button
              class="size-3 rounded-full bg-red-400 hover:bg-red-500 transition-colors"
              title="Close"
              @click="open = false"
            />
            <span class="size-3 rounded-full bg-yellow-400" />
            <span class="size-3 rounded-full bg-green-400" />
          </div>

          <!-- Favicon + URL bar -->
          <div
            class="flex-1 flex items-center gap-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg px-3 py-1.5 min-w-0"
          >
            <img
              v-if="faviconUrl"
              :src="faviconUrl"
              class="size-4 shrink-0 rounded-sm"
              alt=""
              @error="($event.target as HTMLImageElement).style.display = 'none'"
            />
            <UIcon
              v-else
              name="heroicons:globe-alt"
              class="size-4 shrink-0 text-gray-400"
            />
            <span
              class="text-xs text-gray-600 dark:text-gray-300 truncate font-mono select-all"
            >
              {{ bookmark?.url ?? "" }}
            </span>
          </div>

          <!-- Action buttons -->
          <div class="flex items-center gap-0.5 shrink-0">
            <UButton
              size="xs"
              color="neutral"
              variant="ghost"
              :icon="
                loading
                  ? 'heroicons:arrow-path'
                  : 'heroicons:arrow-path'
              "
              :class="loading ? 'animate-spin' : ''"
              title="Reload"
              @click="reload"
            />
            <UButton
              size="xs"
              color="neutral"
              variant="ghost"
              :icon="
                copied
                  ? 'heroicons:check'
                  : 'heroicons:clipboard-document'
              "
              :class="copied ? 'text-green-500' : ''"
              title="Copy URL"
              @click="copyUrl"
            />
            <UButton
              size="xs"
              color="neutral"
              variant="ghost"
              icon="heroicons:x-mark"
              title="Close"
              @click="open = false"
            />
          </div>
        </div>

        <!-- Page meta strip -->
        <div
          v-if="bookmark"
          class="shrink-0 flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800 bg-white dark:bg-gray-900"
        >
          <span
            class="px-2 py-0.5 rounded-full bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300 text-xs font-medium capitalize"
          >
            {{ bookmark.tag }}
          </span>
          <span class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">
            {{ bookmark.title }}
          </span>
          <span class="ml-auto text-xs text-gray-400 shrink-0">
            {{ displayUrl }}
          </span>
        </div>

        <!-- WebView area -->
        <div class="relative flex-1 overflow-hidden bg-gray-100 dark:bg-gray-950">
          <!-- Loading overlay -->
          <Transition name="fade">
            <div
              v-if="loading"
              class="absolute inset-0 z-10 flex flex-col items-center justify-center bg-white dark:bg-gray-900 gap-4"
            >
              <div class="relative size-12">
                <div
                  class="absolute inset-0 rounded-full border-2 border-accent-200 dark:border-accent-900"
                />
                <div
                  class="absolute inset-0 rounded-full border-2 border-transparent border-t-accent-500 animate-spin"
                />
              </div>
              <p class="text-sm text-gray-400">Loading preview…</p>
            </div>
          </Transition>

          <!-- Blocked / error fallback -->
          <Transition name="fade">
            <div
              v-if="!loading && blocked"
              class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-5 bg-white dark:bg-gray-900 px-8 text-center"
            >
              <div
                class="size-16 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center"
              >
                <UIcon
                  name="heroicons:shield-exclamation"
                  class="size-8 text-gray-400"
                />
              </div>
              <div class="flex flex-col gap-1">
                <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Preview blocked
                </p>
                <p class="text-xs text-gray-400 max-w-xs">
                  This site does not allow embedding in a preview panel. Copy
                  the URL and open it in your browser instead.
                </p>
              </div>
              <div class="flex gap-2">
                <UButton
                  size="sm"
                  color="neutral"
                  variant="outline"
                  :icon="copied ? 'heroicons:check' : 'heroicons:clipboard-document'"
                  :class="copied ? 'text-green-500' : ''"
                  @click="copyUrl"
                >
                  {{ copied ? "Copied!" : "Copy URL" }}
                </UButton>
              </div>
            </div>
          </Transition>

          <!-- Iframe -->
          <iframe
            v-if="bookmark?.url"
            ref="iframeRef"
            :src="bookmark.url"
            :key="bookmark.identifier"
            class="w-full h-full border-none"
            sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
            @load="onLoad"
            @error="onError"
          />
        </div>
      </div>
    </template>
  </USlideover>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
