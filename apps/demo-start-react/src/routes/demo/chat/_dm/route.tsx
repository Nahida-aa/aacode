import { createFileRoute, Outlet } from '@tanstack/react-router';

export const Route = createFileRoute('/demo/chat/_dm')({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div className="min-w-0 grid grid-cols-[17rem_1fr] min-h-0">
			Hello "/demo/chat/dm"!
			<div className="flex flex-col gap-2">
				<Outlet />
			</div>
		</div>
	);
}
