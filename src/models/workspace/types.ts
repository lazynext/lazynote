export interface Workspace {
  name: string;
  path: string;
}

export interface WorkspaceStore {
  currentWorkspace?: Workspace;
}
