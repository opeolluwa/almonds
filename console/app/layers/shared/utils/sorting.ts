import type { Bookmark } from "@shared/stores/bookmarks";
import type { Note } from "@shared/stores/notes";
import type { Snippet } from "@shared/stores/snippets";
import type { Todo } from "@shared/stores/todo";

export interface SortOption<T extends string = string> {
  label: string;
  value: T;
  icon?: string;
}

export type NoteSort = "name-asc" | "name-desc" | "date-newest" | "date-oldest";
export type BookmarkSort = NoteSort;
export type SnippetSort = NoteSort;
export type TodoSort = NoteSort | "priority-high" | "priority-low";

export const NOTE_SORT_OPTIONS: SortOption<NoteSort>[][] = [
  [
    { label: "Name A–Z", value: "name-asc", icon: "heroicons:bars-arrow-up" },
    {
      label: "Name Z–A",
      value: "name-desc",
      icon: "heroicons:bars-arrow-down",
    },
  ],
  [
    {
      label: "Last modified (newest)",
      value: "date-newest",
      icon: "heroicons:clock",
    },
    {
      label: "Last modified (oldest)",
      value: "date-oldest",
      icon: "heroicons:clock",
    },
  ],
];

export const BOOKMARK_SORT_OPTIONS: SortOption<BookmarkSort>[][] = [
  [
    { label: "Name A–Z", value: "name-asc", icon: "heroicons:bars-arrow-up" },
    {
      label: "Name Z–A",
      value: "name-desc",
      icon: "heroicons:bars-arrow-down",
    },
  ],
  [
    { label: "Newest first", value: "date-newest", icon: "heroicons:clock" },
    { label: "Oldest first", value: "date-oldest", icon: "heroicons:clock" },
  ],
];

export const SNIPPET_SORT_OPTIONS: SortOption<SnippetSort>[][] = [
  [
    { label: "Name A–Z", value: "name-asc", icon: "heroicons:bars-arrow-up" },
    {
      label: "Name Z–A",
      value: "name-desc",
      icon: "heroicons:bars-arrow-down",
    },
  ],
  [
    { label: "Newest first", value: "date-newest", icon: "heroicons:clock" },
    { label: "Oldest first", value: "date-oldest", icon: "heroicons:clock" },
  ],
];

export const TODO_SORT_OPTIONS: SortOption<TodoSort>[][] = [
  [
    {
      label: "Priority (high → low)",
      value: "priority-high",
      icon: "heroicons:flag",
    },
    {
      label: "Priority (low → high)",
      value: "priority-low",
      icon: "heroicons:flag",
    },
  ],
  [
    { label: "Name A–Z", value: "name-asc", icon: "heroicons:bars-arrow-up" },
    {
      label: "Name Z–A",
      value: "name-desc",
      icon: "heroicons:bars-arrow-down",
    },
  ],
  [
    { label: "Newest first", value: "date-newest", icon: "heroicons:clock" },
    { label: "Oldest first", value: "date-oldest", icon: "heroicons:clock" },
  ],
];

const PRIORITY_ORDER: Record<string, number> = {
  high: 0,
  medium: 1,
  low: 2,
};

function compareByName(a: string | undefined, b: string | undefined) {
  return (a ?? "").localeCompare(b ?? "");
}

function compareByDate(
  a: string,
  b: string,
  field: "createdAt" | "updatedAt",
  dir: 1 | -1,
) {
  return dir * (new Date(a[field]).getTime() - new Date(b[field]).getTime());
}

export function sortNotes(list: Note[], sortBy: NoteSort): Note[] {
  return [...list].sort((a, b) => {
    switch (sortBy) {
      case "name-asc":
        return compareByName(a.title, b.title);
      case "name-desc":
        return compareByName(b.title, a.title);
      case "date-newest":
        return compareByDate(a, b, "updatedAt", -1);
      case "date-oldest":
        return compareByDate(a, b, "updatedAt", 1);
    }
  });
}

export function sortBookmarks(list: Bookmark[], sortBy: BookmarkSort): Bookmark[] {
  return [...list].sort((a, b) => {
    switch (sortBy) {
      case "name-asc":
        return compareByName(a.title, b.title);
      case "name-desc":
        return compareByName(b.title, a.title);
      case "date-newest":
        return compareByDate(a, b, "createdAt", -1);
      case "date-oldest":
        return compareByDate(a, b, "createdAt", 1);
    }
  });
}

export function sortSnippets(list: Snippet[], sortBy: SnippetSort): Snippet[] {
  return [...list].sort((a, b) => {
    switch (sortBy) {
      case "name-asc":
        return compareByName(a.title, b.title);
      case "name-desc":
        return compareByName(b.title, a.title);
      case "date-newest":
        return compareByDate(a, b, "createdAt", -1);
      case "date-oldest":
        return compareByDate(a, b, "createdAt", 1);
    }
  });
}

export function sortTodos(list: Todo[], sortBy: TodoSort): Todo[] {
  return [...list].sort((a, b) => {
    switch (sortBy) {
      case "priority-high":
        return (
          (PRIORITY_ORDER[a.priority] ?? 99) -
          (PRIORITY_ORDER[b.priority] ?? 99)
        );
      case "priority-low":
        return (
          (PRIORITY_ORDER[b.priority] ?? 99) -
          (PRIORITY_ORDER[a.priority] ?? 99)
        );
      case "name-asc":
        return compareByName(a.title, b.title);
      case "name-desc":
        return compareByName(b.title, a.title);
      case "date-newest":
        return compareByDate(a, b, "createdAt", -1);
      case "date-oldest":
        return compareByDate(a, b, "createdAt", 1);
    }
  });
}
