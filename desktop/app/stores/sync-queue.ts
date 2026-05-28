import { ref } from "vue";
import { useNetwork } from "@vueuse/core";
import { defineStore } from "pinia";
import { useBookmarkStore } from "~/stores/bookmarks";
import { useNoteStore } from "~/stores/notes";
import { useTodoStore } from "~/stores/todo";
import { useWorkspacesStore } from "~/stores/workspaces";
import { useReminderStore } from "~/stores/reminder";
import { useUserPreferenceStore } from "~/stores/workspace-preferences";
import { useSnippetStore } from "~/stores/snippets";
import { useRecycleBinStore } from "~/stores/recycle-bin";
import { useNotificationStore } from "~/stores/notifications";
import { gql } from "@apollo/client";

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
            notificationType: "BackupSuccess",
          });
        });
      await Promise.all([
        useBookmarkStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Bookmark Sync Successful",
            body: "Your bookmark data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useNoteStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Note Sync Successful",
            body: "Your note data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useTodoStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Todo Sync Successful",
            body: "Your todo data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useReminderStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Reminder Sync Successful",
            body: "Your reminder data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useUserPreferenceStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "User Preference Sync Successful",
            body: "Your user preference data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useSnippetStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Snippet Sync Successful",
            body: "Your snippet data has been synced successfully.",
            notificationType: "BackupSuccess",
          });
        }),
        useRecycleBinStore().syncUpstream().then(async () => {
          await useNotificationStore().createNotification({
            title: "Recycle Bin Sync Successful",
            body: "Your recycle bin data has been synced successfully.",
            notificationType: "BackupSuccess",
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
