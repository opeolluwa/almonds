<script setup lang="ts">
definePageMeta({ layout: "default" });

const authenticated = ref(true);

onMounted(() => {
  if (!authenticated.value) {
    navigateTo("/auth/login");
  }
});
</script>

<template>
  <div class="flex items-center justify-center h-full bg-red-500">
    <h1 class="text-lg font-semibold text-gray-500">Home</h1>
    Lorem, ipsum dolor sit amet consectetur adipisicing elit. Earum suscipit,
    similique quos ab aperiam sint molestias vel. Perferendis laudantium ducimus
    sequi laboriosam accusantium aspernatur, porro sint numquam illum architecto
    neque!
  </div>
</template>
