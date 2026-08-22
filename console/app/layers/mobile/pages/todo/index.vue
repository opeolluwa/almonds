<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";
import EmptyState from "@shared/components/app/EmptyState.vue";

const todoStore = useTodoStore();
</script>

<template>
  <div>
    <!-- Create task FAB -->
    <div
      v-if="todoStore.todos.length !== 0"
      class="fixed bottom-20 right-5 z-40"
    >
      <AppFab aria-label="Add task" @click="navigateTo('/todo/create-todo')" />
    </div>

    <!-- Loading -->
    <div v-if="todoStore.loading" class="flex flex-col gap-2">
      <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
    </div>

    <!-- Empty state: no todos at all -->
    <div
      v-else-if="todoStore.todos.length === 0"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <EmptyState
        title="No task yet"
        description="Create your first task to get started."
        icon="ri:calendar-todo-line"
        action-label="create task"
        @action="navigateTo('/todo/create-todo')"
      />
    </div>
  </div>
</template>
