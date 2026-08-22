<script lang="ts" setup>
const route = useRoute();
const router = useRouter();
const { mobileNavOpen } = useMobileNav();
const showEditorToolBar = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});
const editorHeaderTitle = computed(() =>
  route.path.includes("/create-notes") ? "New note" : "Edit notes",
);
</script>

<template>
  <main
    id="default_layout_mobile"
    class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
  >
    <AppHeader v-if="!showEditorToolBar" />
    <header
      v-else
      class="absolute top-0 left-0 z-50 flex max-h-20 min-h-15 w-full items-center gap-3 bg-white px-6 py-3.5 dark:bg-app-dark-800"
    >
      <NuxtLink class="inline-flex" @click="router.back()">
        <UIcon name="lucide:arrow-left" class="size-5" />
      </NuxtLink>
      <span
        class="truncate text-sm font-medium text-gray-900 dark:text-gray-200"
      >
        {{ editorHeaderTitle }}
      </span>
    </header>
    <AppViewport :hide-header-and-nav="showEditorToolBar">
<div class="mt-4">
          <slot />
</div>
    </AppViewport>

    <AppSideNav :mobile-nav-open="mobileNavOpen" />
  </main>
</template>
