import { TanStackDevtools } from '@tanstack/solid-devtools';
import { createRootRoute, Link, Outlet } from '@tanstack/solid-router';
import {
	TanStackRouterDevtools,
	TanStackRouterDevtoolsPanel,
} from '@tanstack/solid-router-devtools';
import { AppSidebar } from '#/components/app/AppSidebar.tsx';
import { ThemeProvider } from '#/components/app/theme-provider.tsx';
import { SidebarProvider } from '#/components/ui/sidebar.tsx';

const RootLayout = () => {
	return (
		<div class="h-svh">
			<ThemeProvider>
				<SidebarProvider>
					<AppSidebar />
					<main class="flex-1">
						<header class="p-2 flex gap-2 border-b ">
							<Link to="/" class="[&.active]:font-bold">
								Home
							</Link>{' '}
							<Link to="/about" class="[&.active]:font-bold">
								About
							</Link>
						</header>

						<Outlet />
					</main>
				</SidebarProvider>
			</ThemeProvider>
			<TanStackDevtools
				config={{
					position: 'bottom-right',
				}}
				plugins={[
					{
						name: 'TanStack Router',
						render: <TanStackRouterDevtoolsPanel />,
					},
				]}
			/>
		</div>
	);
};

export const Route = createRootRoute({ component: RootLayout });
