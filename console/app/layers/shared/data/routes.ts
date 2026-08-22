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
    icon: "ri:home-line",
    activeIcon: "ri:home-fill",
  },
  {
    path: "/notes",
    name: "Notes",
    icon: "ri:booklet-line",
    activeIcon: "ri:booklet-fill",
  },

  //     {
  //     path: "/calendar",
  //     name: "Calendar",
  //     icon: "ri:calendar-line",
  //     activeIcon: "ri:calendar-fill",
  //   },
  {
    path: "/bookmarks",
    name: "Bookmarks",
    icon: "ri:bookmark-line",
    activeIcon: "ri:bookmark-fill",
  },

  {
    path: "/reminders",
    name: "Reminders",
    icon: "ri:alarm-line",
    activeIcon: "ri:alarm-fill",
  },

  // {
  //   path: "/ollama",
  //   name: "Ollama",
  //   icon: "ri:cpu-line",
  //   activeIcon: "ri:cpu-fill",
  // },
  {
    path: "/snippets",
    name: "Snippets",
    icon: "ri:code-s-slash-line",
    activeIcon: "ri:code-s-slash-fill",
  },
  {
    path: "/todo",
    name: "Todo",
    icon: "ri:calendar-todo-line",
    activeIcon: "ri:calendar-todo-fill",
  },
  {
    path: "/moodboard",
    name: "Moodboard",
    icon: "ri:layout-grid-line",
    activeIcon: "ri:layout-grid-fill",
  },
  {
    path: "/scratch-pad",
    name: "Scratch Pad",
    icon: "ri:pencil-line",
    activeIcon: "ri:pencil-fill",
  },
];

export const secondaryRoutes: Route[] = [
  {
    path: "/recycle-bin",
    name: "Recycle Bin",
    icon: "ri:delete-bin-line",
    activeIcon: "ri:delete-bin-fill",
  },
  {
    path: "/settings",
    name: "Settings",
    icon: "ri:settings-3-line",
    activeIcon: "ri:settings-3-fill",
  },
];

export const mobileBottomNavRoutes: Route[] = [
  {
    path: "/",
    name: "Home",
    icon: "ri:home-line",
    activeIcon: "ri:home-fill",
  },
  {
    path: "/notes",
    name: "Notes",
    icon: "ri:booklet-line",
    activeIcon: "ri:booklet-fill",
  },
  {
    path: "/bookmarks",
    name: "Bookmarks",
    icon: "ri:bookmark-line",
    activeIcon: "ri:bookmark-fill",
  },
  {
    path: "/todo",
    name: "Tasks",
    icon: "ri:calendar-todo-line",
    activeIcon: "ri:calendar-todo-fill",
  },
  {
    path: "/settings",
    name: "Account",
    icon: "ri:user-line",
    activeIcon: "ri:user-fill",
  },
];
