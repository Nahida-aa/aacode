import {
	type AnyPgColumn,
	boolean,
	index,
	integer,
	jsonb,
	pgTable,
	primaryKey,
	text,
	timestamp,
	uniqueIndex,
	varchar,
} from 'drizzle-orm/pg-core';
import { nanoidWithTimestamps } from '#/db.helpers';
import type {
	ChannelType,
	MessageAttachments,
	MessageMentions,
	MessageReaction,
	PermissionOverwrite,
} from '#/features/channel/channel.const';

import { user } from '#/lib/auth.table';

// 频道表 - 社区/项目/团队的频道
export const channel = pgTable(
	'channel',
	{
		...nanoidWithTimestamps,
		type: text('type').$type<ChannelType>().default('chat').notNull(),
		// 直接拥有者
		ownerType: text('owner_type').notNull().default('community'), // 'community' | 'project' | 'team'
		ownerId: text('owner_id').notNull(), // 指向 owner 的 id

		name: text('name'),
		description: text('description'),
		sort: integer('sort').default(0).notNull(),

		// 是否私有频道 - 私有频道需要成员权限才能访问
		isPrivate: boolean('is_private').default(false).notNull(),
		// 权限覆盖规则 - 针对角色或用户的自定义权限设置，支持细粒度访问控制
		permissionOverwrites: jsonb('permission_overwrites')
			.$type<PermissionOverwrite[]>()
			.default([])
			.notNull(),

		// 是否为成人内容频道 - 前端可据此显示年龄限制提示
		isNsfw: boolean('is_nsfw').default(false).notNull(),
		// 用户发言频率限制（秒）- 防止刷屏，0 表示无限制
		rateLimitPerUser: integer('rate_limit_per_user').default(0).notNull(),
	},
	(table) => [
		index('channel_owner_idx').on(table.ownerType, table.ownerId),
		index('channel_sort_idx').on(table.ownerId, table.sort),
	],
);

// 私聊\私人群聊会话表
export const dmRoom = pgTable(
	'dm_room',
	{
		...nanoidWithTimestamps,
		type: text('type', { enum: ['single', 'group'] })
			.default('single')
			.notNull(),
		name: varchar('name', { length: 100 }), // 群名，single 无需填写
		// 群头像，single 无需
		icon: text('icon'),
	},
	(table) => [index('dm_room_type_idx').on(table.type)],
);

// 私聊成员表
export const dmMember = pgTable(
	'dm_member',
	{
		...nanoidWithTimestamps,
		room_id: text('room_id')
			.notNull()
			.references(() => dmRoom.id, { onDelete: 'cascade' }),
		user_id: text('user_id')
			.notNull()
			.references(() => user.id, { onDelete: 'cascade' }),
		// 群聊中的角色，single 无需
		role: varchar('role', { length: 20 }).default('member'), // 'owner' | 'admin' | 'member'
		nickname: varchar('nickname', { length: 100 }), // 群聊中的昵称
	},
	(t) => [
		uniqueIndex('dm_member_unique_idx').on(t.room_id, t.user_id),
		index('dm_member_user_idx').on(t.user_id),
	],
);

// 消息表 - 统一频道消息和私聊消息
export const message = pgTable(
	'message',
	{
		...nanoidWithTimestamps,
		// 频道消息
		channel_id: text('channel_id').references(() => channel.id, {
			onDelete: 'cascade',
		}),
		// 私聊消息
		room_id: text('room_id').references(() => dmRoom.id, {
			onDelete: 'cascade',
		}),

		user_id: text('user_id').references(() => user.id, {
			onDelete: 'set null',
		}),

		// 发送者冗余信息（用于快速显示，用户改名后可能不一致）
		sender_name: text('sender_name'),
		sender_avatar: text('sender_avatar'),

		content: text('content'),
		content_category: text('content_category').default('text').notNull(),

		reply_to_id: text('reply_to_id').references((): AnyPgColumn => message.id), // 自引用

		is_edited: boolean('is_edited').default(false).notNull(),
		is_deleted: boolean('is_deleted').default(false).notNull(),
		is_pinned: boolean('is_pinned').default(false).notNull(),

		attachments: jsonb('attachments')
			.$type<MessageAttachments>()
			.default([])
			.notNull(),

		mentions: jsonb('mentions').$type<MessageMentions>().default({}).notNull(),

		reactions: jsonb('reactions')
			.$type<MessageReaction[]>()
			.default([])
			.notNull(),
	},
	(table) => [
		index('message_channel_idx').on(table.channel_id),
		index('message_room_idx').on(table.room_id),
		index('message_user_idx').on(table.user_id),
		index('message_created_at_idx').on(table.created_at),
	],
);

// 频道阅读状态
export const userReadState = pgTable(
	'user_read_state',
	{
		user_id: text('user_id')
			.notNull()
			.references(() => user.id, { onDelete: 'cascade' }),
		channel_id: text('channel_id')
			.notNull()
			.references(() => channel.id, { onDelete: 'cascade' }),
		last_read_message_id: text('last_read_message_id')
			.notNull()
			.references(() => message.id, { onDelete: 'cascade' }),
		last_read_at: timestamp('last_read_at')
			.$onUpdate(() => new Date())
			.notNull(),
	},
	(t) => [primaryKey({ columns: [t.user_id, t.channel_id] })],
);

// 私聊阅读状态
export const dmReadState = pgTable(
	'dm_read_state',
	{
		user_id: text('user_id')
			.notNull()
			.references(() => user.id, { onDelete: 'cascade' }),
		room_id: text('room_id')
			.notNull()
			.references(() => dmRoom.id, { onDelete: 'cascade' }),
		last_read_message_id: text('last_read_message_id')
			.notNull()
			.references(() => message.id, { onDelete: 'cascade' }),
		last_read_at: timestamp('last_read_at')
			.$onUpdate(() => new Date())
			.notNull(),
	},
	(t) => [primaryKey({ columns: [t.user_id, t.room_id] })],
);
