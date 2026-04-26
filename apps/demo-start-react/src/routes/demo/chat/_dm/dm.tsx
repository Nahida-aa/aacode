import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/demo/chat/_dm/dm')({
	component: RouteComponent,
});

function RouteComponent() {
	// friend list
	return <div>Hello "/demo/chat/dm/"!</div>;
}
