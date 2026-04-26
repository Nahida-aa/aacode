import { createInsertSchema, createSelectSchema } from 'drizzle-zod';
import z from 'zod';
import { friendRequest } from '#/features/friend/friend.table';

// export const friendItemZ = friendSelectZ
//   .omit({ created_at: true, updated_at: true, status: true, reason: true })
//   .extend({
//     user1: userItemZ.nullable(),
//     user2: userItemZ.nullable(),
//   });
// export type FriendItem = z.infer<typeof friendItemZ>;
export const friendColumns = {
	id: true,
	user1Id: true,
	user2Id: true,
	status: true,
	reason: true,
	// nicknameFromUser1: true,
	// nicknameFromUser2: true,
	created_at: true,
	updated_at: true,
} as const;

const createFriendZ = createSelectSchema(friendRequest).pick({
	emitterId: true,
	receiverId: true,
	nickname: true,
	tags: true,
});
export type CreateFriend = z.infer<typeof createFriendZ>;

const friendRequestInsertZ = createInsertSchema(friendRequest);

export const sendFriendRequestZ = friendRequestInsertZ.pick({
	receiverId: true,
	message: true,
	nickname: true,
	tags: true,
});
export type SendFriendRequest = z.infer<typeof sendFriendRequestZ>;

export const friendRequestZ = z.object({
	targetId: z.string(),
	msg: z.string().default('添加你为好友'),
	nickname: z.string().optional(),
	groupName: z.string().optional().default('default'),
});
export type FriendRequest = z.infer<typeof friendRequestZ>;
