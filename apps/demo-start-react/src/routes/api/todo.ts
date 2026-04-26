import { createFileRoute } from '@tanstack/react-router';
import { createMiddleware } from '@tanstack/react-start';
import {
	prepareElectricUrl,
	proxyElectricRequest,
} from '#/integrations/electric/proxy.ts';
import { auth } from '#/lib/auth.ts';

const authMiddleware = createMiddleware().server(async ({ request, next }) => {
	const session = await auth.api.getSession({ headers: request.headers });
	if (!session) {
		return new Response(JSON.stringify({ error: `Unauthorized` }), {
			status: 401,
			headers: { 'content-type': `application/json` },
		});
	}
	return next({
		context: { user: session.user },
	});
});

export const Route = createFileRoute('/api/todo')({
	server: {
		middleware: [authMiddleware],
		handlers: {
			GET: async ({ request, context }) => {
				const originUrl = prepareElectricUrl(request.url);
				// set shape parameters
				// full spec: https://github.com/electric-sql/electric/blob/main/website/electric-api.yaml
				originUrl.searchParams.set('table', 'todo');
				// Where clause to filter rows in the table (optional).
				const filter = `user_id = '${context.user.id}'`;
				originUrl.searchParams.set('where', filter);
				// Select the columns to sync (optional)
				// originUrl.searchParams.set("columns", "id,text,completed")

				return proxyElectricRequest(originUrl);
			},
		},
	},
});
