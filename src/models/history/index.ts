import { proxy, snapshot, useSnapshot } from 'valtio';

import { store } from '@/bindings';

import { HistoryStore } from './types';

const saveKey = 'history';

let historyStore: HistoryStore | undefined;

const defaultHistory: HistoryStore = {};

export const initHistoryStore = async () => {
  const initHistory: HistoryStore = (await store.common.get(saveKey)) || defaultHistory;
  historyStore = proxy<HistoryStore>(initHistory);
};

export const getHistoryStoreMut = () => {
  return historyStore!;
};

export const getHistoryStore = () => {
  return snapshot(historyStore || {});
};

export const useHistoryStore = () => {
  return useSnapshot(historyStore || {});
};

export const saveHistoryStore = async () => {
  await store.common.set(saveKey, historyStore || {});
};

export * from './types';
