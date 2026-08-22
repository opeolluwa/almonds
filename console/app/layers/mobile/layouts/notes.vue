<script lang="ts" setup>
import EditorToolBar from "@mobile/components/notes/EditorToolBar.vue";

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

    <EditorToolBar v-if="showEditorToolBar" />
    <AppSideNav :mobile-nav-open="mobileNavOpen" />
  </main>
</template>
