import { drizzle, type PostgresJsDatabase } from 'drizzle-orm/postgres-js';
import postgres from 'postgres';
import { env } from '#/env.ts';
import * as schema from './db.schema.ts';

// 🟢 防止开发模式下重复创建客户端
declare global {
	var __db_client__: ReturnType<typeof postgres> | undefined;
}
// 如果已经存在全局实例就复用，否则创建新的
const client =
	global.__db_client__ ??
	postgres(env.DATABASE_URL, {
		max: 10,
	});

export const db = drizzle({ client, schema });

type _Db = PostgresJsDatabase<typeof schema>;
export type Tx = Parameters<Parameters<_Db['transaction']>[0]>[0];
export type Db = typeof db | Tx;
