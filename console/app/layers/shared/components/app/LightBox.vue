<script setup lang="ts">
import type { UploadsInterface } from "~/bindings/UploadsInterface";

const props = defineProps<{
  images: UploadsInterface[];
}>();

const model = defineModel<string | null>({ default: null });

const search = ref("");

const filtered = computed(() => {
  const q = search.value.toLowerCase().trim();
  if (!q) return props.images;
  return props.images.filter((img) => img.name.toLowerCase().includes(q));
});

function toggle(id: string) {
  model.value = model.value === id ? null : id;
}
</script>

<template>
  <div class="space-y-3">
    <AppInput
      v-model="search"
      name="lightbox-search"
      label="Search"
      placeholder="Search images..."
    />
    {{ images }}
    <div
      v-if="filtered.length === 0"
      class="text-sm text-gray-400 dark:text-white/30 text-center py-8"
    >
      No images found.
    </div>

    <div
      v-else
      class="grid grid-cols-3 sm:grid-cols-4 gap-3 max-h-64 overflow-y-auto"
    >
      <button
        v-for="img in filtered"
        :key="img.identifier"
        type="button"
        class="aspect-square rounded-xl overflow-hidden border-2 transition-all cursor-pointer"
        :class="
          model === img.identifier
            ? 'border-black dark:border-white ring-2 ring-black/20 dark:ring-white/20'
            : 'border-transparent hover:border-gray-300 dark:hover:border-white/20'
        "
        @click="toggle(img.identifier)"
      >
        <img
          :src="img.url"
          :alt="img.name"
          class="w-full h-full object-cover"
        />
      </button>
    </div>
  </div>
</template>
