<script setup lang="ts">
definePageMeta({ layout: false });

interface Todo {
  id: number;
  title: string;
  completed: boolean;
  priority: "high" | "medium" | "low";
  dueDate: string;
}

const todos = ref<Todo[]>([
  {
    id: 1,
    title: "Review pull requests",
    completed: false,
    priority: "high",
    dueDate: "Today",
  },
  {
    id: 2,
    title: "Update documentation",
    completed: false,
    priority: "medium",
    dueDate: "Tomorrow",
  },
  {
    id: 3,
    title: "Fix navigation bug",
    completed: true,
    priority: "high",
    dueDate: "Yesterday",
  },
  {
    id: 4,
    title: "Design new dashboard layout",
    completed: false,
    priority: "low",
    dueDate: "Feb 20",
  },
  {
    id: 5,
    title: "Write unit tests",
    completed: false,
    priority: "medium",
    dueDate: "Feb 19",
  },
  {
    id: 6,
    title: "Optimize database queries",
    completed: true,
    priority: "high",
    dueDate: "Feb 15",
  },
]);

const newTodo = ref("");
const filter = ref<"all" | "active" | "completed">("all");

const filteredTodos = computed(() => {
  if (filter.value === "active") return todos.value.filter((t) => !t.completed);
  if (filter.value === "completed")
    return todos.value.filter((t) => t.completed);
  return todos.value;
});

const priorityColor: Record<string, string> = {
  high: "text-rose-500",
  medium: "text-amber-500",
  low: "text-emerald-500",
};

function toggleTodo(id: number) {
  const todo = todos.value.find((t) => t.id === id);
  if (todo) todo.completed = !todo.completed;
}

function addTodo() {
  if (!newTodo.value.trim()) return;
  todos.value.push({
    id: Date.now(),
    title: newTodo.value,
    completed: false,
    priority: "medium",
    dueDate: "No date",
  });
  newTodo.value = "";
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-center justify-end mb-6">
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
      </div>

      <div class="flex items-center gap-3 mb-4">
        <input
          v-model="newTodo"
          type="text"
          placeholder="Add a new task..."
          class="flex-1 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500"
          @keydown.enter="addTodo"
        />
        <button
          class="p-2.5 bg-accent-500 text-white rounded-lg hover:bg-accent-600 transition-colors"
          @click="addTodo"
        >
          <UIcon name="heroicons:plus" class="size-4" />
        </button>
      </div>

      <div class="flex flex-col gap-2">
        <div
          v-for="todo in filteredTodos"
          :key="todo.id"
          class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 flex items-center gap-4 hover:shadow-sm transition-shadow"
        >
          <button @click="toggleTodo(todo.id)">
            <UIcon
              :name="
                todo.completed
                  ? 'heroicons:check-circle-solid'
                  : 'heroicons:circle'
              "
              class="size-5"
              :class="
                todo.completed
                  ? 'text-accent-500'
                  : 'text-gray-300 dark:text-gray-600'
              "
            />
          </button>
          <div class="flex-1 min-w-0">
            <p
              class="text-sm"
              :class="
                todo.completed
                  ? 'line-through text-gray-400'
                  : 'text-gray-700 dark:text-gray-200'
              "
            >
              {{ todo.title }}
            </p>
          </div>
          <UIcon
            name="heroicons:flag"
            class="size-4"
            :class="priorityColor[todo.priority]"
          />
          <span class="text-xs text-gray-400 shrink-0">{{ todo.dueDate }}</span>
          <UIcon
            name="heroicons:ellipsis-vertical"
            class="size-4 text-gray-400 cursor-pointer"
          />
        </div>
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
            {{ todos.filter((t) => !t.completed).length }}
          </p>
          <p class="text-xs text-accent-500 dark:text-accent-400">
            Active tasks
          </p>
        </div>
        <div class="bg-emerald-50 dark:bg-emerald-950 rounded-lg p-3">
          <p
            class="text-2xl font-semibold text-emerald-700 dark:text-emerald-300"
          >
            {{ todos.filter((t) => t.completed).length }}
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
            todos.filter((t) => t.priority === "high").length
          }}</span>
        </div>
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <UIcon name="heroicons:flag" class="size-4 text-amber-500" />
            <span class="text-gray-600 dark:text-gray-400">Medium</span>
          </div>
          <span class="text-xs text-gray-400">{{
            todos.filter((t) => t.priority === "medium").length
          }}</span>
        </div>
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <UIcon name="heroicons:flag" class="size-4 text-emerald-500" />
            <span class="text-gray-600 dark:text-gray-400">Low</span>
          </div>
          <span class="text-xs text-gray-400">{{
            todos.filter((t) => t.priority === "low").length
          }}</span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
