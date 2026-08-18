<template>
  <div class="flex items-center gap-x-3">
    <div class="relative group cursor-pointer" @click="openFilePicker">
      <UAvatar
        v-if="userStore.user.profilePicture"
        :src="userStore.user.profilePicture"
        :alt="userStore.fullName"
        size="lg"
        class="squircle"
      />
      <UAvatar
        v-else
        :alt="userStore.fullName"
        :name="userStore.fullName"
        color="primary"
        size="lg"
        class="squircle"
      />
      <div
        class="absolute inset-0 flex items-center justify-center bg-black/40 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity"
      >
        <UIcon name="heroicons:camera" class="size-5 text-white" />
      </div>
      <input
        ref="fileInput"
        type="file"
        accept="image/*"
        class="hidden"
        @change="onFileSelected"
      />
    </div>
    <div class="flex flex-col">
      <span class="font-medium">{{ userStore.fullName }}</span>
      <small class="-mt-1 text-gray-400">{{ userStore.email }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUserInformationStore } from "~/stores/users";

const userStore = useUserInformationStore();
const toast = useToast();
const fileInput = ref<HTMLInputElement | null>(null);
const uploading = ref(false);

function openFilePicker() {
  fileInput.value?.click();
}

async function onFileSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  if (!file.type.startsWith("image/")) {
    toast.add({ title: "Please select an image file", color: "error" });
    return;
  }

  uploading.value = true;
  try {
    await userStore.uploadProfilePicture(file);
    toast.add({ title: "Profile picture updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to upload profile picture", color: "error" });
  } finally {
    uploading.value = false;
    if (fileInput.value) fileInput.value.value = "";
  }
}
</script>

<style>
.squircle {
  mask-image: url("data:image/svg+xml,%3csvg width='200' height='200' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M100 0C20 0 0 20 0 100s20 100 100 100 100-20 100-100S180 0 100 0Z'/%3e%3c/svg%3e");
  mask-size: contain;
  mask-position: center;
  mask-repeat: no-repeat;
}
</style>
