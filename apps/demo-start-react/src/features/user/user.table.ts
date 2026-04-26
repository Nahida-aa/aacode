import { boolean, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import { pgNanoid } from '#/db.helpers';
import { user } from '#/lib/auth.table';

// 由于可能存在命名冲突( console.profile  ), 因此补充后缀 Table, 改名: profile -> profile
export const profile = pgTable('profile', {
	id: pgNanoid(),
	userId: text('user_id')
		.references(() => user.id, { onDelete: 'cascade' })
		.notNull()
		.unique(),
	summary: text('summary'),
	description: text('description'),
	birthday: timestamp('birthday'),
	personalizedRecommendation: boolean('personalized_recommendation').default(
		false,
	),
	color: text('color'),
	banner: text('banner'),
});
