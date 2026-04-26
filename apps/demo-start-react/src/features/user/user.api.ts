import { and, eq, ilike, ne, or, sql } from 'drizzle-orm';
import z from 'zod';
import { pickColumns } from '#/db.helpers';
import { db } from '#/db.server';
import { userUpdateZ } from '#/features/user/user.schema';
import { profile } from '#/features/user/user.table';
import { user } from '#/lib/auth.table';
import { searchQuery } from '#/lib/utils/zod.ts';
import { authFn, Fn, getAuthFn, getAuthOrNotFn, getFn } from '#/orpc.base';

const getUser = async (id: string) => {
	const userProfile = await db
		.select({
			...pickColumns(user, {
				id: true,
				name: true,
				email: true,
				image: true,
				username: true,
				displayUsername: true,
				phoneNumber: true,
			}),
			...pickColumns(profile, {
				summary: true,
				birthday: true,
				personalizedRecommendation: true,
				color: true,
				banner: true,
			}),
		})
		.from(user)
		.leftJoin(profile, eq(user.id, profile.userId))
		.where(eq(user.id, id));
	if (userProfile.length === 0) {
		// throw AppErr('用户不存在')
		return null;
	}
	return userProfile[0];
};
export type UserProfile = NonNullable<Awaited<ReturnType<typeof getUser>>>;

const updateUser = authFn
	.input(userUpdateZ)
	.handler(
		async ({
			input: { username, displayUsername, name, image, ...data },
			context,
		}) => {
			return await db.transaction(async (tx) => {
				await tx
					.update(user)
					.set({
						username: username,
						displayUsername: displayUsername,
						name: name || displayUsername || undefined,
						image: image,
					})
					.where(eq(user.id, context.user.id));
				// 如果 data 字段全部为空，则不更新 profile 表
				if (!Object.values(data).some((v) => v !== undefined)) return true;
				await tx
					.insert(profile)
					.values({
						userId: context.user.id,
						...data,
					})
					.onConflictDoUpdate({
						target: profile.userId,
						set: data,
					});
				return true;
			});
		},
	);

export const userApi = {
	getUser: getFn
		.input(z.object({ id: z.string() }))
		.handler(async ({ input }) => await getUser(input.id)),
	// getMe: authFn
	searchUser: getAuthOrNotFn
		.input(searchQuery.extend({}))
		.handler(async ({ input, context }) => {
			const filter = [
				or(
					ilike(user.username, `%${input.q}%`),
					ilike(user.displayUsername, `%${input.q}%`),
					eq(user.phoneNumber, `+86${input.q}`),
				),
			];
			if (context.user) {
				// 已登录用户需要排除自己
				filter.push(ne(user.id, context.user.id));
			}
			const users = await db
				.select()
				.from(user)
				.where(and(...filter));
			return users;
		}),

	updateUser,
};
