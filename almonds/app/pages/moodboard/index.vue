<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useMoodboardStore } from "~/stores/moodboard";

definePageMeta({ layout: false });

const moodboardStore = useMoodboardStore();
const fileInputRef = ref<HTMLInputElement | null>(null);

// AI state
const aiAvailable = ref(false);
const modelReady = ref(false);
const downloading = ref(false);
const downloadStatus = ref("");
const downloadProgress = ref(0);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  moodboardStore.fetchImages();

  aiAvailable.value = await invoke<boolean>("is_ollama_installed");
  if (aiAvailable.value) {
    modelReady.value = await invoke<boolean>("check_ai_model");
  }

  unlisten = await listen<{
    status: string;
    total?: number;
    completed?: number;
  }>("ai://pull-progress", (event) => {
    downloadStatus.value = event.payload.status;
    if (event.payload.total && event.payload.completed) {
      downloadProgress.value = Math.round(
        (event.payload.completed / event.payload.total) * 100,
      );
    }
    if (event.payload.status === "success") {
      downloading.value = false;
      modelReady.value = true;
    }
  });
});

onUnmounted(() => {
  unlisten?.();
});

async function downloadModel() {
  downloading.value = true;
  downloadStatus.value = "Starting download…";
  downloadProgress.value = 0;
  try {
    await invoke("pull_ai_model");
    modelReady.value = true;
  } catch {
    downloadStatus.value = "Download failed. Please try again.";
  } finally {
    downloading.value = false;
  }
}

function triggerUpload() {
  fileInputRef.value?.click();
}

async function handleFileChange(event: Event) {
  const files = (event.target as HTMLInputElement).files;
  if (!files || files.length === 0) return;

  for (const file of Array.from(files)) {
    const buffer = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buffer));
    await moodboardStore.saveImage(file.name, bytes);
  }

  if (fileInputRef.value) fileInputRef.value.value = "";
}

async function handleDelete(filename: string) {
  await moodboardStore.deleteImage(filename);
}
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <!-- Desktop: full label -->
      <div class="hidden md:flex items-center justify-end">
        <button
          :disabled="moodboardStore.uploading"
          class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          @click="triggerUpload"
        >
          <UIcon
            :name="
              moodboardStore.uploading
                ? 'heroicons:arrow-path'
                : 'heroicons:plus'
            "
            :class="['size-4', moodboardStore.uploading && 'animate-spin']"
          />
          {{ moodboardStore.uploading ? "Uploading…" : "Add Image" }}
        </button>
      </div>
      <!-- Mobile: icon-only round FAB -->
      <button
        :disabled="moodboardStore.uploading"
        class="md:hidden flex items-center justify-center w-14 h-14 bg-accent-500 text-white rounded-full shadow-xl active:scale-95 transition-transform disabled:opacity-50"
        aria-label="Add Image"
        @click="triggerUpload"
      >
        <UIcon
          :name="
            moodboardStore.uploading ? 'heroicons:arrow-path' : 'heroicons:plus'
          "
          :class="['size-6', moodboardStore.uploading && 'animate-spin']"
        />
      </button>

      <!-- Hidden file input -->
      <input
        ref="fileInputRef"
        type="file"
        accept="image/*"
        multiple
        class="hidden"
        @change="handleFileChange"
      >
    </template>

    <template #main_content>
      <!-- Loading skeleton -->
      <div v-if="moodboardStore.loading" class="columns-2 gap-3 space-y-3">
        <div
          v-for="i in 4"
          :key="i"
          class="break-inside-avoid rounded-lg bg-gray-100 dark:bg-gray-800 animate-pulse"
          :style="{ height: `${120 + (i % 3) * 60}px` }"
        />
      </div>

      <!-- Empty state -->
      <div
        v-else-if="moodboardStore.images.length === 0"
        class="flex flex-col items-center justify-center h-full py-24 text-center"
      >
        <div
          class="size-16 rounded-2xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center mb-4"
        >
          <UIcon
            name="heroicons:photo"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No images yet
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Add images to start building your moodboard.
        </p>
        <button
          class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
          @click="triggerUpload"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          Add Image
        </button>
      </div>

      <!-- Masonry grid -->
      <div v-else class="columns-2 gap-3 space-y-3">
        <div
          v-for="image in moodboardStore.images"
          :key="image.filename"
          class="break-inside-avoid bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 overflow-hidden hover:shadow-sm transition-shadow cursor-pointer group"
        >
          <div class="relative">
            <img
              :src="image.src"
              :alt="image.title"
              class="w-full object-cover"
            >
            <div
              class="absolute inset-0 bg-black/0 group-hover:bg-black/30 transition-colors flex items-end justify-between"
            >
              <p
                class="text-white text-xs font-medium p-3 opacity-0 group-hover:opacity-100 transition-opacity truncate"
              >
                {{ image.title }}
              </p>
              <button
                class="opacity-0 group-hover:opacity-100 transition-opacity m-2 p-1.5 rounded-lg bg-black/40 hover:bg-red-500/80 text-white"
                title="Delete image"
                @click.stop="handleDelete(image.filename)"
              >
                <UIcon name="heroicons:trash" class="size-3.5" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Boards
      </h2>
      <div
        class="flex flex-col items-center justify-center py-6 text-center mb-6"
      >
        <UIcon
          name="heroicons:squares-2x2"
          class="size-6 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">No boards yet</p>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tags
      </h2>
      <p class="text-xs text-gray-400 dark:text-gray-500">No tags yet</p>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        AI
      </h2>

      <!-- AI service not running -->
      <div v-if="!aiAvailable" class="flex flex-col gap-1">
        <p class="text-xs text-gray-400 dark:text-gray-500">
          AI service is not running.
        </p>
      </div>

      <!-- Model ready -->
      <div v-else-if="modelReady" class="flex items-center gap-2">
        <UIcon
          name="heroicons:check-circle"
          class="size-4 text-green-500 shrink-0"
        />
        <p class="text-xs text-gray-500 dark:text-gray-400">AI model ready</p>
      </div>

      <!-- Downloading -->
      <div v-else-if="downloading" class="flex flex-col gap-2">
        <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
          {{ downloadStatus }}
        </p>
        <div
          class="w-full h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden"
        >
          <div
            class="h-full bg-accent-500 rounded-full transition-all duration-300"
            :style="{ width: `${downloadProgress}%` }"
          />
        </div>
        <p class="text-xs text-gray-400 dark:text-gray-500 text-right">
          {{ downloadProgress }}%
        </p>
      </div>

      <!-- Prompt to download -->
      <div v-else class="flex flex-col gap-2">
        <p class="text-xs text-gray-400 dark:text-gray-500">
          Download the AI model to get smart suggestions for your images.
        </p>
        <button
          class="flex items-center gap-2 py-1.5 px-3 bg-accent-500 text-white rounded-lg text-xs font-medium hover:bg-accent-600 transition-colors"
          @click="downloadModel"
        >
          <UIcon name="heroicons:arrow-down-tray" class="size-3.5" />
          Download AI (~2 GB)
        </button>
      </div>
    </template>
  </NuxtLayout>
</template>
