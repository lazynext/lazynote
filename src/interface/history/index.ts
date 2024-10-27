import { getHistoryStore, getHistoryStoreMut, saveHistoryStore } from '@/models';
import { HistoryWorkspace } from '@/models';

export const getHistoryWorkspaceList = (): readonly HistoryWorkspace[] | undefined => {
  return getHistoryStore().workspaceHistoryList;
};

export const addHistoryWorkspace = async (workspace: HistoryWorkspace) => {
  const store = getHistoryStoreMut();
  if (!store.workspaceHistoryList) {
    store.workspaceHistoryList = [];
  }
  store.workspaceHistoryList.push(workspace);
  await saveHistoryStore();
};

export const getLastHistoryWorkspace = (): HistoryWorkspace | undefined => {
  return getHistoryStore().lastWorkspace;
};

export const setLastHistoryWorkspace = async (workspace: HistoryWorkspace) => {
  const store = getHistoryStoreMut();
  store.lastWorkspace = workspace;
  await saveHistoryStore();
};
