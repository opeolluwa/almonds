<script setup lang="ts">
import TodoCard from "@shared/components/todo/todo-card.vue";
import { useTodoStore } from "@shared/stores/todo";
import EmptyState from "@shared/components/app/EmptyState.vue";

const todoStore = useTodoStore();
const router = useRouter();
</script>

<template>
  <NuxtLink
    v-if="todoStore.todos.length !== 0"
    to="/todo/create-todo"
    class="fixed bottom-20 right-5 z-40 flex items-center justify-center w-14 h-14 bg-primary-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
  >
    <UIcon name="heroicons:plus" class="size-6" />
  </NuxtLink>

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

  <!--
  <div v-else class="flex flex-col gap-2">
    <TodoCard
      v-for="todo in filteredTodos"
      :key="todo.identifier"
      :todo="todo"
      @toggle="handleToggle"
      @edit="handleEdit"
      @delete="handleDelete"
    />-->
  <!-- </div> -->
</template>
