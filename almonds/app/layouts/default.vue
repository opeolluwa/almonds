<script setup lang="ts">
interface Route {
  path?: string | null;
  name: string;
  icon: string;
  activeIcon: string;
  onclick?: () => void;
}

const routes: Route[] = [
  {
    path: null,
    name: "Search",
    icon: "heroicons:magnifying-glass",
    activeIcon: "heroicons:magnifying-glass-solid",
  },
  {
    name: "Home",
    icon: "heroicons-outline:home",
    activeIcon: "heroicons-solid:home",
  },
  {
    name: "Notes",
    icon: "heroicons:document-text",
    activeIcon: "heroicons:document-text-solid",
    path: "/notes",
  },
  {
    name: "Tasks",
    icon: "heroicons:check-circle",
    activeIcon: "heroicons:check-circle-solid",
  },
];

const secondaryRoutes: Route[] = [
  {
    path: null,
    name: "Settings",
    icon: "heroicons:magnifying-glass",
    activeIcon: "heroicons:magnifying-glass-solid",
  },
  {
    name: "Color mode",
    icon: "heroicons-outline:home",
    activeIcon: "heroicons-solid:home",
  },

  {
    name: "Pricacy policy",
    icon: "heroicons:check-circle",
    activeIcon: "heroicons:check-circle-solid",
  },
];
</script>

<template>
  <div class="w-full h-screen grid grid-cols-12" id="wild_almonds_app">
    <nav
      class="col-span-2 bg-white border-r-gray-300 border-r px-2 text-gray-500 h-full flex flex-col"
    >
      <UUser
        name="John Doe"
        description="Software Engineer"
        :avatar="{
          src: 'https://i.pravatar.cc/150?u=john-doe',
          icon: 'i-lucide-image',
        }"
      />
      <USeparator class="my-4" />
      <div class="flex flex-col gap-2">
        <NuxtLink
          :to="route.path ? route.path : '#'"
          v-for="route in routes"
          :key="route.name"
          class="flex items-center gap-3 py-3 cursor-pointer rounded-lg hover:bg-gray-100 transition-colors"
          @click="route.onclick"
        >
          <UIcon :name="route.icon" class="size-5" />
          <span>{{ route.name }}</span>
        </NuxtLink>
      </div>
      <USeparator class="my-4 mb-6" />

      <div class="flex flex-col gap-2 mt-auto">
        <NuxtLink
          :to="route.path ? route.path : '#'"
          v-for="route in secondaryRoutes"
          :key="route.name"
          class="flex items-center gap-3 py-3 cursor-pointer rounded-lg hover:bg-gray-100 transition-colors"
          @click="route.onclick"
        >
          <UIcon :name="route.icon" class="size-5" />
          <span>{{ route.name }}</span>
        </NuxtLink>
      </div>
    </nav>

    <main class="col-span-7">
      <slot name="main_content" />
    </main>

    <aside class="col-span-3">
      <slot name="side_content" />
    </aside>
  </div>
</template>

<style scoped>
@import "tailwindcss";

#wild_almonds_app > * {
  @apply pt-5 pb-12;
  @apply px-5;
}

#wild_almonds_app > * > div {
  @apply bg-white;
  @apply rounded-lg;
}
</style>
