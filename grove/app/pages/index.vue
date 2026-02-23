<template>
  <div class="min-h-screen bg-white dark:bg-[#0d1220] text-slate-900 dark:text-slate-100">
    <AppNav />
    <AppHero />
    <AppPreview />
    <AppFeatures />
    <AppAiSection />
    <AppTechStack />
    <!-- <AppPricing /> -->
    <AppDownload />
    <AppFooter />
  </div>
</template>

<script setup lang="ts">
useSeoMeta({
  title: 'Almonds — Your Personal Workspace, Reimagined',
  description:
    'A beautiful desktop app that unifies your notes, bookmarks, snippets, tasks, and local AI into one focused workspace. Free and open source.',
  ogTitle: 'Almonds — Your Personal Workspace, Reimagined',
  ogDescription:
    'Notes, bookmarks, snippets, tasks, and local AI — all in one beautiful desktop app. Free, open source, privacy-first.',
})
</script>
