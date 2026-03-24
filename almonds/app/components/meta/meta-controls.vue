<script lang="ts" setup>
import { ref } from "vue";
import type { DropdownMenuItem } from "@nuxt/ui";

const { notify } = useAppNotification();

const workspaceStore = useWorkspacesStore();
const showDuplicateModal = ref(false);
const showTransferModal = ref(false);

const targetWorkspaceId = ref("");
const isSubmitting = ref(false);

const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const emit = defineEmits(["transferRecord", "duplicateRecord"]);
const props = defineProps({
  itemName: {
    type: String,
    required: false,
    default: "",
  },
});

const controls = [
  {
    label: "Duplicate",
    icon: "i-lucide-copy",
    onSelect: () => {
      showDuplicateModal.value = true;
    },
  },
  {
    label: "Transfer",
    icon: "i-lucide-share",
    onSelect: () => {
      showTransferModal.value = true;
    },
  },
];

const handleDuplicate = async () => {
  if (!targetWorkspaceId.value || isSubmitting.value) return;

  isSubmitting.value = true;

  try {
    emit("duplicateRecord", targetWorkspaceId.value);
    showDuplicateModal.value = false;
    targetWorkspaceId.value = "";
    notify({ message: `${props.itemName} duplicated successfully` });
  } finally {
    isSubmitting.value = false;
  }
};

const handleTransfer = async () => {
  if (!targetWorkspaceId.value || isSubmitting.value) return;

  isSubmitting.value = true;

  try {
    emit("transferRecord", targetWorkspaceId.value);
    showTransferModal.value = false;
    targetWorkspaceId.value = "";
    notify({ message: `${props.itemName} transferred successfully` });
  } finally {
    isSubmitting.value = false;
  }
};

watch(showDuplicateModal, (open) => {
  if (open) targetWorkspaceId.value = "";
});

watch(showTransferModal, (open) => {
  if (open) targetWorkspaceId.value = "";
});

const workspaces = computed<DropdownMenuItem[]>(() => [
  ...workspaceStore.workspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === currentWorkspaceId.value;
      return {
        label: w.name,
        value: w.identifier,

        icon: isActive
          ? "heroicons:check-circle-solid"
          : "heroicons:check-circle",
        class: isActive ? "font-semibold text-accent-500" : "",
        disabled: w.identifier === currentWorkspaceId.value,
      };
    }),
]);
</script>

<template>
  <UDropdownMenu :items="controls" size="sm">
    <UIcon
      name="heroicons:ellipsis-vertical"
      class="size-5 hover:text-accent-500 shrink-0"
    />
  </UDropdownMenu>

  <UModal v-model:open="showDuplicateModal">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Duplicate {{ itemName }}
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Create a duplicate of this in another workspace.
        </p>

        <AppSelect
          v-model="targetWorkspaceId"
          placeholder="Select target workspace"
          :items="workspaces"
          class="w-full mt-8"
        />
        <AppButton
          class="mt-2 w-fit ml-auto"
          :disabled="!targetWorkspaceId || isSubmitting"
          @click="handleDuplicate"
        >
          {{ isSubmitting ? "Processing..." : "Duplicate" }}
        </AppButton>
      </div>
    </template>
  </UModal>

  <UModal v-model:open="showTransferModal">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Transfer {{ itemName }}
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Transfer this workspace to another user.
        </p>

        <AppSelect
          v-model="targetWorkspaceId"
          placeholder="Select target workspace"
          label="Select target workspace"
          :items="workspaces"
          class="w-full mt-8"
        />
        <AppButton
          class="mt-2 w-fit ml-auto"
          :disabled="!targetWorkspaceId || isSubmitting"
          @click="handleTransfer"
        >
          {{ isSubmitting ? "Processing..." : "Transfer" }}
        </AppButton>
      </div>
    </template>
  </UModal>
</template>
