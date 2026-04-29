import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "villus";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

export interface Todo {
  identifier: string;
  title: string;
  description: string | null;
  dueDate: string | null;
  time: string | null;
  priority: "high" | "medium" | "low";
  done: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateTodoPayload {
  title: string;
  description?: string;
  dueDate?: string;
  time?: string;
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
        this.todos = await invoke<Todo[]>("get_all_todos", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async createTodo(payload: CreateTodoPayload): Promise<Todo> {
      const created = await invoke<Todo>("create_todo", {
        todo: payload,
        meta: await getWorkspaceMeta(),
      });

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
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async toggleDone(identifier: string, done: boolean): Promise<Todo> {
      const updated = await invoke<Todo>("mark_todo_done", {
        identifier,
        done,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async changePriority(
      identifier: string,
      priority: "high" | "medium" | "low",
    ): Promise<Todo> {
      const updated = await invoke<Todo>("change_todo_priority", {
        identifier,
        priority,
        meta: await getWorkspaceMeta(),
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
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async deleteTodo(identifier: string) {
      await invoke("delete_todo", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.todos = this.todos.filter((t) => t.identifier !== identifier);
    },

    async duplicateTodo(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("duplicate_todo", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      this.todos.push(updated);
      return updated;
    },

    async transferTodo(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("transfer_todo", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async fetchUnsynced() {
      const todo = await invoke<Todo[]>("get_unsynced_todos");
      return todo;
    },

    async syncUpstream() {
      const todo = await this.fetchUnsynced();
      if (!todo.length) return;

      const { data, execute } = useMutation(`
        mutation SyncTodos($input: [SyncTodoInput!]!) {
          sync_todo(input: $input) { success error_message identifier }
        }
      `);
      await execute({ input: todo });

      const synced = data.value?.sync_todo
        .filter((r: SyncResult) => r.success)
        .map((r: SyncResult) => r.identifier);
      if (synced?.length)
        await invoke("clear_synced_todos", { identifiers: synced });
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_todos", { identifiers });
    },
  },
});
