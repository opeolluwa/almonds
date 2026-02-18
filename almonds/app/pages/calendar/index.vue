<script setup lang="ts">
definePageMeta({ layout: false });

interface CalendarEvent {
  id: number;
  title: string;
  date: string; // YYYY-MM-DD
  color: "accent" | "emerald" | "rose" | "amber";
}

const today = new Date();

function formatDate(year: number, month: number, day: number): string {
  const d = new Date(year, month, day);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

const todayStr = formatDate(today.getFullYear(), today.getMonth(), today.getDate());

const currentYear = ref(today.getFullYear());
const currentMonth = ref(today.getMonth());
const selectedDate = ref<string>(todayStr);

const events = ref<CalendarEvent[]>([
  { id: 1, title: "Team standup", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate()), color: "accent" },
  { id: 2, title: "Design review", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate() + 2), color: "emerald" },
  { id: 3, title: "Sprint planning", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate() + 5), color: "rose" },
  { id: 4, title: "1:1 with manager", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate() + 7), color: "amber" },
  { id: 5, title: "Product demo", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate() + 10), color: "accent" },
  { id: 6, title: "Retrospective", date: formatDate(today.getFullYear(), today.getMonth(), today.getDate() + 14), color: "emerald" },
]);

const monthNames = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];

const dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const calendarDays = computed(() => {
  const year = currentYear.value;
  const month = currentMonth.value;
  const firstDay = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const daysInPrevMonth = new Date(year, month, 0).getDate();

  const days: { date: string; day: number; isCurrentMonth: boolean }[] = [];

  for (let i = firstDay - 1; i >= 0; i--) {
    const d = daysInPrevMonth - i;
    const prevMonth = month === 0 ? 11 : month - 1;
    const prevYear = month === 0 ? year - 1 : year;
    days.push({ date: formatDate(prevYear, prevMonth, d), day: d, isCurrentMonth: false });
  }

  for (let d = 1; d <= daysInMonth; d++) {
    days.push({ date: formatDate(year, month, d), day: d, isCurrentMonth: true });
  }

  const remaining = 42 - days.length;
  for (let d = 1; d <= remaining; d++) {
    const nextMonth = month === 11 ? 0 : month + 1;
    const nextYear = month === 11 ? year + 1 : year;
    days.push({ date: formatDate(nextYear, nextMonth, d), day: d, isCurrentMonth: false });
  }

  return days;
});

function eventsOnDate(date: string) {
  return events.value.filter((e) => e.date === date);
}

function prevMonth() {
  if (currentMonth.value === 0) { currentMonth.value = 11; currentYear.value--; }
  else currentMonth.value--;
}

function nextMonth() {
  if (currentMonth.value === 11) { currentMonth.value = 0; currentYear.value++; }
  else currentMonth.value++;
}

function goToToday() {
  currentYear.value = today.getFullYear();
  currentMonth.value = today.getMonth();
  selectedDate.value = todayStr;
}

const selectedDateEvents = computed(() => eventsOnDate(selectedDate.value));

const upcomingEvents = computed(() =>
  [...events.value]
    .filter((e) => e.date >= todayStr)
    .sort((a, b) => a.date.localeCompare(b.date))
    .slice(0, 5),
);

const eventColorClass: Record<string, string> = {
  accent: "bg-accent-100 dark:bg-accent-900 text-accent-700 dark:text-accent-300",
  emerald: "bg-emerald-100 dark:bg-emerald-900 text-emerald-700 dark:text-emerald-300",
  rose: "bg-rose-100 dark:bg-rose-900 text-rose-700 dark:text-rose-300",
  amber: "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-300",
};

const eventDotClass: Record<string, string> = {
  accent: "bg-accent-500",
  emerald: "bg-emerald-500",
  rose: "bg-rose-500",
  amber: "bg-amber-500",
};

