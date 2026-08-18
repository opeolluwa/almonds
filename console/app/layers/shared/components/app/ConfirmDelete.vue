<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    title?: string;
    description?: string;
  }>(),
  {
    title: "Delete item?",
    description: "This action cannot be undone.",
  },
);

const { title, description } = props;
const modelValue = defineModel<boolean>();

const emit = defineEmits<{
  confirm: [];
}>();

function cancel() {
  modelValue.value = false;
}

function confirm() {
  emit("confirm");
  modelValue.value = false;
}
</script>

<template>
  <UModal v-model:open="modelValue">
    <template #header>
      <h3 class="text-lg font-semibold">
        {{ title }}
      </h3>
    </template>

    <div class="py-4">
      <p class="text-sm text-muted">
        {{ description }}
      </p>
    </div>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" @click="cancel">
          Cancel
        </UButton>

        <UButton color="error" @click="confirm"> Delete </UButton>
      </div>
    </template>
  </UModal>
</template>
