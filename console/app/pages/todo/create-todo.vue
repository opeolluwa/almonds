<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopCreateTodo from "~/desktop/pages/todo/create-todo.vue";
import MobileCreateTodo from "~/mobile/pages/todo/create-todo.vue";
</script>

<template>
  <DesktopCreateTodo v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileCreateTodo />
  </NuxtLayout>
</template>
