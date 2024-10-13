import { initOsConfig } from './os';

export * from './os';
export * from './appConfig';

export const initConfig = async () => {
  await initOsConfig();
};
