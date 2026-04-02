<script setup lang="ts">
import { useMoodboardStore } from "~/stores/moodboard";

definePageMeta({ layout: false });

const moodboardStore = useMoodboardStore();
const fileInputRef = ref<HTMLInputElement | null>(null);

onMounted(() => {
  moodboardStore.fetchImages();
});

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
            :name="moodboardStore.uploading ? 'heroicons:arrow-path' : 'heroicons:plus'"
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
          :name="moodboardStore.uploading ? 'heroicons:arrow-path' : 'heroicons:plus'"
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
      />
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
            />
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
    </template>
  </NuxtLayout>
</template>
