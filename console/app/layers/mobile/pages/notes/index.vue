<script setup lang="ts">
import EmptyState from "@shared/components/app/EmptyState.vue";
import { useNoteStore } from "@shared/stores/notes";

const noteStore = useNoteStore();
</script>

<template>
  <!-- Loading -->
  <div v-if="noteStore.loading" class="flex flex-col gap-3">
    <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
  </div>

  <!-- Empty state: no notes at all -->
  <div
    v-else-if="noteStore.notes.length === 0"
    class="flex flex-col items-center justify-center py-20 text-center"
  >
    <EmptyState
      title="No notes yet"
      description="Create your first note to get started."
      icon="ri:booklet-line"
      action-label="create note"
      @action="navigateTo('/notes/create-notes')"
    />
  </div>
</template>
