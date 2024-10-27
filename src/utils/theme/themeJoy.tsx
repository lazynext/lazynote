import { CssBaseline } from '@mui/joy';
import InitColorSchemeScript from '@mui/joy/InitColorSchemeScript';
import { CssVarsProvider, extendTheme, useColorScheme } from '@mui/joy/styles';

export const colorModeList = ['light', 'dark', 'system'] as const;

export type ColorMode = (typeof colorModeList)[number];
export type CurrentColorMode = 'light' | 'dark';

export type ThemeProviderProps = {
  children: React.ReactNode;
};

const primary = {
  '50': '#fdf2f8',
  '100': '#fce7f3',
  '200': '#fbcfe8',
  '300': '#f9a8d4',
  '400': '#f472b6',
  '500': '#ec4899',
  '600': '#db2777',
  '700': '#be185d',
  '800': '#9d174d',
  '900': '#831843',
};

const theme = extendTheme({
  colorSchemes: {
    light: {
      palette: {
        primary,
      },
    },
    dark: {
      palette: {
        primary,
      },
    },
  },
});

export function ThemeProvider(props: ThemeProviderProps) {
  return (
    <>
      <InitColorSchemeScript />
      <CssVarsProvider theme={theme}>
        <CssBaseline />
        {props.children}
      </CssVarsProvider>
    </>
  );
}

export type ThemeProviderState = {
  currentColorMode: CurrentColorMode;
  colorMode: ColorMode;
  setColorMode: (colorMode: ColorMode) => void;
};

export const useColorMode = (): ThemeProviderState => {
  const { mode, setMode, systemMode } = useColorScheme();
  const colorMode = mode || 'system';
  const currentMode = (colorMode === 'system' ? systemMode : colorMode) || 'light';
  return {
    colorMode,
    currentColorMode: currentMode,
    setColorMode(colorMode) {
      setMode(colorMode);
    },
  };
};
