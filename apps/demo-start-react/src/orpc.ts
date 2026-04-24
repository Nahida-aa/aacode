import { orpcTodoApi } from '#/features/orpcTodo/orpcTodo.api.ts';
import { tanstackDbApi } from '#/features/tanstackDb/tanstackDb.api.ts';
import { todoApi } from '#/features/todo/todo.api';
import { uploadApi } from '#/lib/upload/upload.api.ts';

export const orpcRouter = {
	...orpcTodoApi,
	...todoApi,
	...tanstackDbApi,
	...uploadApi,
} as const;
