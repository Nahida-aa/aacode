import { todo } from '#/features/todo/todo.table';
import {
	account,
	session,
	twoFactor,
	user,
	verification,
} from '#/lib/auth.table';
import { download, file } from '#/lib/upload/upload.table';

export * from './db.relations';
export {
	account,
	download,
	file,
	session,
	todo,
	twoFactor,
	user,
	verification,
};
