import { defineStore } from "pinia";

export const useAuthStore = defineStore("auth_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    tokenExpiry: 0,
    /** Transient token used mid-flow for account verification / password reset. */
    pendingToken: "",
    /** Local guest session without a cloud account. */
    guestMode: false,
  }),

  getters: {
    isAuthenticated: (state) => !!state.accessToken,
    hasPendingToken: (state) => !!state.pendingToken,
    isGuest: (state) => state.guestMode,
  },

  actions: {
    setSession(accessToken: string, refreshToken: string, tokenExpiry = 0) {
      this.accessToken = accessToken;
      this.refreshToken = refreshToken;
      this.tokenExpiry = tokenExpiry;
    },

    setPendingToken(token: string) {
      this.pendingToken = token;
    },

    clearPendingToken() {
      this.pendingToken = "";
    },

    clearSession() {
      this.accessToken = "";
      this.refreshToken = "";
      this.tokenExpiry = 0;
      this.pendingToken = "";
    },

    enterGuestMode() {
      this.accessToken = "";
      this.refreshToken = "";
      this.tokenExpiry = 0;
      this.pendingToken = "";
      this.guestMode = true;
    },

    exitGuestMode() {
      this.guestMode = false;
    },
  },

  persist: true,
});
