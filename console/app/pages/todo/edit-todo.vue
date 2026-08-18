<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopEditTodo from "~/desktop/pages/todo/edit-todo.vue";
import MobileEditTodo from "~/mobile/pages/todo/edit-todo.vue";
</script>

<template>
  <DesktopEditTodo v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileEditTodo />
  </NuxtLayout>
</template>
