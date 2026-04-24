import { createFileRoute } from '@tanstack/react-router';
import { z } from 'zod';
import { useAppForm } from '#/components/uix/form/useAppForm.tsx';
import { Editor } from '#/components/uix/prosemirror/editor.tsx';

export const Route = createFileRoute('/ui/display/md/prosemirror/')({
	component: RouteComponent,
});

function RouteComponent() {
	const formSchema = z.object({
		content: z.any(),
	});
	localStorage.getItem('prosemirror');
	const form = useAppForm({
		formId: 'prosemirror',
		defaultValues: {} as z.input<typeof formSchema>,
		validators: {
			onChange: formSchema,
		},
		onSubmit: async ({ value }) => {
			console.log(value);
		},
	});

	return (
		<form.AppForm>
			<form.SyncToLocalStorage />
			<form.Form>
				<div className="p-4 sm:p-10">
					<form.AppField
						name="content"
						children={(field) => (
							<Editor
								initialValue={field.state.value}
								onSave={field.handleChange}
								className="bg-input/50 prose dark:prose-invert prose-neutral"
							/>
						)}
					/>
				</div>
			</form.Form>
		</form.AppForm>
	);
}
