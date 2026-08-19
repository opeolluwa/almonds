import { useWorkspacesStore } from "@shared/stores/workspaces";
import type { RequestMeta } from "lunar";
export async function getWorkspaceMeta(): Promise<RequestMeta> {
  const workspaceStore = useWorkspacesStore();

  if (!workspaceStore.workspaces.length) {
    await workspaceStore.fetchWorkspaces();
  }

  const workspace = workspaceStore.currentWorkspace;

  if (!workspace) {
    throw new Error("No active workspace");
  }

  return {
    workspaceIdentifier: workspace.identifier,
  };
}
