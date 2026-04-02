import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface MoodboardImage {
  filename: string;
  src: string;
  title: string;
}

export const useMoodboardStore = defineStore("moodboard_store", {
  state: () => ({
    images: [] as MoodboardImage[],
    loading: false,
    uploading: false,
  }),

  actions: {
    async fetchImages() {
      this.loading = true;
      try {
        this.images = await invoke<MoodboardImage[]>("list_moodboard_images");
      } finally {
        this.loading = false;
      }
    },

    async saveImage(filename: string, bytes: number[]): Promise<void> {
      this.uploading = true;
      try {
        await invoke("save_moodboard_image", { filename, bytes });
        await this.fetchImages();
      } finally {
        this.uploading = false;
      }
    },

    async deleteImage(filename: string): Promise<void> {
      await invoke("delete_moodboard_image", { filename });
      this.images = this.images.filter((img) => img.filename !== filename);
    },
  },
});
