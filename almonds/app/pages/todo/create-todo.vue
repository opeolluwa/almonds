<script setup lang="ts">
import DatePicker from "~/components/ui/date-picker.vue";
import { useTodoStore } from "~/stores/todo";

definePageMeta({ layout: false, name: "Create Todo" });

const todoStore = useTodoStore();
const router = useRouter();
const { notify } = useAppNotification();

const form = reactive({
  title: "",
  description: "",
  dueDate: null as Date | null,
  time: "",
  priority: "medium" as "high" | "medium" | "low",
});

const submitting = ref(false);

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

async function handleSubmit() {
  if (!form.title.trim()) return;
  submitting.value = true;
  try {
    await todoStore.createTodo({
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      dueDate: toIsoDate(form.dueDate) ?? undefined,
      time: form.time || undefined,
      priority: form.priority,
    });
    notify({ type: "success", message: "Todo created" });
    router.push("/todo");
  } catch {
    notify({ type: "error", message: "Failed to create todo" });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-lg">
        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Title <span class="text-rose-500">*</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              placeholder="What needs to be done?"
              autofocus
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
              Time
            </label>
            <div class="flex items-center gap-2">
              <div class="relative flex-1">
                <UIcon
                  name="heroicons:clock"
                  class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-gray-400 pointer-events-none"
                />
                <input
                  v-model="form.time"
                  type="time"
                  class="w-full bg-white dark:bg-gray-800 rounded-lg pl-9 pr-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
                >
              </div>
              <button
                v-if="form.time"
                type="button"
                class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
                @click="form.time = ''"
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

          <div class="flex justify-end gap-3 pt-2">
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
              {{ submitting ? "Creating..." : "Create" }}
            </button>
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
