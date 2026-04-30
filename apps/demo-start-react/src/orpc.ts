import { friend } from '#/features/friend/friend.api.ts';
import { friendRequest } from '#/features/friend/friendRequest.rpc.ts';
import { friendTag } from '#/features/friend/friendTag.rpc.ts';
import { orpcTodoApi } from '#/features/orpcTodo/orpcTodo.api.ts';
import { tanstackDbApi } from '#/features/tanstackDb/tanstackDb.api.ts';
import { todoApi } from '#/features/todo/todo.api';
import { profile } from '#/features/user/profile.rpc.ts';
import { userApi } from '#/features/user/user.api';
import { uploadApi } from '#/lib/upload/upload.api.ts';

export const orpcRouter = {
	user: userApi,
	profile,
	friendRequest,
	friend,
	friendTag,
	...orpcTodoApi,
	...todoApi,
	...tanstackDbApi,
	...uploadApi,
} as const;
