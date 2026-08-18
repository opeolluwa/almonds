import { defineStore } from "pinia";

export const useSidebarStore = defineStore("sidebar", {
  state: () => ({
    collapsed: false,
    size: 18,
  }),
  persist: true,
});
