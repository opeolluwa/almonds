<script setup lang="ts">
definePageMeta({
  layout: false,
  name: "Welcome back, Nick!",

});

const projects = [
  {
    title: "Workshop planning and ideas",
    date: "27th April, 2021",
    color: "violet",
  },
  { title: "Design exploration", date: "27th April, 2021", color: "amber" },
  { title: "Users' feedback", date: "27th April, 2021", color: "emerald" },
  {
    title: "Plans for future and other directions",
    date: "27th April, 2021",
    color: "rose",
  },
  { title: "Testing results", date: "27th April, 2021", color: "sky" },
];

const tasks = [
  { title: "Work on the plan", time: "3 hours left", starred: true },
  { title: "Collect feedback", time: "1 hour overdue", starred: false },
  { title: "Analyse test results", time: "16 hours left", starred: false },
  { title: "Conclusion", time: "1 day left", starred: false },
  { title: "Sort all ideas", time: "1 day left", starred: false },
];

const recentActivity = [
  { name: "Maya Hayes", action: "Design challenges", time: "27th April, 2021" },
  {
    name: "Cassie Melendez",
    action: "Note B - Dribbble strategy",
    time: "27th April, 2021",
  },
  { name: "Ronny Schultz", action: "A - Pricing", time: "27th April, 2021" },
];

const colorMap: Record<string, string> = {
  violet: "border-l-violet-500",
  amber: "border-l-amber-500",
  emerald: "border-l-emerald-500",
  rose: "border-l-rose-500",
  sky: "border-l-sky-500",
};
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <section class="mb-8">
        <h2
          class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3 flex items-center gap-2"
        >
          <UIcon name="heroicons:folder" class="size-4" />
          My projects
        </h2>
        <div class="grid grid-cols-3 gap-3">
          <div
            v-for="project in projects"
            :key="project.title"
            class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow cursor-pointer border-l-4"
            :class="colorMap[project.color]"
          >
            <h3
              class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-2 line-clamp-2"
            >
              {{ project.title }}
            </h3>
            <p class="text-xs text-gray-400">{{ project.date }}</p>
          </div>
        </div>
      </section>

      <section>
        <h2
          class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3 flex items-center gap-2"
        >
          <UIcon name="heroicons:document-text" class="size-4" />
          My notes
        </h2>
        <div class="grid grid-cols-2 gap-3">
          <div
            class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700"
          >
            <h3
              class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-1"
            >
              Plans for future and other directions
            </h3>
            <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">
              A Design Direction unifies everyone and adds meaning to web
              design. It's a combination of art...
            </p>
          </div>
          <div
            class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700"
          >
            <h3
              class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-1"
            >
              Design challenges
            </h3>
            <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">
              What is A Design Challenge? Design challenges are exercises or
              competitions that designers can do to boost creativity...
            </p>
          </div>
        </div>
      </section>
    </template>

    <template #side_content>
      <section class="mb-8">
        <h2
          class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3 flex items-center gap-2"
        >
          <UIcon name="heroicons:clipboard-document-check" class="size-4" />
          My tasks
        </h2>
        <div class="flex flex-col gap-2">
          <div
            v-for="task in tasks"
            :key="task.title"
            class="flex items-center gap-3 py-2"
          >
            <UIcon
              name="heroicons:check-circle"
              class="size-5 text-gray-300 dark:text-gray-600 shrink-0"
            />
            <div class="flex-1 min-w-0">
              <p class="text-sm text-gray-700 dark:text-gray-300 truncate">
                {{ task.title }}
              </p>
              <p class="text-xs text-gray-400">{{ task.time }}</p>
            </div>
            <UIcon
              v-if="task.starred"
              name="heroicons:star-solid"
              class="size-4 text-amber-400 shrink-0"
            />
            <UIcon
              v-else
              name="heroicons:star"
              class="size-4 text-gray-300 dark:text-gray-600 shrink-0"
            />
          </div>
        </div>
      </section>

      <USeparator class="my-4" />

      <section>
        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Recent activity
        </h2>
        <div class="flex flex-col gap-3">
          <div
            v-for="activity in recentActivity"
            :key="activity.name"
            class="flex items-center gap-3"
          >
            <div
              class="size-8 rounded-full bg-violet-100 dark:bg-violet-900 flex items-center justify-center shrink-0"
            >
              <span
                class="text-xs font-medium text-violet-600 dark:text-violet-300"
                >{{ activity.name.charAt(0) }}</span
              >
            </div>
            <div class="flex-1 min-w-0">
              <p
                class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate"
              >
                {{ activity.name }}
              </p>
              <p class="text-xs text-gray-400 truncate">
                {{ activity.action }}
              </p>
            </div>
            <p class="text-xs text-gray-400 shrink-0 whitespace-nowrap">
              {{ activity.time }}
            </p>
          </div>
        </div>
      </section>
    </template>
  </NuxtLayout>
</template>
