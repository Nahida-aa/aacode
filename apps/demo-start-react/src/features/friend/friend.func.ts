import type { Db } from '#/db.server';
import { friend, friendRequest } from '#/features/friend/friend.table';
import type { CreateFriend, SendFriendRequest } from './friend.schema';
export const _sendFriendRequest = (
	db: Db,
	authId: string,
	data: SendFriendRequest,
) =>
	db
		.insert(friendRequest)
		.values({
			emitterId: authId, // 发送请求的用户
			receiverId: data.receiverId, // 接收请求的用户
			status: 'pending',
			message: data.message,
			nickname: data.nickname,
		})
		.onConflictDoUpdate({
			target: [friendRequest.emitterId, friendRequest.receiverId],
			set: {
				status: 'pending',
				message: data.message,
				nickname: data.nickname,
			},
		})
		.returning();

// 创建好友关系: 双向插入
export const _createFriend = async (
	db: Db,
	{ emitterId, receiverId, nickname }: CreateFriend,
) => {
	await db.insert(friend).values([
		{ userId: emitterId, friendId: receiverId, nickname },
		{ userId: receiverId, friendId: emitterId },
	]);
};

// export const nicknameSql = (t: typeof friend, authId: string) => sql<string | null>`
//     CASE
//       WHEN ${t.user1Id} = ${authId} THEN ${t.nicknameFromUser1}
//       ELSE ${t.nicknameFromUser2}
//     END
//   `
