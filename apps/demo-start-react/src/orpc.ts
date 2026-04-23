import { orpcTodoApi } from '#/features/orpcTodo/orpcTodo.api.ts';
import { tanstackDbApi } from '#/features/tanstackDb/tanstackDb.api.ts';
import { todoApi } from '#/features/todo/todo.api';

export const orpcRouter = {
	...orpcTodoApi,
	...todoApi,
	...tanstackDbApi,
} as const;
