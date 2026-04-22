import {
	createRouteMask,
	createRouter,
	RouterProvider,
} from '@tanstack/solid-router';
import { render } from 'solid-js/web';
import './styles.css';
// Import the generated route tree
import { routeTree } from './routeTree.gen';

const settingsSubRoutes = ['/settings', '/settings/appearance'] as const;

const settingsMasks = settingsSubRoutes.map((path) =>
	createRouteMask({
		routeTree,
		from: path,
		to: '.',
		search: true,
	}),
);
export function getRouter() {
	// Create a new router instance
	const router = createRouter({ routeTree, routeMasks: [...settingsMasks] });
	return router;
}

// Register the router instance for type safety
declare module '@tanstack/solid-router' {
	interface Register {
		router: ReturnType<typeof getRouter>;
	}
}
