import { gql } from "@apollo/client";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useNoteStore } from "@shared/stores/notes";
import { useNotificationStore } from "@shared/stores/notifications";
import { useRecycleBinStore } from "@shared/stores/recycle-bin";
import { useReminderStore } from "@shared/stores/reminder";
import { useSnippetStore } from "@shared/stores/snippets";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-preferences";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import { useNetwork } from "@vueuse/core";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useSyncQueueStore = defineStore("sync_queue_store", () => {
  const { isOnline } = useNetwork();
  const runningSync = ref(false);

  async function preflightCheck(name: string) {
    const query = gql`
      mutation PreflightCheck($name: String!) {
        preflight(name: $name)
      }
    `;

    const variables = { name };

    const { mutate } = useMutation(query, { variables });
    const data = await mutate();
    console.log("Preflight check response:", data);
  }

  async function runSync() {
    if (runningSync.value || !isOnline.value) return;
    runningSync.value = true;
    try {
      await useWorkspacesStore()
        .syncUpstream()
        .then(async () => {
          await useNotificationStore().createNotification({
            title: "Workspace Sync Successful",
            body: "Your data has been synced successfully.",
            notificationType: "backup_success",
          });
        });
      await Promise.all([
        useBookmarkStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Bookmark Sync Successful",
              body: "Your bookmark data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useNoteStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Note Sync Successful",
              body: "Your note data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useTodoStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Todo Sync Successful",
              body: "Your todo data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useReminderStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Reminder Sync Successful",
              body: "Your reminder data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useUserPreferenceStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "User Preference Sync Successful",
              body: "Your user preference data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useSnippetStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Snippet Sync Successful",
              body: "Your snippet data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
        useRecycleBinStore()
          .syncUpstream()
          .then(async () => {
            await useNotificationStore().createNotification({
              title: "Recycle Bin Sync Successful",
              body: "Your recycle bin data has been synced successfully.",
              notificationType: "backup_success",
            });
          }),
      ]);
    } catch (err) {
      console.error("Error during sync:", err);
    } finally {
      runningSync.value = false;
    }
  }

  return { isOnline, runningSync, preflightCheck, runSync };
});
