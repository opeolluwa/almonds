import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export function useCopySnippet() {
  const copied = ref(false);

  async function copySnippet(code: string) {
    try {
      await writeText(code);
      copied.value = true;
      setTimeout(() => {
        copied.value = false;
      }, 2000);
    } catch (e) {
      console.error("Failed to copy snippet:", e);
    }
  }

  return { copySnippet, copied };
}
