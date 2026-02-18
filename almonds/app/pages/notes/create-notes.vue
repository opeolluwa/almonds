<script setup lang="ts">
import NotesEditor from "~/components/notes/notes-editor.vue";
import { useNoteStore } from "~/stores/notes";

definePageMeta({ layout: false, name: "Create note", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const submitting = ref(false);
const error = ref<string | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
  error.value = null;
  submitting.value = false;
});

async function handleSave() {
  if (!title.value.trim() && !content.value.trim()) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Back -->
      <button
        class="flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors mb-5"
        @click="router.push('/notes')"
      >
        <UIcon name="heroicons:arrow-left" class="size-3.5" />
        Back
      </button>

      <!-- Title -->
      <UInput
        v-model="title"
        placeholder="Note title…"
        size="lg"
        variant="none"
        class="mb-4 text-xl font-semibold w-full"
        :disabled="submitting"
      />

      <!-- Editor -->
      <div class="border border-gray-100 dark:border-gray-700 rounded-lg overflow-hidden">
        <NotesEditor v-model="content" />
      </div>

      <p v-if="error" class="text-xs text-red-500 mt-3">{{ error }}</p>

      <!-- Actions -->
      <div class="flex items-center gap-2 mt-4">
        <UButton
          size="sm"
          :loading="submitting"
          :disabled="!title.trim() && !content.trim()"
          @click="handleSave"
        >
          Save note
        </UButton>
        <UButton
          variant="ghost"
          size="sm"
          :disabled="submitting"
          @click="router.push('/notes')"
        >
          Cancel
        </UButton>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tips
      </h2>
      <ul class="flex flex-col gap-3">
        <li
          v-for="tip in [
            'Type / in the editor for quick formatting commands.',
            'Use @ to mention people and : for emojis.',
            'Select text to access AI-powered improvements.',
            'Drag the handle on the left of any block to reorder.',
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
