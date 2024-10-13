import {
  InvokeArgs,
  InvokeOptions,
  invoke as _invoke,
  isTauri as _isTauri,
} from '@tauri-apps/api/core';

const invokeFunName = 'invoke';
const invokeAsyncFunName = 'invoke_async';

export const invoke = async <T>(
  cmd: string,
  args?: any,
  opt?: {
    raw?: boolean;
    async?: boolean;
    headers?: Record<string, string>;
  },
): Promise<T | undefined> => {
  const headers: Record<string, string> = {
    key: cmd,
    ...opt?.headers,
  };
  if (opt?.raw) {
    headers['raw'] = 'true';
  }
  const data = opt?.raw ? args : { args };
  const invokeKey = opt?.async ? invokeAsyncFunName : invokeFunName;
  return await _invoke(invokeKey, data, {
    headers,
  });
};

export const invokeAsync = async <T>(
  cmd: string,
  args?: any,
  opt?: {
    raw?: boolean;
    headers?: Record<string, string>;
  },
): Promise<T | undefined> => {
  return await invoke<T>(cmd, args, {
    async: true,
    ...opt,
  });
};

export const invokeCommand = async <T>(
  cmd: string,
  args?: any,
  options?: InvokeOptions,
): Promise<T | undefined> => {
  return await _invoke(cmd, args, options);
};

export const invokePlugin = async <T>(
  pluginName: string,
  funcName: string,
  args?: InvokeArgs,
  options?: InvokeOptions,
): Promise<T | undefined> => {
  return await _invoke(`plugin:${pluginName}|${funcName}`, args, options);
};

export const isTauri = _isTauri();
