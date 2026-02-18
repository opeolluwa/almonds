<script setup lang="ts">
import DatePicker from "~/components/ui/date-picker.vue";
import { useTodoStore } from "~/stores/todo";

definePageMeta({ layout: false });

const todoStore = useTodoStore();
const router = useRouter();
const route = useRoute();
const { notify } = useAppNotification();

const identifier = computed(() => route.query.id as string);

const form = reactive({
  title: "",
  description: "",
  dueDate: null as Date | null,
  priority: "medium" as "high" | "medium" | "low",
  done: false,
});

const loading = ref(true);
const submitting = ref(false);
const deleting = ref(false);

function toIsoDate(date: Date | null): string | null {
  if (!date) return null;
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, "0"),
    String(date.getDate()).padStart(2, "0"),
  ].join("-");
}

function formatDisplayDate(date: Date | null): string {
  if (!date) return "";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

onMounted(async () => {
  if (!identifier.value) {
    router.replace("/todo");
    return;
  }

  if (todoStore.todos.length === 0) {
    await todoStore.fetchTodos();
  }

  const todo = todoStore.todos.find((t) => t.identifier === identifier.value);
  if (!todo) {
    router.replace("/todo");
    return;
  }

  form.title = todo.title;
  form.description = todo.description ?? "";
  form.dueDate = todo.dueDate ? new Date(todo.dueDate) : null;
  form.priority = todo.priority;
  form.done = todo.done;
  loading.value = false;
});

async function handleSubmit() {
  if (!form.title.trim()) return;
  submitting.value = true;
  try {
    await todoStore.updateTodo(identifier.value, {
      title: form.title.trim(),
      description: form.description.trim() || undefined,
    });

    const todo = todoStore.todos.find((t) => t.identifier === identifier.value);
    if (todo) {
      if (todo.done !== form.done) {
        await todoStore.toggleDone(identifier.value, form.done);
      }
      if (todo.priority !== form.priority) {
        await todoStore.changePriority(identifier.value, form.priority);
      }
      const newDate = toIsoDate(form.dueDate);
      const existingDate = todo.dueDate
        ? toIsoDate(new Date(todo.dueDate))
        : null;
      if (newDate !== existingDate) {
        await todoStore.updateDueDate(identifier.value, newDate);
      }
    }

    notify({ type: "success", message: "Todo updated" });
    router.push("/todo");
  } catch {
    notify({ type: "error", message: "Failed to update todo" });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  deleting.value = true;
  try {
    await todoStore.deleteTodo(identifier.value);
    notify({ type: "success", message: "Todo deleted" });
    router.push("/todo");
  } catch {
    notify({ type: "error", message: "Failed to delete todo" });
  } finally {
    deleting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-lg">
        <div class="flex items-center gap-3 mb-6">
          <button
            class="p-1.5 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            @click="router.back()"
          >
            <UIcon name="heroicons:arrow-left" class="size-4" />
          </button>
          <h1 class="text-sm font-semibold text-gray-800 dark:text-gray-100">
            Edit Todo
          </h1>
        </div>

        <div v-if="loading" class="flex flex-col gap-4">
          <USkeleton v-for="i in 4" :key="i" class="h-10 rounded-lg" />
        </div>

        <form v-else class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Title <span class="text-rose-500">*</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              placeholder="What needs to be done?"
              class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500"
            >
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Description
            </label>
            <textarea
              v-model="form.description"
              placeholder="Add more details..."
              rows="3"
              class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500 resize-none"
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Due date
            </label>
            <div class="flex items-center gap-2">
              <UPopover class="flex-1">
                <button
                  type="button"
                  class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left"
                  :class="
                    form.dueDate
                      ? 'text-gray-700 dark:text-gray-200'
                      : 'text-gray-400 dark:text-gray-500'
                  "
                >
                  <UIcon
                    name="heroicons:calendar"
                    class="size-4 shrink-0 text-gray-400"
                  />
                  {{
                    form.dueDate
                      ? formatDisplayDate(form.dueDate)
                      : "Pick a date"
                  }}
                </button>
                <template #content>
                  <DatePicker v-model="form.dueDate" />
                </template>
              </UPopover>
              <button
                v-if="form.dueDate"
                type="button"
                class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
                @click="form.dueDate = null"
              >
                <UIcon name="heroicons:x-mark" class="size-4" />
              </button>
            </div>
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Priority
            </label>
            <div class="flex gap-2">
              <button
                v-for="p in ['low', 'medium', 'high'] as const"
                :key="p"
                type="button"
                class="flex-1 py-2 rounded-lg text-xs font-medium capitalize border transition-colors"
                :class="
                  form.priority === p
                    ? p === 'high'
                      ? 'bg-rose-50 dark:bg-rose-950 border-rose-300 dark:border-rose-700 text-rose-600 dark:text-rose-400'
                      : p === 'medium'
                        ? 'bg-amber-50 dark:bg-amber-950 border-amber-300 dark:border-amber-700 text-amber-600 dark:text-amber-400'
                        : 'bg-emerald-50 dark:bg-emerald-950 border-emerald-300 dark:border-emerald-700 text-emerald-600 dark:text-emerald-400'
                    : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
                "
                @click="form.priority = p"
              >
                {{ p }}
              </button>
            </div>
          </div>

          <button
            type="button"
            class="flex items-center gap-3 py-3 px-4 rounded-lg border transition-colors w-full"
            :class="
              form.done
                ? 'bg-emerald-50 dark:bg-emerald-950 border-emerald-300 dark:border-emerald-700'
                : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
            "
            @click="form.done = !form.done"
          >
            <UIcon
              :name="
                form.done ? 'heroicons:check-circle-solid' : 'heroicons:circle'
              "
              class="size-5 transition-colors"
              :class="
                form.done
                  ? 'text-emerald-500'
                  : 'text-gray-300 dark:text-gray-600'
              "
            />
            <span
              class="text-sm font-medium transition-colors"
              :class="
                form.done
                  ? 'text-emerald-700 dark:text-emerald-300'
                  : 'text-gray-600 dark:text-gray-400'
              "
            >
              {{ form.done ? "Marked as done" : "Mark as done" }}
            </span>
          </button>

          <div class="flex items-center justify-between pt-2">
            <button
              type="button"
              :disabled="deleting"
              class="px-4 py-2 rounded-lg text-sm text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              @click="handleDelete"
            >
              {{ deleting ? "Deleting..." : "Delete" }}
            </button>
            <div class="flex gap-3">
              <button
                type="button"
                class="px-4 py-2 rounded-lg text-sm text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                @click="router.back()"
              >
                Cancel
              </button>
              <button
                type="submit"
                :disabled="!form.title.trim() || submitting"
                class="px-4 py-2 rounded-lg text-sm font-medium bg-accent-500 text-white hover:bg-accent-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {{ submitting ? "Saving..." : "Save" }}
              </button>
            </div>
          </div>
        </form>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Priority guide
      </h2>
      <div class="flex flex-col gap-3">
        <div
          class="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <UIcon
            name="heroicons:flag"
            class="size-4 text-rose-500 shrink-0 mt-px"
          />
          <span>
            <strong class="text-gray-700 dark:text-gray-300">High</strong>
            — Urgent, needs immediate attention
          </span>
        </div>
        <div
          class="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <UIcon
            name="heroicons:flag"
            class="size-4 text-amber-500 shrink-0 mt-px"
          />
          <span>
            <strong class="text-gray-700 dark:text-gray-300">Medium</strong>
            — Important but not urgent
          </span>
        </div>
        <div
          class="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <UIcon
            name="heroicons:flag"
            class="size-4 text-emerald-500 shrink-0 mt-px"
          />
          <span>
            <strong class="text-gray-700 dark:text-gray-300">Low</strong>
            — Nice to have
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
