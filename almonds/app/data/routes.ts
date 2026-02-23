export interface Route {
  path: string;
  name: string;
  icon: string;
  activeIcon: string;
}

export const primaryRoutes: Route[] = [
  {
    path: "/",
    name: "Home",
    icon: "heroicons:home",
    activeIcon: "heroicons:home-solid",
  },
  {
    path: "/notes",
    name: "Notes",
    icon: "heroicons:document-text",
    activeIcon: "heroicons:document-text-solid",
  },

  //     {
  //     path: "/calendar",
  //     name: "Calendar",
  //     icon: "heroicons:calendar-days",
  //     activeIcon: "heroicons:calendar-days-solid",
  //   },
  {
    path: "/bookmarks",
    name: "Bookmarks",
    icon: "heroicons:bookmark",
    activeIcon: "heroicons:bookmark-solid",
  },

  {
    path: "/reminders",
    name: "Reminders",
    icon: "heroicons:clock",
    activeIcon: "heroicons:clock-solid",
  },

  {
    path: "/ollama",
    name: "Ollama",
    icon: "heroicons:cpu-chip",
    activeIcon: "heroicons:cpu-chip-solid",
  },
  {
    path: "/snippets",
    name: "Snippets",
    icon: "heroicons:code-bracket",
    activeIcon: "heroicons:code-bracket-solid",
  },
  {
    path: "/todo",
    name: "Todo",
    icon: "heroicons:check-circle",
    activeIcon: "heroicons:check-circle-solid",
  },
  {
    path: "/moodboard",
    name: "Moodboard",
    icon: "heroicons:squares-2x2",
    activeIcon: "heroicons:squares-2x2-solid",
  },
  {
    path: "/scratch-pad",
    name: "Scratch Pad",
    icon: "heroicons:pencil-square",
    activeIcon: "heroicons:pencil-square-solid",
  },
];

export const secondaryRoutes: Route[] = [
  {
    path: "/recycle-bin",
    name: "Recycle Bin",
    icon: "heroicons:trash",
    activeIcon: "heroicons:trash-solid",
  },
  {
    path: "/settings",
    name: "Settings",
    icon: "heroicons:cog-6-tooth",
    activeIcon: "heroicons:cog-6-tooth-solid",
  },
];
