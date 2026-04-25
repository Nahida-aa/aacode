import { boolean, json, jsonb, pgTable, text } from 'drizzle-orm/pg-core';
import { nanoidWithTimestamps } from '#/db.helpers.ts';

export const todo = pgTable('todo', {
	...nanoidWithTimestamps,
	title: text(),
	content: jsonb().$type<any>(),
	completed: boolean().default(false).notNull(),
});
