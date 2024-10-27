import { initHistoryStore } from './history';

export * from './history';
export * from './workspace';

export const initModels = async () => {
  await initHistoryStore();
};
