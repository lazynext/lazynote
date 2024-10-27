import { FC } from 'react';
import { Outlet } from 'react-router-dom';

import { createReactRouter } from './createRouter';
import type { ReactRouterLinkProps, ReactRouterNavigateOptions } from './createRouter';
import { Fallback, routes } from './routes';

const router = createReactRouter({
  routes,
  fallback: <Fallback />,
});

export type RouteNavigateOptions = ReactRouterNavigateOptions;

export type LinkProps = {
  to?: string | number | undefined;
} & ReactRouterLinkProps;

export const Router: FC = () => {
  return <router.Router />;
};

export const Link = (props: LinkProps) => {
  return <router.Link {...(props as any)} />;
};

export const navigate = (to: string, opt?: RouteNavigateOptions) => {
  return router.navigate(to, opt);
};

export const back = (opt?: RouteNavigateOptions & { to: string | number | undefined }) => {
  return router.back(opt as any);
};

export { Outlet };
