<script setup lang="ts">
interface Route {
  path: string;
  name: string;
  icon: string;
  activeIcon: string;
}

const route = useRoute();
const colorMode = useColorMode();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});

function toggleTheme() {
  isDark.value = !isDark.value;
}

const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon"
);

const themeLabel = computed(() =>
  isDark.value ? "Light mode" : "Dark mode"
);

const primaryRoutes: Route[] = [
  {
    path: "/",
    name: "Home",
    icon: "heroicons:home",
    activeIcon: "heroicons:home-solid",
  },
  {
    path: "/notes",
    name: "Notes",
    icon: "heroicons:document-text",
    activeIcon: "heroicons:document-text-solid",
  },
  {
    path: "/bookmarks",
    name: "Bookmarks",
    icon: "heroicons:bookmark",
    activeIcon: "heroicons:bookmark-solid",
  },
  {
    path: "/ollama",
    name: "Ollama",
    icon: "heroicons:cpu-chip",
    activeIcon: "heroicons:cpu-chip-solid",
  },
  {
    path: "/snippets",
    name: "Snippets",
    icon: "heroicons:code-bracket",
    activeIcon: "heroicons:code-bracket-solid",
  },
  {
    path: "/todo",
    name: "Todo",
    icon: "heroicons:check-circle",
    activeIcon: "heroicons:check-circle-solid",
  },
  {
    path: "/moodboard",
    name: "Moodboard",
    icon: "heroicons:squares-2x2",
    activeIcon: "heroicons:squares-2x2-solid",
  },
  {
    path: "/scratch-pad",
    name: "Scratch Pad",
    icon: "heroicons:pencil-square",
    activeIcon: "heroicons:pencil-square-solid",
  },
];

const secondaryRoutes: Route[] = [
  {
    path: "/settings",
    name: "Settings",
    icon: "heroicons:cog-6-tooth",
    activeIcon: "heroicons:cog-6-tooth-solid",
  },
];

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<template>
  <div class="w-full h-screen grid grid-cols-12 bg-gray-50 dark:bg-surface-950" id="wild_almonds_app">
    <nav
      class="col-span-2 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 flex flex-col h-screen overflow-y-auto"
    >
      <div class="px-4 pt-5 pb-3">
        <UUser
          name="Nick Woods"
          description="nick.woods@gmail.com"
          :avatar="{
            src: 'https://i.pravatar.cc/150?u=nick-woods',
            icon: 'i-lucide-image',
          }"
        />
      </div>

      <div class="px-3 mb-2">
        <button
          class="w-full flex items-center justify-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          <span>New Project</span>
        </button>
      </div>

      <USeparator class="mx-3 my-2" />

      <div class="flex-1 px-3">
        <div class="flex flex-col gap-0.5">
          <NuxtLink
            v-for="r in primaryRoutes"
            :key="r.name"
            :to="r.path"
            class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
            :class="
              isActive(r.path)
                ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
            "
          >
            <UIcon
              :name="isActive(r.path) ? r.activeIcon : r.icon"
              class="size-4"
            />
            <span>{{ r.name }}</span>
          </NuxtLink>
        </div>
      </div>

      <div class="px-3 pb-5 mt-auto">
        <USeparator class="my-3" />
        <div class="flex flex-col gap-0.5">
          <button
            @click="toggleTheme"
            class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
          >
            <UIcon :name="themeIcon" class="size-5" />
            <span>{{ themeLabel }}</span>
          </button>
          <NuxtLink
            v-for="r in secondaryRoutes"
            :key="r.name"
            :to="r.path"
            class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
            :class="
              isActive(r.path)
                ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
            "
          >
            <UIcon
              :name="isActive(r.path) ? r.activeIcon : r.icon"
              class="size-4"
            />
            <span>{{ r.name }}</span>
          </NuxtLink>
        </div>
      </div>
    </nav>

    <main class="col-span-7 overflow-y-auto h-screen p-6">
      <slot name="main_content" />
    </main>

    <aside class="col-span-3 overflow-y-auto h-screen border-l border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 p-5">
      <slot name="side_content" />
    </aside>
  </div>
</template>
