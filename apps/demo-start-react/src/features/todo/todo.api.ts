import { eq } from 'drizzle-orm';
import { z } from 'zod';
import { todo } from '#/db.schema.ts';
import { db } from '#/db.server.ts';
import { addTodoSchema, selectTodoSchema, updateTodoSchema } from '#/features/todo/todo.schema.ts';
import { generateTxId } from '#/integrations/electric/genTxid.ts';
import { Fn } from '#/orpc.base';

export const todoApi = {
	addTodo: Fn.input(addTodoSchema).handler(({ input }) =>db.transaction(async (tx)  => {
			const txid = await generateTxId(tx);
			const [newTodo] = await tx.insert(todo).values(input).returning();
			return { txid, item: newTodo };
		})),
	updateTodo: Fn.input(updateTodoSchema.extend({ id: z.string() })).handler(
		({ input }) =>
			db.transaction(async (tx) => {
				const txid = await generateTxId(tx);
				const { id, ...updateData } = input;
				const [updatedTodo] = await tx
					.update(todo)
					.set(updateData)
					.where(eq(todo.id, id))
					.returning();
				return { txid, item: updatedTodo };
			}),
	),
	deleteTodo: Fn.input(z.object({ id: z.string() })).handler(
		 ({ input }) =>db.transaction(async (tx) => {
				const txid = await generateTxId(tx);
				await tx.delete(todo).where(eq(todo.id, input.id));
				return { txid };
			})
	),
};
