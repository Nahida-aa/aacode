import hljs from 'highlight.js';
import { exampleSetup } from 'prosemirror-example-setup';
import { highlightPlugin } from 'prosemirror-highlightjs';
import { inputRules } from 'prosemirror-inputrules';
import { EditorState } from 'prosemirror-state';
import { type Decoration, DecorationSet, EditorView } from 'prosemirror-view';
import {
	forwardRef,
	memo,
	useCallback,
	useEffect,
	useImperativeHandle,
	useRef,
	useState,
} from 'react';
import { createPortal } from 'react-dom';
import { useDebounceCallback } from 'usehooks-ts';
import {
	buildNullDocument,
	documentToMd,
	handleTransaction,
	jsonToDocument,
	mdToDocument,
} from '#/components/uix/prosemirror/utils.tsx';
import { codeBlockRule, documentSchema, headingRule } from './config';
// 引入一個你喜歡的高亮主題
import 'highlight.js/styles/github-dark.css';
type EditorProps = {
	initialValue?: any;
	onSave?: (json: any) => void;
	className?: string;
};
export interface EditorRef {
	save: () => { json: any; md: string };
}
const PureEditor = forwardRef<EditorRef, EditorProps>(
	(
		{
			initialValue,
			onSave,
			className = 'prose dark:prose-invert prose-neutral',
		},
		ref,
	) => {
		const containerRef = useRef<HTMLDivElement>(null);
		const editorRef = useRef<EditorView | null>(null);
		const debouncedSave = useDebounceCallback(() => {
			if (!editorRef.current) return;
			const json = editorRef.current.state.doc.toJSON();
			onSave?.(json);
		}, 1000);

		const save = () => {
			if (!editorRef.current) return { json: null, md: '' };
			const doc = editorRef.current.state.doc;
			return {
				json: doc.toJSON(),
				md: documentToMd(doc),
			};
		};
		// 暴露给外部的方法
		useImperativeHandle(ref, () => ({
			save,
		}));
		useEffect(() => {
			if (containerRef.current && !editorRef.current) {
				const state = EditorState.create({
					doc: jsonToDocument(initialValue),
					plugins: [
						...exampleSetup({ schema: documentSchema, menuBar: false }),
						highlightPlugin(hljs),
						inputRules({
							rules: [
								headingRule(1),
								headingRule(2),
								headingRule(3),
								headingRule(4),
								headingRule(5),
								headingRule(6),
								codeBlockRule(), // 加入代碼塊規則
							],
						}),
					],
				});

				editorRef.current = new EditorView(containerRef.current, {
					state,
					attributes: {
						// class: 'prose prose-slate max-w-none focus:outline-none min-h-[300px] p-4 cursor-text'
						class: 'p-4 cursor-text',
					},
					handleDOMEvents: {
						click(_view, event) {},
					},
					dispatchTransaction(transaction) {
						if (!editorRef?.current) return;
						const newState = this.state.apply(transaction);
						editorRef.current.updateState(newState);
						// 3. 內容有變動時，觸發防抖
						if (transaction.docChanged) {
							debouncedSave();
						}
					},
				});
			}

			return () => {
				if (editorRef.current) {
					editorRef.current.destroy();
					editorRef.current = null;
				}
			};
		}, [initialValue, debouncedSave]);

		return (
			<div className={`relative max-w-none ${className}`} ref={containerRef} />
		);
	},
);
function areEqual(prevProps: EditorProps, nextProps: EditorProps) {
	return (
		prevProps.className === nextProps.className &&
		prevProps.onSave === nextProps.onSave &&
		prevProps.initialValue === nextProps.initialValue
	);
}

export const Editor = memo(PureEditor, areEqual);
