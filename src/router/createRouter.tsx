import { FC, Suspense } from 'react';
import { Link as RLink, RouterProvider, createBrowserRouter } from 'react-router-dom';
import type { LinkProps, NavigateOptions, RouteObject } from 'react-router-dom';

export type ReactRouterLinkProps = LinkProps;
export type ReactRouterNavigateOptions = NavigateOptions;
export type ReactRouterDOMRouterOpts = Parameters<typeof createBrowserRouter>[1];

interface CreateReactRouterData {
  routes: RouteObject[];
  fallback?: React.ReactNode;
}

export const createReactRouter = (
  { routes, fallback }: CreateReactRouterData,
  opts?: ReactRouterDOMRouterOpts,
) => {
  const router = createBrowserRouter(routes, opts);
  const Router: FC = () => {
    return (
      <Suspense fallback={fallback}>
        <RouterProvider router={router} />
      </Suspense>
    );
  };
  const navigate = (to: string, opt?: ReactRouterNavigateOptions) => {
    return router.navigate(to, opt);
  };
  const back = (opt?: { to?: number }) => {
    const backTo = opt?.to || -1;
    return router.navigate(backTo);
  };
  const Link = (props: LinkProps) => {
    return <RLink {...props} />;
  };
  return {
    Router,
    navigate,
    back,
    Link,
  };
};
