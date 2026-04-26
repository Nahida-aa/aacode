import { createInsertSchema, createSelectSchema } from 'drizzle-zod';
import {
	channel,
	dmMember,
	dmRoom,
} from '#/features/channel/channel.table';

// channel
export const channelSelectSchema = createSelectSchema(channel);
export type ChannelSelect = typeof channel.$inferSelect;
export const channelInsertSchema = createInsertSchema(channel);
export type ChannelInsert = typeof channel.$inferInsert;

// dmRoom
export const dmRoomSelectSchema = createSelectSchema(dmRoom);
export type DmRoomSelect = typeof dmRoom.$inferSelect;
export const dmRoomInsertSchema = createInsertSchema(dmRoom);
export type DmRoomInsert = typeof dmRoom.$inferInsert;

// dmMember
export const dmMemberSelectSchema = createSelectSchema(dmMember);
export type DmMemberSelect = typeof dmMember.$inferSelect;
export const dmMemberInsertSchema = createInsertSchema(dmMember);
export type DmMemberInsert = typeof dmMember.$inferInsert;