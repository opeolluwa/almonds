<script setup lang="ts">
import TodoCard from "~/components/todo/todo-card.vue";
import { useTodoStore } from "~/stores/todo";

definePageMeta({ layout: false });

const todoStore = useTodoStore();
const router = useRouter();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "active" | "completed">("all");
type TodoSort = "priority-high" | "priority-low" | "name-asc" | "name-desc" | "date-newest" | "date-oldest";
const sortBy = ref<TodoSort>("priority-high");

const priorityOrder: Record<string, number> = { high: 0, medium: 1, low: 2 };

const sortItems = computed(() => [
  [
    {
      label: "Priority (high → low)",
      icon: sortBy.value === "priority-high" ? "heroicons:check" : "heroicons:flag",
      onSelect: () => { sortBy.value = "priority-high"; },
    },
    {
      label: "Priority (low → high)",
      icon: sortBy.value === "priority-low" ? "heroicons:check" : "heroicons:flag",
      onSelect: () => { sortBy.value = "priority-low"; },
    },
  ],
  [
    {
      label: "Name A–Z",
      icon: sortBy.value === "name-asc" ? "heroicons:check" : "heroicons:bars-arrow-up",
      onSelect: () => { sortBy.value = "name-asc"; },
    },
    {
      label: "Name Z–A",
      icon: sortBy.value === "name-desc" ? "heroicons:check" : "heroicons:bars-arrow-down",
      onSelect: () => { sortBy.value = "name-desc"; },
    },
  ],
  [
    {
      label: "Newest first",
      icon: sortBy.value === "date-newest" ? "heroicons:check" : "heroicons:clock",
      onSelect: () => { sortBy.value = "date-newest"; },
    },
    {
      label: "Oldest first",
      icon: sortBy.value === "date-oldest" ? "heroicons:check" : "heroicons:clock",
      onSelect: () => { sortBy.value = "date-oldest"; },
    },
  ],
]);

const filteredTodos = computed(() => {
  let list = todoStore.todos;

  if (filter.value === "active") list = list.filter((t) => !t.done);
  if (filter.value === "completed") list = list.filter((t) => t.done);

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (t) =>
        t.title.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    );
  }

  return [...list].sort((a, b) => {
    switch (sortBy.value) {
      case "priority-high": return (priorityOrder[a.priority] ?? 99) - (priorityOrder[b.priority] ?? 99);
      case "priority-low": return (priorityOrder[b.priority] ?? 99) - (priorityOrder[a.priority] ?? 99);
      case "name-asc": return a.title.localeCompare(b.title);
      case "name-desc": return b.title.localeCompare(a.title);
      case "date-newest": return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
      case "date-oldest": return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
    }
  });
});

async function handleToggle(identifier: string, done: boolean) {
  await todoStore.toggleDone(identifier, done);
}

async function handleDelete(identifier: string) {
  await todoStore.deleteTodo(identifier);
}

async function deleteCompleted() {
  await Promise.all(
    todoStore.completedTodos.map((t) => todoStore.deleteTodo(t.identifier)),
  );
}

function handleEdit(identifier: string) {
  router.push(`/todo/edit-todo?id=${identifier}`);
}

onMounted(async () => {
  setSearch({ placeholder: "Search todos..." });
  await todoStore.fetchTodos();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <PrimaryCta
        v-if="todoStore.todos.length !== 0"
        label="New Todo"
        icon="heroicons:plus"
        to="/todo/create-todo"
      />
    </template>

    <template #main_content>
      <!-- Filter tabs -->
      <div
        v-if="!todoStore.loading && todoStore.todos.length > 0"
        class="flex items-center gap-2 mb-4"
      >
        <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5">
          <button
            v-for="f in ['all', 'active', 'completed'] as const"
            :key="f"
            class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
            :class="
              filter === f
                ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
                : 'text-gray-500 dark:text-gray-400'
            "
            @click="filter = f"
          >
            {{ f }}
          </button>
        </div>
        <UDropdownMenu
          :items="sortItems"
          size="sm"
          :ui="{
            content: 'min-w-44 rounded-xl shadow-lg border border-gray-100 dark:border-gray-800 py-1',
            item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
            separator: 'my-1 mx-2',
          }"
        >
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-xs font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
          >
            <UIcon name="heroicons:arrows-up-down" class="size-3.5" />
            Sort
          </button>
        </UDropdownMenu>
        <button
          v-if="todoStore.completedTodos.length > 0"
          class="ml-auto flex items-center gap-1.5 text-xs text-red-400 hover:text-red-500 transition-colors"
          @click="deleteCompleted"
        >
          <UIcon name="heroicons:trash" class="size-3.5" />
          Clear done ({{ todoStore.completedTodos.length }})
        </button>
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
        <div
          class="mb-4 p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:check-circle"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No todos yet
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Create your first todo to get started.
        </p>
        <NuxtLink
          to="/todo/create-todo"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
        >
          Create todo
        </NuxtLink>
      </div>

      <!-- Empty state: search or filter yields no results -->
      <div
        v-else-if="filteredTodos.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-4 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon
            name="heroicons:magnifying-glass"
            class="w-8 h-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No results found
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Try a different search or filter.
        </p>
        <div class="flex gap-3">
          <button
            v-if="searchQuery"
            class="text-xs text-accent-500 hover:text-accent-600 font-medium"
            @click="searchQuery = ''"
          >
            Clear search
          </button>
          <button
            v-if="filter !== 'all'"
            class="text-xs text-gray-400 hover:text-gray-600 font-medium"
            @click="filter = 'all'"
          >
            Clear filter
          </button>
        </div>
      </div>

      <!-- Todo list -->
      <div v-else class="flex flex-col gap-2">
        <TodoCard
          v-for="todo in filteredTodos"
          :key="todo.identifier"
          :todo="todo"
          @toggle="handleToggle"
          @edit="handleEdit"
          @delete="handleDelete"
        />
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Summary
      </h2>
      <div class="flex flex-col gap-3 mb-6">
        <div class="bg-accent-50 dark:bg-accent-950 rounded-lg p-3">
          <p
            class="text-2xl font-semibold text-accent-700 dark:text-accent-300"
          >
            {{ todoStore.activeTodos.length }}
          </p>
          <p class="text-xs text-accent-500 dark:text-accent-400">
            Active tasks
          </p>
        </div>
        <div class="bg-emerald-50 dark:bg-emerald-950 rounded-lg p-3">
          <p
            class="text-2xl font-semibold text-emerald-700 dark:text-emerald-300"
          >
            {{ todoStore.completedTodos.length }}
          </p>
          <p class="text-xs text-emerald-500 dark:text-emerald-400">
            Completed
          </p>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Priority
      </h2>
      <div class="flex flex-col gap-2">
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <UIcon name="heroicons:flag" class="size-4 text-rose-500" />
            <span class="text-gray-600 dark:text-gray-400">High</span>
          </div>
          <span class="text-xs text-gray-400">{{
            todoStore.highPriorityCount
          }}</span>
        </div>
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <UIcon name="heroicons:flag" class="size-4 text-amber-500" />
            <span class="text-gray-600 dark:text-gray-400">Medium</span>
          </div>
          <span class="text-xs text-gray-400">{{
            todoStore.mediumPriorityCount
          }}</span>
        </div>
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <UIcon name="heroicons:flag" class="size-4 text-emerald-500" />
            <span class="text-gray-600 dark:text-gray-400">Low</span>
          </div>
          <span class="text-xs text-gray-400">{{
            todoStore.lowPriorityCount
          }}</span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
