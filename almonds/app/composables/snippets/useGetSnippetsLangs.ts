import { useGetSnippets } from "./useGetSnippets";

export function useSnippetLanguages() {
  const { snippets } = useGetSnippets();

  const languages = computed(() => {
    const langs = new Set<string>();
    for (const snippet of snippets.value) {
      if (snippet.language) {
        langs.add(snippet.language);
      }
    }
    return Array.from(langs).sort();
  });

  return { languages };
}
