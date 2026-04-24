import { defaultMarkdownSerializer } from 'prosemirror-markdown';
import { DOMParser, type Node, Schema } from 'prosemirror-model';
import type { Transaction } from 'prosemirror-state';
import type { EditorView } from 'prosemirror-view';
import type { RefObject } from 'react';
import { renderToString } from 'react-dom/server';
import { MessageResponse } from '#/components/ai-elements/message';
import { documentSchema } from '#/components/uix/prosemirror/documentSchema.ts';

export const mdToDocument = (content: string): Node => {
	const parser = DOMParser.fromSchema(documentSchema);
	// Markdown content -> html string
	const stringFromMarkdown = renderToString(
		<MessageResponse>{content}</MessageResponse>,
	);
	const tempContainer = document.createElement('div');
	tempContainer.innerHTML = stringFromMarkdown;
	// html node -> ProseMirror document node
	return parser.parse(tempContainer);
};
export const buildNullDocument = (): Node => {
	const parser = DOMParser.fromSchema(documentSchema);
	return parser.parse(document.createElement('div'));
};
export const jsonToDocument = (json?: any): Node => {
	if (!json) {
		return buildNullDocument();
	}
	try {
		return documentSchema.nodeFromJSON(json);
	} catch (error) {
		console.error('Failed to parse JSON to ProseMirror document:', error);
		return buildNullDocument();
	}
};
// ProseMirror document node -> Markdown content
export const documentToMd = (document: Node): string => {
	return defaultMarkdownSerializer.serialize(document);
};

/**
 * ###  **交易处理器** (`handleTransaction`)
 * **参数说明**：
- `transaction`：编辑器状态变更
- `editorRef`：编辑器实例引用
- `onSaveContent`：保存内容的回调函数

* **处理流程**：
1. ✅ 检查编辑器是否存在
2. 🔄 应用交易，更新编辑器状态
3. 💾 如果文档内容改变（`docChanged`）且未标记 `no-save`：
   - 构建更新后的内容
   - 判断是否需要防抖（debounce）
   - 触发 `onSaveContent` 回调

* **特殊标记**：
- `"no-save"`：标记的交易不触发保存
- `"no-debounce"`：立即保存，不等待防抖
 */
export const handleTransaction = ({
	transaction,
	editorRef,
	onSaveContent,
}: {
	transaction: Transaction;
	editorRef: RefObject<EditorView | null>;
	onSaveContent: (updatedContent: string, debounce: boolean) => void;
}) => {
	if (!editorRef?.current) {
		return;
	}

	const newState = editorRef.current.state.apply(transaction);
	editorRef.current.updateState(newState);

	if (transaction.docChanged && !transaction.getMeta('no-save')) {
		const updatedContent = documentToMd(newState.doc);

		if (transaction.getMeta('no-debounce')) {
			onSaveContent(updatedContent, false);
		} else {
			onSaveContent(updatedContent, true);
		}
	}
};
