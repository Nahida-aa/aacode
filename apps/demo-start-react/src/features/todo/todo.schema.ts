import { createSelectSchema, createUpdateSchema } from 'drizzle-zod';
import type { z } from 'zod';
import { todo } from '#/features/todo/todo.table.ts';

export const selectTodoSchema = createSelectSchema(todo);
export const updateTodoSchema = createUpdateSchema(todo).pick({
	title: true,
	content: true,
	completed: true,
});
export const addTodoSchema = selectTodoSchema.pick({
	title: true,
	content: true,
});

export type Todo = z.infer<typeof selectTodoSchema>;
