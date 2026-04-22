import { createFileRoute } from '@tanstack/react-router';
import { prepareElectricUrl, proxyElectricRequest } from '#/integrations/electric/proxy.ts';


const serve = async ({ request }: { request: Request }) => {
const originUrl = prepareElectricUrl(request.url)

	// set shape parameters
	// full spec: https://github.com/electric-sql/electric/blob/main/website/electric-api.yaml
	originUrl.searchParams.set('table', 'todo');
	// Where clause to filter rows in the table (optional).
	// originUrl.searchParams.set("where", "completed = true")

	// Select the columns to sync (optional)
	// originUrl.searchParams.set("columns", "id,text,completed")

	return proxyElectricRequest(originUrl)
};

export const Route = createFileRoute('/api/todo')({
	server: {
		handlers: {
			GET: serve,
		},
	},
});
