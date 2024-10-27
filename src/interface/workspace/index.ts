import { getWorkspaceStore } from '@/models';

export const getCurrentWorkspace = () => {
  return getWorkspaceStore().currentWorkspace;
};

export const loadWorkspace = async (workspacePath: string) => {
  console.log('todo loadWorkspace: ', workspacePath);
};

export const createAndLoadWorkspace = async (workspaceName: string, workspacePath: string) => {
  console.log('todo createWorkspace', workspaceName, workspacePath);
};
