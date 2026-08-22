<script lang="ts" setup>
const route = useRoute();
const { mobileNavOpen } = useMobileNav();
const showEditorToolBar = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});
</script>

<template>
  <main
    id="default_layout_mobile"
    class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
  >
    <AppHeader v-if="!showEditorToolBar" />
    <AppViewport :hide-header-and-nav="showEditorToolBar">
      <slot />
    </AppViewport>

    <AppSideNav :mobile-nav-open="mobileNavOpen" />
  </main>
</template>
