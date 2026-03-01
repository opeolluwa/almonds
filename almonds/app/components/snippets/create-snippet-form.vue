<script setup lang="ts">
import type { CreateSnippetPayload } from "~/stores/snippets";
import { SNIPPET_LANGUAGES } from "~/data/languages";
import AppInput from "../forms/app-input.vue";
import AppSelect from "../forms/app-select.vue";

const snippetStore = useSnippetStore();
const router = useRouter();

const title = ref("");
const language = ref("");
const code = ref("");
const description = ref("");
const submitting = ref(false);
const error = ref<string | null>(null);

async function handleSubmit() {
  if (!code.value.trim()) return;

  submitting.value = true;
  error.value = null;

  const payload: CreateSnippetPayload = {
    title: title.value.trim() || null,
    language: language.value.trim() || null,
    code: code.value,
    description: description.value.trim() || null,
  };

  try {
    await snippetStore.createSnippet(payload);
    router.push("/snippets");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-5" @submit.prevent="handleSubmit">
    <div class="flex flex-col gap-1">
      <AppInput
        v-model="title"
        hint="(optional)"
        placeholder="My snippet"
        label="Title"
        name="snippet title"
        :disabled="submitting"
      />
    </div>

    <!-- Language -->

    <AppSelect
      v-model="language"
      name="language"
      label="Language"
      hint="optional"
      placeholder="e.g. TypeScript"
      :items="SNIPPET_LANGUAGES"
      :disabled="submitting"
    />

    <!-- Code -->
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
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
      <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
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
        Save snippet
      </UButton>
      <UButton
        type="button"
        variant="ghost"
        size="sm"
        :disabled="submitting"
        @click="router.push('/snippets')"
      >
        Cancel
      </UButton>
    </div>
  </form>
</template>
