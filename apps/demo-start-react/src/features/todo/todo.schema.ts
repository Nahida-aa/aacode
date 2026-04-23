import { createSelectSchema, createUpdateSchema } from 'drizzle-zod';
import type { z } from 'zod';
import { todo } from '#/features/todo/todo.table.ts';

export const selectTodoSchema = createSelectSchema(todo).partial({
	title: true,
	content: true,
});
export const updateTodoSchema = createUpdateSchema(todo).pick({
	title: true,
	content: true,
	completed: true,
});
export const addTodoSchema = selectTodoSchema
	.pick({
		id: true,
		title: true,
		content: true,
	})
	.partial();

export type Todo = z.infer<typeof selectTodoSchema>;
