import { and, desc, eq, or, sql } from 'drizzle-orm';
import { alias } from 'drizzle-orm/pg-core';
import z from 'zod';
import { type Db, db } from '#/db.server';
import { _createSingleDm } from '#/features/channel/channel.func';
import {
	_createFriend,
	_sendFriendRequest,
} from '#/features/friend/friend.func';
import {
	friendRequestTable,
	friendTable,
	friendTagTable,
} from '#/features/friend/friend.table';
import { friendTag } from '#/features/friend/friendTag.rpc.ts';
import { userItemFields } from '#/features/user/user.schema';
import { user } from '#/lib/auth/auth.table.ts';
import { authFn, getAuthFn } from '#/orpc.base';

// R
const getFriend = getAuthFn
	.input(z.object({ id: z.string().min(1) }))
	.handler(async ({ input, context, errors }) => {
		const [ret] = await db
			.select()
			.from(friendTable)
			.where(
				and(
					eq(friendTable.userId, context.user.id),
					eq(friendTable.friendId, input.id),
				),
			);
		if (!ret) return null;
		return ret;
	});

// action listFriend

// export async function listFriend(authId: string): Promise<FriendItem[]> {
//   // 查询所有已接受的好友关系，并关联用户信息
//   const friends = await db
//     .select({
//       id: friend.id,
//       user1Id: friend.user1Id,
//       user2Id: friend.user2Id,
//       user1: {
//         id: user.id,
//         username: user.username,
//         displayUsername: user.displayUsername,
//         image: user.image,
//       },
//       user2: {
//         id: user.id,
//         username: user.username,
//         displayUsername: user.displayUsername,
//         image: user.image,
//       },
//     })
//     .from(friend)
//     .leftJoin(user, eq(friend.user1Id, user.id))
//     .leftJoin(user, eq(friend.user2Id, user.id))
//     .where(
//       and(
//         or(eq(friend.user1Id, authId), eq(friend.user2Id, authId)),
//         eq(friend.status, "accepted"),
//       ),
//     );

//   // 转换为 Friend 类型，排除当前用户，只保留好友信息
//   return friends;
// }

export const _removeFriend = async (authId: string, userId: string) => {
	await db
		.delete(friendTable)
		.where(
			or(
				and(eq(friendTable.userId, authId), eq(friendTable.friendId, userId)),
				and(eq(friendTable.userId, authId), eq(friendTable.friendId, userId)),
			),
		);
};
const removeFriend = authFn
	.input(z.object({ userId: z.string().min(1) }))
	.handler(({ input, context }) =>
		_removeFriend(context.user.id, input.userId),
	);

export const friend = {
	getFriend,
	select: getAuthFn.handler(({ context }) =>
		db
			.select()
			.from(friendTable)
			.where(eq(friendTable.userId, context.user.id)),
	),
	removeFriend,
};
