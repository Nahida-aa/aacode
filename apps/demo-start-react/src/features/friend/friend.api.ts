import { and, desc, eq, or, sql } from 'drizzle-orm';
import { alias } from 'drizzle-orm/pg-core';
import z from 'zod';
import { type Db, db } from '#/db.server';
import { _createSingleDm } from '#/features/channel/channel.func';
import {
	_createFriend,
	_sendFriendRequest,
} from '#/features/friend/friend.func';
import { sendFriendRequestZ } from '#/features/friend/friend.schema';
import {
	friend,
	friendRequest,
	friendTag,
} from '#/features/friend/friend.table';
import { userItemFields } from '#/features/user/user.schema';
import { user } from '#/lib/auth.table';
import { authFn, getAuthFn } from '#/orpc.base';

const sendFriendRequest = authFn
	.input(sendFriendRequestZ)
	.handler(async ({ context, input, errors }) => {
		// 检查是否已经有好友关系或待处理的请求
		const friendRecord = await db
			.select()
			.from(friend)
			.where(
				and(
					eq(friend.userId, context.user.id),
					eq(friend.friendId, input.receiverId),
				),
			)
			.limit(1);
		if (friendRecord.length > 0)
			throw errors.BAD_REQUEST({ message: '已经是好友了' });
		// 检查是否已有 pending 请求
		const existingPending = await db
			.select()
			.from(friendRequest)
			.where(
				and(
					eq(friendRequest.emitterId, context.user.id),
					eq(friendRequest.receiverId, input.receiverId),
					// inArray(friendRequest.status, ['pending', 'rejected'])
				),
			)
			.limit(1);
		if (existingPending.length > 0 && existingPending[0].status === 'pending') {
			throw errors.BAD_REQUEST({ message: '请求已发送，等待对方回应' });
		}

		return await db.transaction(async (tx) => {
			// 插入或更新好友请求
			const [friendItem] = await _sendFriendRequest(tx, context.user.id, input);
			// let friendGroupId = await tx.query.friendGroup
			//   .findFirst({
			//     where: and(eq(friendGroup.userId, id), eq(friendGroup.name, data.groupName)),
			//   })
			//   .then(group => group?.id)
			// if (!friendGroupId) {
			//   const newGroup = await __createGroup(authId, groupName, tx)
			//   friendGroupId = newGroup.id
			// }
			// await tx.insert(friendGroupLink).values({
			//   friendGroupId,
			//   friendId: friendItem.id,
			// })

			// notify
			// const [newNotify] = await tx
			//   .insert(notify)
			//   .values(
			//     buildNotifyInsert('friend_request', id, {
			//       targetId,
			//       friendTableId: friendItem.id,
			//       username: username!,
			//       image,
			//       msg: data.message,
			//     }),
			//   )
			//   .returning()
			// await tx.insert(notifyReceiver).values({
			//   notifyId: newNotify.id,
			//   userId: targetId,
			// })
			// const wsIds = listWsByUser(targetId)
			// if (wsIds) {
			//   io?.to(wsIds).emit('friend_request', {
			//     senderId: id,
			//     friendTableId: friendItem.id,
			//     username: username!,
			//     image,
			//     msg: data.message,
			//   })
			// }
			return friendItem;
		});
	});

const listFriendRequest = getAuthFn.handler(async ({ context, errors }) => {
	const authId = context.user.id;
	const userEmitter = alias(user, 'user_emitter');
	const userReceiver = alias(user, 'user_receiver');
	const friendRequestList = await db
		.select({
			id: friendRequest.id,
			emitterId: friendRequest.emitterId,
			receiverId: friendRequest.receiverId,
			status: friendRequest.status,
			message: friendRequest.message,
			created_at: friendRequest.created_at,
			updated_at: friendRequest.updated_at,
			user: {
				id: sql<string>`COALESCE(
          CASE WHEN ${friendRequest.emitterId} = ${authId} THEN ${userReceiver.id}
              ELSE ${userEmitter.id}
          END
        )`.as('id'),
				username: sql<string>`COALESCE(
          CASE WHEN ${friendRequest.emitterId} = ${authId} THEN ${userReceiver.username}
              ELSE ${userEmitter.username}
          END
        )`.as('username'),
				displayUsername: sql<string>`COALESCE(
          CASE WHEN ${friendRequest.emitterId} = ${authId} THEN ${userReceiver.displayUsername}
              ELSE ${userEmitter.displayUsername}
          END
        )`.as('displayUsername'),

				image: sql<string>`COALESCE(
          CASE WHEN ${friendRequest.emitterId} = ${authId} THEN ${userReceiver.image}
              ELSE ${userEmitter.image}
          END
        )`.as('avatar'),
			},
		})
		.from(friendRequest)
		.leftJoin(userEmitter, eq(friendRequest.emitterId, userEmitter.id))
		.leftJoin(userReceiver, eq(friendRequest.receiverId, userReceiver.id))
		.where(
			or(
				eq(friendRequest.emitterId, authId),
				and(
					eq(friendRequest.receiverId, authId),
					eq(friendRequest.status, 'pending'),
				),
			),
		)
		.orderBy(desc(friendRequest.created_at));
	//
	return friendRequestList;
});

