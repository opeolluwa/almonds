<script setup lang="ts">
import CreateSnippetForm from "~/components/snippets/create-snippet-form.vue";

definePageMeta({ layout: false, name: "Create snippet" });
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-2xl">
        <CreateSnippetForm />
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tips
      </h2>
      <ul class="flex flex-col gap-3">
        <li
          v-for="tip in [
            'Title and language are optional but help you find snippets later.',
            'Language enables filtering on the snippets list.',
            'Add a description to explain what the snippet does.',
          ]"
          :key="tip"
          class="flex items-start gap-2 text-xs text-gray-400 dark:text-gray-500"
        >
          <UIcon
            name="heroicons:light-bulb"
            class="size-3.5 mt-0.5 shrink-0 text-accent-400"
          />
          {{ tip }}
        </li>
      </ul>
    </template>
  </NuxtLayout>
</template>
