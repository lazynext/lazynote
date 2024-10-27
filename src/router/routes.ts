import { lazy } from 'react';
import type { RouteObject } from 'react-router-dom';

import Fallback from '@/pages/components/Fallback';

const Layout = lazy(() => import('@/pages/components/Layout'));
const Index = lazy(() => import('@/pages/index'));
// const Index = lazy(() => import('@/pages/test/index'));

export const routes: RouteObject[] = [
  {
    path: '/',
    Component: Layout,
    children: [
      {
        index: true,
        Component: Index,
      },
    ],
  },
];

export { Fallback };
