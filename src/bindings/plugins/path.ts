import { invokePlugin } from '../invoke';

export const appPathKeys = [
  'app_cache_dir',
  'app_config_dir',
  'app_data_dir',
  'app_local_data_dir',
  'app_log_dir',
  'audio_dir',
  'cache_dir',
  'config_dir',
  'data_dir',
  'desktop_dir',
  'document_dir',
  'download_dir',
  'executable_dir',
  'font_dir',
  'home_dir',
  'local_data_dir',
  'picture_dir',
  'public_dir',
  'resource_dir',
  'runtime_dir',
  'template_dir',
  'data_path',
  'local_res_path',
  'local_data_path',
] as const;

export type AppPathKey = (typeof appPathKeys)[number];

const name = 'path';

const getPath = async (key: AppPathKey): Promise<string> => {
  return (await invokePlugin(name, key))!;
};

export const path = {
  getPath,
  getAllPath: async (): Promise<{ [key in AppPathKey]: string }> => {
    const pathsArray = await Promise.all(
      appPathKeys.map(async (key) => {
        const path = await getPath(key);
        return { [key]: path };
      }),
    );
    return pathsArray.reduce(
      (acc, curr) => ({ ...acc, ...curr }),
      {} as { [key in AppPathKey]: string },
    ) as { [key in AppPathKey]: string };
  },
  appCacheDir: async () => await getPath('app_cache_dir'),
  appConfigDir: async () => await getPath('app_config_dir'),
  appDataDir: async () => await getPath('app_data_dir'),
  appLocalDataDir: async () => await getPath('app_local_data_dir'),
  appLogDir: async () => await getPath('app_log_dir'),
  audioDir: async () => await getPath('audio_dir'),
  cacheDir: async () => await getPath('cache_dir'),
  configDir: async () => await getPath('config_dir'),
  dataDir: async () => await getPath('data_dir'),
  desktopDir: async () => await getPath('desktop_dir'),
  documentDir: async () => await getPath('document_dir'),
  downloadDir: async () => await getPath('download_dir'),
  executableDir: async () => await getPath('executable_dir'),
  fontDir: async () => await getPath('font_dir'),
  homeDir: async () => await getPath('home_dir'),
  localDataDir: async () => await getPath('local_data_dir'),
  pictureDir: async () => await getPath('picture_dir'),
  publicDir: async () => await getPath('public_dir'),
  resourceDir: async () => await getPath('resource_dir'),
  runtimeDir: async () => await getPath('runtime_dir'),
  templateDir: async () => await getPath('template_dir'),
  dataPath: async () => await getPath('data_path'),
  localResPath: async () => await getPath('local_res_path'),
  localDataPath: async () => await getPath('local_data_path'),
};
