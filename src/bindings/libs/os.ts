import { invokePlugin } from '../invoke';

export type Platform =
  | 'windows'
  | 'macos'
  | 'linux'
  | 'android'
  | 'ios'
  | 'freebsd'
  | 'dragonfly'
  | 'netbsd'
  | 'openbsd'
  | 'solaris';

export type OSType = 'windows' | 'macos' | 'linux' | 'android' | 'ios';

export type FamilyType = 'windows' | 'unix';

const name = 'os';

export const os = {
  async arch(): Promise<string> {
    return (await invokePlugin(name, 'arch'))!;
  },
  async family(): Promise<FamilyType> {
    return (await invokePlugin(name, 'family'))!;
  },
  async hostname(): Promise<string> {
    return (await invokePlugin(name, 'hostname'))!;
  },
  async locale(): Promise<string> {
    return (await invokePlugin(name, 'locale'))!;
  },
  async osType(): Promise<OSType> {
    return (await invokePlugin(name, 'os_type'))!;
  },
  async platform(): Promise<Platform> {
    return (await invokePlugin(name, 'platform'))!;
  },
  async version(): Promise<string> {
    return (await invokePlugin(name, 'version'))!;
  },
};
