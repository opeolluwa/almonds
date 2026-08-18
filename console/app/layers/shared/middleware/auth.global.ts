export default defineNuxtRouteMiddleware((to) => {
  const authStore = useAuthStore();
  const isAuthRoute = to.path.startsWith("/auth");
  const allowed = authStore.isAuthenticated || authStore.isGuest;

  if (!allowed) {
    if (!isAuthRoute) {
      return navigateTo("/auth/login");
    }
  } else if (isAuthRoute) {
    return navigateTo("/");
  }
});
