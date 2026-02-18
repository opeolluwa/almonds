<script setup lang="ts">
import { SNIPPET_LANGUAGES } from "~/data/languages";

definePageMeta({ layout: false });

const route = useRoute();
const router = useRouter();
const snippetStore = useSnippetStore();

const id = computed(() => route.query.id as string | undefined);
const original = computed(
  () => snippetStore.snippets.find((s) => s.identifier === id.value) ?? null,
);

const title = ref("");
const language = ref("");
const code = ref("");
const description = ref("");
const submitting = ref(false);
const error = ref<string | null>(null);

watch(
  original,
  (s) => {
    if (s) {
      title.value = s.title ?? "";
      language.value = s.language ?? "";
      code.value = s.code;
      description.value = s.description ?? "";
    }
  },
  { immediate: true },
);

async function handleSubmit() {
  if (!code.value.trim() || !original.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await snippetStore.updateSnippet(original.value.identifier, {
      title: title.value.trim() || null,
      language: language.value.trim() || null,
      code: code.value,
      description: description.value.trim() || null,
    });
    router.push(`/snippets/view-snippet?id=${original.value.identifier}`);
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

onMounted(async () => {
  if (snippetStore.snippets.length === 0) {
    await snippetStore.fetchSnippets();
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Not found -->
      <div
        v-if="!original"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon name="heroicons:code-bracket" class="size-7 text-gray-400" />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Snippet not found
        </h3>
        <button
          class="text-xs text-accent-500 hover:text-accent-600 font-medium mt-2"
          @click="router.push('/snippets')"
        >
          Back to snippets
        </button>
      </div>

      <template v-else>
        <!-- Back -->
        <button
          class="flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors mb-5"
          @click="
            router.push(`/snippets/view-snippet?id=${original.identifier}`)
          "
        >
          <UIcon name="heroicons:arrow-left" class="size-3.5" />
          Back
        </button>

        <div class="max-w-2xl">
          <form class="flex flex-col gap-5" @submit.prevent="handleSubmit">
            <!-- Title -->
            <div class="flex flex-col gap-1">
              <label
                class="text-xs font-medium text-gray-600 dark:text-gray-400"
              >
                Title
                <span class="text-gray-400 dark:text-gray-600 font-normal ml-1"
                  >(optional)</span
                >
              </label>
              <UInput
                v-model="title"
                placeholder="My snippet"
                size="sm"
                :disabled="submitting"
              />
            </div>

            <!-- Language -->
            <div class="flex flex-col gap-1">
              <label
                class="text-xs font-medium text-gray-600 dark:text-gray-400"
              >
                Language
                <span class="text-gray-400 dark:text-gray-600 font-normal ml-1"
                  >(optional)</span
                >
              </label>
              <USelect
                v-model="language"
                placeholder="e.g. TypeScript"
                size="sm"
                :items="SNIPPET_LANGUAGES"
                :disabled="submitting"
              />
            </div>

            <!-- Code -->
            <div class="flex flex-col gap-1">
              <label
                class="text-xs font-medium text-gray-600 dark:text-gray-400"
              >
                Code
                <span class="text-red-400 ml-0.5">*</span>
              </label>
              <UTextarea
                v-model="code"
                placeholder="Paste your code here..."
                :rows="12"
                :disabled="submitting"
                class="font-mono"
              />
            </div>

            <!-- Description -->
            <div class="flex flex-col gap-1">
              <label
                class="text-xs font-medium text-gray-600 dark:text-gray-400"
              >
                Description
                <span class="text-gray-400 dark:text-gray-600 font-normal ml-1"
                  >(optional)</span
                >
              </label>
              <UTextarea
                v-model="description"
                placeholder="What does this snippet do?"
                :rows="3"
                :disabled="submitting"
              />
            </div>

            <!-- Error -->
            <p v-if="error" class="text-xs text-red-500">{{ error }}</p>

            <!-- Actions -->
            <div class="flex items-center gap-2">
              <UButton
                type="submit"
                size="sm"
                :loading="submitting"
                :disabled="!code.trim()"
              >
                Save changes
              </UButton>
              <UButton
                type="button"
                variant="ghost"
                size="sm"
                :disabled="submitting"
                @click="
                  router.push(
                    `/snippets/view-snippet?id=${original.identifier}`,
                  )
                "
              >
                Cancel
              </UButton>
            </div>
          </form>
        </div>
      </template>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tips
      </h2>
      <ul class="flex flex-col gap-3">
        <li
          v-for="tip in [
            'Language enables filtering on the snippets list.',
            'Add a description to explain what the snippet does.',
            'Changes are saved immediately after clicking Save changes.',
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
