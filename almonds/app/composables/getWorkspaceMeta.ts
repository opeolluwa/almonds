import { useWorkspacesStore } from "@/stores/workspaces";
import type { RequestMeta } from "~/adapters/request-meta";

export async function getWorkspaceMeta(): Promise<RequestMeta> {
  const workspaceStore = useWorkspacesStore();

  await workspaceStore.fetchWorkspaces();

  const workspace = workspaceStore.currentWorkspace;

  if (!workspace) {
    throw new Error("No active workspace");
  }

  return {
    workspaceIdentifier: workspace.identifier,
  };
}