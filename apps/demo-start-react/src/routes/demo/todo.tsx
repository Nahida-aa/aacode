import { useLiveQuery } from '@tanstack/react-db';
import { ClientOnly, createFileRoute } from '@tanstack/react-router';
import type { z } from 'better-auth';
import { Check, HistoryIcon, PlusIcon, TimerIcon } from 'lucide-react';
import { useLocalStorage } from 'usehooks-ts';
import { Button } from '#/components/ui/button.tsx';
import { useAppForm } from '#/components/uix/form/useAppForm.tsx';
import { Description } from '#/components/uix/label.tsx';
import { todoCollection } from '#/features/todo/todo.collection.ts';
import { addTodoSchema, type Todo } from '#/features/todo/todo.schema.ts';
import { formatToNow } from '#/lib/utils.timeFormat.ts';

export const Route = createFileRoute('/demo/todo')({
	loader: async () => {
		await Promise.all([todoCollection.preload()]);

		return null;
	},
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div>
			<ClientOnly fallback={<TodoListFallback />}>
				<TodoListClient />
			</ClientOnly>
		</div>
	);
}

function TodoListClient() {
	const { data: todos } = useLiveQuery((q) =>
		q
			.from({ todo: todoCollection })
			.orderBy(({ todo }) => todo.updated_at, 'desc'),
	);
	console.log({
		isDate: todos?.[0]?.updated_at instanceof Date,
	});
	return (
		<div className="p-2 flex flex-col gap-2">
			<AddTodoCard />
			{todos?.map((todo) => (
				<TodoCard key={todo.id} todo={todo} />
			))}
		</div>
	);
}

function TodoListFallback() {
	return <div>Loading todos...</div>;
}

function TodoCard({ todo }: { todo: Todo }) {
	return (
		<div className="bg-muted p-2 rounded-md">
			<div className="flex justify-between items-center">
				<div className="flex items-center gap-2">
					<button
						onClick={() =>
							todoCollection.update(todo.id, (draft) => {
								console.log('UpdatingTodo.draft', draft);
								draft.completed = !todo.completed;
								draft.created_at = new Date(draft.created_at!);
								draft.updated_at = new Date();
							})
						}
						className={`shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all duration-200 ${
							todo.completed
								? 'bg-green-500 border-green-500 text-white'
								: 'border-green-300 hover:border-green-400 text-transparent hover:text-green-400'
						}`}
					>
						<Check size={14} />
					</button>
					<h3>{todo.title}</h3>
				</div>
				<Button
					size={'sm'}
					variant={'destructive'}
					onClick={() => todoCollection.delete(todo.id)}
				>
					Delete
				</Button>
			</div>
			{todo.content}
			<Description className="flex items-center gap-1">
				<HistoryIcon size={16} /> {formatToNow(todo.updated_at)}
			</Description>
		</div>
	);
}
function AddTodoCard() {
	type FormValues = z.input<typeof addTodoSchema>;
	const [value, setValue, removeValue] = useLocalStorage(
		'AddTodoCard',
		{} as FormValues,
	);
	const form = useAppForm({
		formId: 'AddTodoCard',
		defaultValues: value,
		validators: {
			onChange: addTodoSchema,
		},
		onSubmit: async ({ value, formApi }) => {
			const tx = await todoCollection.insert(value);
			formApi.reset();
		},
	});
	return (
		<form.AppForm>
			<form.Form
				onSubmit={form.handleSubmit}
				className="flex flex-col gap-3 bg-card rounded-md p-2"
			>
				<form.SyncToLocalStorage />
				<form.AppField name="title">
					{(field) => <field.FieldInput placeholder="Title" />}
				</form.AppField>
				<form.AppField name="content">
					{(field) => <field.FieldTextarea placeholder="Content" />}
				</form.AppField>
				<form.SubmitButton
					label="Add"
					canSubmitDefault
					icon={<PlusIcon size={20} />}
				/>
			</form.Form>
		</form.AppForm>
	);
}
