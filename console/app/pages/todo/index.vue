<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopTodo from "~/desktop/pages/todo/index.vue";
import MobileTodo from "~/mobile/pages/todo/index.vue";
</script>

<template>
  <DesktopTodo v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileTodo />
  </NuxtLayout>
</template>
