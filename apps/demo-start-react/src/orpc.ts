import { orpcTodoApi } from '#/features/orpcTodo/orpcTodo.api.ts';
import { todoApi } from '#/features/todo/todo.api';

export const orpcRouter = {
	...orpcTodoApi,
	...todoApi,
} as const;
