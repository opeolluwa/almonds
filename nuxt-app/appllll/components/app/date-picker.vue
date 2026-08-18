<script setup lang="ts">
import { CalendarDate } from "@internationalized/date";

const props = defineProps<{
  modelValue: Date | null;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: Date | null];
}>();

const calendarValue = computed({
  get() {
    if (!props.modelValue) return undefined;
    return new CalendarDate(
      props.modelValue.getFullYear(),
      props.modelValue.getMonth() + 1,
      props.modelValue.getDate(),
    );
  },
  set(val) {
    emit(
      "update:modelValue",
      val ? new Date(val.year, val.month - 1, val.day) : null,
    );
  },
});
</script>

<template>
  <UCalendar v-model="calendarValue" class="p-2" />
</template>
