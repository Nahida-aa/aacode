import { ClientOnly, createFileRoute } from '@tanstack/react-router';
import { SendIcon } from 'lucide-react';
import { useRef } from 'react';
import { useLocalStorage } from 'usehooks-ts';
import { z } from 'zod';
import { useAppForm } from '#/components/uix/form/useAppForm.tsx';
import {
	Editor,
	type EditorRef,
} from '#/components/uix/prosemirror/editor.tsx';
import { useFileUpload } from '#/lib/upload/useFileUpload.ts';

export const Route = createFileRoute('/ui/display/md/prosemirror/')({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div>
			<Editor className="bg-input/50 prose dark:prose-invert prose-neutral" />
			<ClientOnly>
				<MsgInput />
			</ClientOnly>
		</div>
	);
}

const MsgInput = () => {
	const formSchema = z.object({
		content: z.any(),
	});
	const editorRef = useRef<EditorRef>(null);
	type FormValues = z.infer<typeof formSchema>;
	// const [value, setValue] = useLocalStorage<FormValues>(
	// 	'prosemirror',
	// 	{} as FormValues,
	// );

	const { uploadFiles, status } = useFileUpload();
	const initialValuesRef = useRef<FormValues | null>(null);
	if (initialValuesRef.current === null) {
		try {
			const raw = localStorage.getItem('prosemirror');
			initialValuesRef.current = raw
				? (JSON.parse(raw) as FormValues)
				: ({} as FormValues);
		} catch {
			initialValuesRef.current = {} as FormValues;
		}
	}
	const form = useAppForm({
		formId: 'prosemirror',
		defaultValues: initialValuesRef.current as z.input<typeof formSchema>,
		validators: {
			onChange: formSchema,
		},
		onSubmit: async ({ value }) => {
			// 2. 通過 ref 拿到內部的 Map
			const cache = editorRef.current?.getFileCache();
			// 1. 找出真正存在於文檔中的 blobUrls 和對應的 Files
			const activeEntries =
				cache && cache.size > 0
					? Array.from(cache.entries()).filter(([url]) =>
							value.content.includes(url),
						)
					: [];
			if (activeEntries.length > 0) {
				const blobUrls = activeEntries.map(([url]) => url);
				const files = activeEntries.map(([_, file]) => file);
				console.log(`正在批量上傳 ${files.length} 張圖片...`);

				// 2. 呼叫你的批量上傳函數
				const uploadedResults = await uploadFiles(files);

				// 3. 遍历结果，替换本地结果
				// 注意：uploadFiles 返回的順序通常與傳入的 files 順序一致
				uploadedResults.forEach((result, index) => {
					const localUrl = blobUrls[index];
					const remoteUrl = `key://${result.storageKey}`; // 替換成實際的遠程 URL，可能需要根據你的後端返回值調整

					if (localUrl && remoteUrl) {
						// 全局替換 JSON 字符串中的本地鏈接
						value.content = value.content.split(localUrl).join(remoteUrl);

						// 釋放內存
						URL.revokeObjectURL(localUrl);
						cache?.delete(localUrl);
					}
				});
			}

			console.log(value);
		},
	});
	// https://avatars.githubusercontent.com/u/188596056?v=4&size=64
	return (
		<form.AppForm>
			<form.SyncToLocalStorage />
			<form.Form onSubmit={form.handleSubmit}>
				<div className="p-4 sm:p-10">
					<form.AppField
						name="content"
						children={(field) => (
							// <ClientOnly>
							<Editor
								ref={editorRef}
								initialValue={initialValuesRef.current?.content}
								onSave={field.handleChange}
								className="bg-input/50 prose dark:prose-invert prose-neutral"
							/>
							// </ClientOnly>
						)}
					/>
				</div>
				<form.SubmitButton label="Send" icon={<SendIcon />} />
			</form.Form>
		</form.AppForm>
	);
};
