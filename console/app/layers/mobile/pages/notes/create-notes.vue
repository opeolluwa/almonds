<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { onBeforeRouteLeave } from "vue-router";

definePageMeta({ name: "New note", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
  error.value = null;
  submitting.value = false;
  saved.value = false;
});

const lastSaved = ref<Date | null>(null);

const hasContent = computed(
  () => !!title.value.trim() || !!content.value.trim(),
);

async function handleSave() {
  if (!hasContent.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
    saved.value = true;
    lastSaved.value = new Date();
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

useEventListener("keydown", (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "s") {
    e.preventDefault();
    if (hasContent.value && !submitting.value) handleSave();
  }
});

onBeforeRouteLeave(async () => {
  if (submitting.value || saved.value) return;
  if (!hasContent.value) return;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <div>
    <textarea
      v-model="title"
      placeholder="Untitled"
      rows="1"
      :disabled="submitting"
      class="w-full resize-none bg-transparent outline-none text-xl mt-2 font-bold text-gray-900 dark:text-gray-200 placeholder:text-gray-400 dark:placeholder:text-gray-500 leading-tight mb-0 overflow-hidden"
      @input="
        ($event.target as HTMLTextAreaElement).style.height = 'auto';
        ($event.target as HTMLTextAreaElement).style.height =
          ($event.target as HTMLTextAreaElement).scrollHeight + 'px';
      "
    />

    <NotesEditor v-model="content" />

    <p v-if="error" class="text-xs text-red-500 mt-6">
      {{ error }}
    </p>

    <AppFab icon="ri:save-line" @click="handleSave" />
  </div>
</template>
