import { useState } from 'react';

import { useInited } from './hooks';
import { initModels } from './models';
import { Router } from './router';
import { ThemeProvider } from './utils/theme';

export const App = () => {
  const [inited, setInited] = useState(false);
  useInited(async () => {
    await initModels();
    setInited(true);
  });
  return <ThemeProvider>{inited && <Router />}</ThemeProvider>;
};
