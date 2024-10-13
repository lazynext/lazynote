import { Option, Select } from '@mui/joy';

export const LanguageSelect = (props: { width?: string | number }) => {
  return (
    <Select sx={{ width: props.width || '100%' }} value="简体中文" onChange={() => {}}>
      <Option key="1" value="简体中文">
        简体中文
      </Option>
    </Select>
  );
};
