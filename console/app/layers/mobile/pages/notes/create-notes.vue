<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { onBeforeRouteLeave } from "vue-router";
import EditorToolBar from "@mobile/components/notes/EditorToolBar.vue";
import NoteTitleInput from "@shared/components/notes/note-title-input.vue";
definePageMeta({ name: "New note", layout: "notes", keepalive: true });

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
  <div class="pb-24">
    <NoteTitleInput v-model="title" :disabled="submitting" />

    <NotesEditor v-model="content">
      <template #toolbar>
        <EditorToolBar />
      </template>
    </NotesEditor>

    <p v-if="error" class="text-xs text-red-500 mt-6">
      {{ error }}
    </p>

    <AppFab
      icon="ri:save-line"
      style="
        bottom: calc(
          var(--kb-inset, 0px) + env(safe-area-inset-bottom) + 4.5rem
        );
      "
      @click="handleSave"
    />
  </div>
</template>
