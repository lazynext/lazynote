import { proxy, snapshot, useSnapshot } from 'valtio';

import { WorkspaceStore } from './types';

const workspaceStore = proxy<WorkspaceStore>({});

export const getWorkspaceStoreMut = () => {
  return workspaceStore;
};

export const getWorkspaceStore = () => {
  return snapshot(workspaceStore);
};

export const useWorkspaceStore = () => {
  return useSnapshot(workspaceStore);
};

export * from './types';
