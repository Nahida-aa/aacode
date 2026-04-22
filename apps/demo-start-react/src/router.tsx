import { QueryClient } from '@tanstack/react-query';
import { createRouter as createTanStackRouter } from '@tanstack/react-router';
import { setupRouterSsrQueryIntegration } from '@tanstack/react-router-ssr-query';
import { ErrorCard } from '#/components/app/error.tsx';
import { NotFound } from '#/components/app/NotFound.tsx';

import { routeTree } from './routeTree.gen';

export function getRouter() {
	const queryClient = new QueryClient();
	const router = createTanStackRouter({
		routeTree,
		context: { queryClient },
		scrollRestoration: true,
		defaultPreload: 'intent',
		defaultPreloadStaleTime: 0,
		defaultErrorComponent: ErrorCard,
		defaultNotFoundComponent: ({ isNotFound, routeId, data }) => <NotFound />,
	});

	setupRouterSsrQueryIntegration({ router, queryClient });

	return router;
}

declare module '@tanstack/react-router' {
	interface Register {
		router: ReturnType<typeof getRouter>;
	}
}
