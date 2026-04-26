import { z } from 'zod';
import { fileShowZ } from '#/lib/upload/upload.const';

export const channelOwnerTypes = [
	'community',
	'project',
	'team',
	'user',
] as const;

export const messageMentionsZ = z.object({
	users: z.array(z.string()).optional(),
	roles: z.array(z.string()).optional(),
	channels: z.array(z.string()).optional(),
	everyone: z.boolean().optional(),
});
export type MessageMentions = z.output<typeof messageMentionsZ>;

export const messageReactionZ = z.object({
	emoji: z.string(),
	count: z.number(),
	users: z.array(z.string()),
});
export type MessageReaction = z.output<typeof messageReactionZ>;

export const messageAttachmentsZ = z.array(fileShowZ);
export type MessageAttachments = z.output<typeof messageAttachmentsZ>;
export type ChannelType =
	| 'chat'
	| 'discussion' // 讨论,评论,社区
	| 'readme' //  自述文件, 自我介绍
	| 'forum' // 论坛
	| 'welcome' // 欢迎
	| 'announcement' // 公告
	| 'guide' // 指南: 可以是其他人提供的攻略
	| 'release'; // 发布会, 发布版本,用户侧

export type PermissionOverwrite = {
	id: string; // 角色ID或用户ID
	type: 'role' | 'member';
	allow: string[]; // 允许的权限
	deny: string[]; // 拒绝的权限
};
