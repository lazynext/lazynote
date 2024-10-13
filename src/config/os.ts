import { FamilyType, OSType, Platform, os as _os } from '@/bindings';

interface OSConfig {
  arch?: string;
  family?: FamilyType;
  hostname?: string;
  locale?: string;
  osType?: OSType;
  platform?: Platform;
  version?: string;
  desktop?: boolean;
  mobile?: boolean;
  isInited?: boolean;
}

const os: OSConfig = {};

export const initOsConfig = async () => {
  if (os.isInited) return;
  os.arch = await _os.arch();
  os.family = await _os.family();
  os.hostname = await _os.hostname();
  os.locale = await _os.locale();
  os.osType = await _os.osType();
  os.platform = await _os.platform();
  os.version = await _os.version();
  os.desktop = os.osType === 'windows' || os.osType === 'macos' || os.osType === 'linux';
  os.mobile = os.osType === 'android' || os.osType === 'ios';
  os.isInited = true;
};

export { os };
