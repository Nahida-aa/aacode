// src/server.ts

import handler, { createServerEntry } from '@tanstack/react-start/server-entry';
import { paraglideMiddleware } from './paraglide/server.js';

export default createServerEntry({
	fetch(req) {
		try {
			console.log('Received request');
			return paraglideMiddleware(req, () => handler.fetch(req));
		} catch (error) {
			console.warn('Error in createServerEntry:', error);
			return Response.json({ error: JSON.stringify(error) }, { status: 500 });
		}
	},
});
