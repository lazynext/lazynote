export interface HistoryWorkspace {
  workspaceName: string;
  workspacePath: string;
}

export interface HistoryStore {
  workspaceHistoryList?: HistoryWorkspace[];
  lastWorkspace?: HistoryWorkspace;
}