const acceptFriendRequest = authFn
	.input(z.object({ id: z.string().min(1) }))
	.handler(
		async ({ input, context, errors }) =>
			await db.transaction(async (tx) => {
				// 更新现有请求状态为accepted
				const [friendRequestItem] = await tx
					.update(friendRequest)
					.set({
						status: 'accepted',
						acceptedAt: new Date().toISOString(),
					})
					.where(
						and(
							eq(friendRequest.emitterId, input.id),
							eq(friendRequest.receiverId, context.user.id),
							eq(friendRequest.status, 'pending'),
						),
					)
					.returning({
						emitterId: friendRequest.emitterId,
						receiverId: friendRequest.receiverId,
						nickname: friendRequest.nickname,
						tags: friendRequest.tags,
					});
				if (!friendRequestItem) {
					throw errors.NOT_FOUND({ message: '没有找到对应的好友请求' });
				}
				await _createFriend(tx, friendRequestItem);
				// 创建私聊频道
				await _createSingleDm(input.id, context.user.id);
				return friendRequestItem;
			}),
	);

const rejectFriendRequest = authFn
	.input(z.object({ id: z.string().min(1) }))
	.handler(async ({ input, context, errors }) => {
		// 更新请求状态为 rejected
		await db
			.update(friendRequest)
			.set({
				status: 'rejected',
			})
			.where(eq(friendRequest.id, input.id));
		return { message: '成功' };
	});

// R
const getFriend = getAuthFn
	.input(z.object({ id: z.string().min(1) }))
	.handler(async ({ input, context, errors }) => {
		const [ret] = await db
			.select()
			.from(friend)
			.where(
				and(eq(friend.userId, context.user.id), eq(friend.friendId, input.id)),
			);
		if (!ret) return null;
		return ret;
	});

// action listFriend
const listFriend = getAuthFn.handler(
	async ({ context, errors }) =>
		await db.query.friend.findMany({
			with: {
				user: {
					columns: userItemFields,
				},
				friendToTags: {
					with: {
						tag: true,
					},
				},
			},
		}),
);

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
		.delete(friend)
		.where(
			or(
				and(eq(friend.userId, authId), eq(friend.friendId, userId)),
				and(eq(friend.userId, authId), eq(friend.friendId, userId)),
			),
		);
};
const removeFriend = authFn
	.input(z.object({ userId: z.string().min(1) }))
	.handler(({ input, context }) =>
		_removeFriend(context.user.id, input.userId),
	);

/** 列出 用户有哪些 好友 标签 */
const listFriendTag = getAuthFn.handler(async ({ context, errors }) => {
	return db
		.select()
		.from(friendTag)
		.where(eq(friendTag.userId, context.user.id));
});

export async function __createTag(userId: string, name: string, db: Db) {
	const [newTag] = await db
		.insert(friendTag)
		.values({
			userId,
			name,
		})
		.returning();
	return newTag;
}
const createFriendTag = authFn
	.input(z.object({ name: z.string().min(1) }))
	.handler(async ({ input, context, errors }) => {
		return await __createTag(context.user.id, input.name, db);
	});

export const friendApi = {
	sendFriendRequest,
	acceptFriendRequest,
	listFriendRequest,
	rejectFriendRequest,
	getFriend,
	listFriend,
	removeFriend,
	listFriendTag,
	createFriendTag,
};
