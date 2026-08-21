const STORAGE_KEY = "auth:last_email";

export function useRememberedEmail() {
  const rememberedEmail = ref("");

  function load() {
    try {
      rememberedEmail.value = localStorage.getItem(STORAGE_KEY) ?? "";
    } catch {
      rememberedEmail.value = "";
    }
  }

  function rememberEmail(email: string) {
    try {
      localStorage.setItem(STORAGE_KEY, email);
      rememberedEmail.value = email;
    } catch {
      console.warn("Failed to save email to localStorage");
    }
  }

  function forgetEmail() {
    try {
      localStorage.removeItem(STORAGE_KEY);
    } catch {
      console.warn("Failed to remove email from localStorage");
    }
    rememberedEmail.value = "";
  }

  load();

  return { rememberedEmail, rememberEmail, forgetEmail };
}
