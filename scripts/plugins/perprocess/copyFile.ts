import path from 'path';

import { fastcopy } from '../utils';

export interface CopyFileItem {
  from: string;
  to: string;
}

export const defaultCopyFiles: CopyFileItem[] = [];

export const executeCopyFiles = async (context: any, copyFiles: CopyFileItem[] | undefined) => {
  const projectPath = process.cwd();
  copyFiles = copyFiles || defaultCopyFiles;
  for (let i = 0; i < copyFiles.length; i++) {
    const file = copyFiles[i];
    if (file == null) continue;
    const from = path.join(projectPath, file.from);
    const to = path.join(projectPath, file.to);
    try {
      await fastcopy(from, to);
    } catch (e: any) {
      context.warn(`copy file error: ${e.message}`);
    }
  }
  // 第一次拷贝启动过快可能会加载不到文件
  await new Promise((resolve) => setTimeout(resolve, 300));
};
