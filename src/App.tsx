import { useState } from 'react';

import { initConfig } from './config';
import { useInited } from './hooks';
import { ThemeProvider } from './libs/theme';
import { initModels } from './models';
import { Router } from './router';

export const App = () => {
  const [inited, setInited] = useState(false);
  useInited(async () => {
    await initConfig();
    await initModels();
    setInited(true);
  });
  return <ThemeProvider>{inited && <Router />}</ThemeProvider>;
};