function formatDisplayDate(dateStr: string): string {
  const [year, month, day] = dateStr.split("-").map(Number);
  return new Date(year, month - 1, day).toLocaleDateString("en-US", {
    month: "long", day: "numeric", year: "numeric",
  });
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Month navigation -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2">
          <button
            class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            @click="prevMonth"
          >
            <UIcon name="heroicons:chevron-left" class="size-4 text-gray-500" />
          </button>
          <h2 class="text-base font-semibold text-gray-800 dark:text-gray-100 w-40 text-center">
            {{ monthNames[currentMonth] }} {{ currentYear }}
          </h2>
          <button
            class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            @click="nextMonth"
          >
            <UIcon name="heroicons:chevron-right" class="size-4 text-gray-500" />
          </button>
        </div>
        <button
          class="px-3 py-1.5 text-xs font-medium text-accent-600 dark:text-accent-400 border border-accent-200 dark:border-accent-800 rounded-lg hover:bg-accent-50 dark:hover:bg-accent-950 transition-colors"
          @click="goToToday"
        >
          Today
        </button>
      </div>

      <!-- Day headers -->
      <div class="grid grid-cols-7 mb-1">
        <div
          v-for="day in dayNames"
          :key="day"
          class="text-center text-xs font-medium text-gray-400 dark:text-gray-500 py-2"
        >
          {{ day }}
        </div>
      </div>

      <!-- Calendar grid -->
      <div class="grid grid-cols-7 gap-px bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden border border-gray-100 dark:border-gray-800">
        <button
          v-for="cell in calendarDays"
          :key="cell.date"
          class="relative bg-white dark:bg-gray-900 min-h-20 p-2 text-left transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/80"
          :class="{ 'opacity-40': !cell.isCurrentMonth }"
          @click="selectedDate = cell.date"
        >
          <span
            class="inline-flex items-center justify-center size-6 rounded-full text-sm mb-1 transition-colors"
            :class="[
              cell.date === todayStr
                ? 'bg-accent-500 text-white font-semibold'
                : selectedDate === cell.date
                  ? 'bg-gray-100 dark:bg-gray-700 font-medium text-gray-800 dark:text-gray-100'
                  : 'text-gray-600 dark:text-gray-400',
            ]"
          >
            {{ cell.day }}
          </span>
          <div class="flex flex-col gap-0.5">
            <div
              v-for="ev in eventsOnDate(cell.date).slice(0, 2)"
              :key="ev.id"
              class="text-xs px-1.5 py-0.5 rounded truncate"
              :class="eventColorClass[ev.color]"
            >
              {{ ev.title }}
            </div>
            <span
              v-if="eventsOnDate(cell.date).length > 2"
              class="text-xs text-gray-400 px-1"
            >
              +{{ eventsOnDate(cell.date).length - 2 }} more
            </span>
          </div>
        </button>
      </div>
    </template>

    <template #side_content>
      <!-- Selected day -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        {{ formatDisplayDate(selectedDate) }}
      </h2>
      <div v-if="selectedDateEvents.length" class="flex flex-col gap-2 mb-6">
        <div
          v-for="ev in selectedDateEvents"
          :key="ev.id"
          class="flex items-center gap-2.5 p-2.5 rounded-lg bg-gray-50 dark:bg-gray-800"
        >
          <div class="size-2 rounded-full shrink-0" :class="eventDotClass[ev.color]" />
          <span class="text-sm text-gray-700 dark:text-gray-300">{{ ev.title }}</span>
        </div>
      </div>
      <p v-else class="text-xs text-gray-400 mb-6">
        No events scheduled
      </p>

      <USeparator class="my-4" />

      <!-- Upcoming events -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Upcoming
      </h2>
      <div class="flex flex-col gap-1">
        <div
          v-for="ev in upcomingEvents"
          :key="ev.id"
          class="flex items-start gap-3 py-2.5 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
          @click="selectedDate = ev.date"
        >
          <div class="size-2 rounded-full mt-1.5 shrink-0" :class="eventDotClass[ev.color]" />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
              {{ ev.title }}
            </p>
            <p class="text-xs text-gray-400">{{ formatDisplayDate(ev.date) }}</p>
          </div>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
