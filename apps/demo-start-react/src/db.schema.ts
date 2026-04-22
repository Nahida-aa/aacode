import { todo } from '#/features/todo/todo.table';
import {
	account,
	session,
	twoFactor,
	user,
	verification,
} from '#/lib/auth.table';

export * from './db.relations';
export { account, session, todo, twoFactor, user, verification };
