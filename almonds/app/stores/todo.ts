import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface Todo {
  identifier: string;
  title: string;
  description: string | null;
  dueDate: string | null;
  priority: "high" | "medium" | "low";
  done: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateTodoPayload {
  title: string;
  description?: string;
  dueDate?: string;
  priority: "high" | "medium" | "low";
}

export interface UpdateTodoPayload {
  title?: string;
  description?: string;
}

export const useTodoStore = defineStore("todo_store", {
  state: () => ({
    todos: [] as Todo[],
    loading: false,
  }),

  getters: {
    activeTodos: (state) => state.todos.filter((t) => !t.done),
    completedTodos: (state) => state.todos.filter((t) => t.done),
    highPriorityCount: (state) =>
      state.todos.filter((t) => t.priority === "high").length,
    mediumPriorityCount: (state) =>
      state.todos.filter((t) => t.priority === "medium").length,
    lowPriorityCount: (state) =>
      state.todos.filter((t) => t.priority === "low").length,
  },

  actions: {
    async fetchTodos() {
      this.loading = true;
      try {
        this.todos = await invoke<Todo[]>("get_all_todos");
      } finally {
        this.loading = false;
      }
    },

    async createTodo(payload: CreateTodoPayload): Promise<Todo> {
      const created = await invoke<Todo>("create_todo", { todo: payload });
      this.todos.unshift(created);
      return created;
    },

    async updateTodo(
      identifier: string,
      payload: UpdateTodoPayload,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("update_todo", {
        identifier,
        todo: payload,
      });
      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async toggleDone(identifier: string, done: boolean): Promise<Todo> {
      const updated = await invoke<Todo>("mark_todo_done", {
        identifier,
        done,
      });
      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async changePriority(identifier: string, priority: string): Promise<Todo> {
      const updated = await invoke<Todo>("change_todo_priority", {
        identifier,
        priority,
      });
      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async updateDueDate(
      identifier: string,
      dueDate: string | null,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("update_todo_due_date", {
        identifier,
        dueDate,
      });
      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async deleteTodo(identifier: string) {
      await invoke("delete_todo", { identifier });
      this.todos = this.todos.filter((t) => t.identifier !== identifier);
    },
  },
});
