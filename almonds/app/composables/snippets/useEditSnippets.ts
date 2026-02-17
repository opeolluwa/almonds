import type { Snippet } from "./useGetSnippets";

export function useEditSnippets() {
  const editingSnippet = useState<Snippet | null>("editingSnippet", () => null);
  const isEditing = computed(() => editingSnippet.value !== null);

  function startEditing(snippet: Snippet) {
    editingSnippet.value = { ...snippet };
  }

  function updateField<K extends keyof Snippet>(field: K, value: Snippet[K]) {
    if (editingSnippet.value) {
      editingSnippet.value = { ...editingSnippet.value, [field]: value };
    }
  }

  function cancelEditing() {
    editingSnippet.value = null;
  }

  return {
    editingSnippet,
    isEditing,
    startEditing,
    updateField,
    cancelEditing,
  };
}
