import { Option, Select } from '@mui/joy';

import { ColorMode, colorModeList, useColorMode } from '@/utils/theme';

const colorModeI18n = {
  light: {
    zh_CN: '浅色',
  },
  dark: {
    zh_CN: '深色',
  },
  system: {
    zh_CN: '跟随系统',
  },
};

export const ThemeSelect = (props: { width?: string | number }) => {
  const { colorMode, setColorMode } = useColorMode();
  return (
    <Select
      sx={{ width: props.width || '100%' }}
      value={colorMode}
      onChange={(_, newValue) => {
        if (newValue) {
          setColorMode(newValue as ColorMode);
        }
      }}
    >
      {colorModeList.map((item, index) => (
        <Option key={index} value={item}>
          {colorModeI18n[item].zh_CN}
        </Option>
      ))}
    </Select>
  );
};
