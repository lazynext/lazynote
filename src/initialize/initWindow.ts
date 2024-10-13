import { getCurrentWindow } from '@tauri-apps/api/window';
import { attachConsole } from '@tauri-apps/plugin-log';

import { appConfig, initOsConfig, os } from '@/config';

const win = getCurrentWindow();

const initWindow = async () => {
  await initOsConfig();
  const desktop = os.desktop;
  if (desktop) {
    await win.show();
  }
  if (appConfig.isDev) {
    // 将rust侧的日志附加到webview的console
    const _ = await attachConsole();
  }
};

initWindow();

export { win };
