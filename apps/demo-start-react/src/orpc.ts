import { todoApi } from '#/features/todo/todo.api';


export const orpcRouter = {
	...todoApi,
} as const;
