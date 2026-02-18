<script setup lang="ts">
import CreateSnippetSideBar from "~/components/snippets/create-snippet-side-bar.vue";

definePageMeta({ layout: false, name: "Create snippet" });
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      Lorem ipsum dolor, sit amet consectetur adipisicing elit. Quo
      exercitationem amet, quaerat non libero autem id facere omnis tempore.
      Reiciendis fugiat placeat nemo voluptate illum quos sed asperiores debitis
      voluptates!
    </template>

    <template #side_content>
      <CreateSnippetSideBar />
    </template>
  </NuxtLayout>
</template>
