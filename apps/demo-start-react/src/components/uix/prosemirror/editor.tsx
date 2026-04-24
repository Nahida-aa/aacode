import hljs from 'highlight.js';
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
import {
	blockQuoteRule,
	bulletListRule,
	codeBlockBoundaryArrowDown,
	codeBlockBoundaryArrowUp,
	codeBlockEnter,
	codeBlockRule,
	createCodeBlockBackspace,
	documentSchema,
	exampleSetup,
	headingRule,
	orderedListRule,
} from './config';
// 引入一個你喜歡的高亮主題
// import 'highlight.js/styles/github-dark.css';
import '@catppuccin/highlightjs/css/catppuccin-macchiato.css';
import { keymap } from 'prosemirror-keymap';
import { splitListItem } from 'prosemirror-schema-list';

type EditorProps = {
	initialValue?: any;
	onSave?: (json: any) => void;
	className?: string;
};
export interface EditorRef {
	save: () => { json: any; md: string };
	getFileCache: () => Map<string, File>;
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
		const imageFileCache = new Map<string, File>();
		const handlePaste = useCallback(
			(view: EditorView, event: ClipboardEvent) => {
				if (!editorRef.current) {
					console.warn('Editor not initialized yet');
					return false;
				}
				console.log('handlePaste');
				const items = event.clipboardData?.items;
				const html = event.clipboardData?.getData('text/html');

				if (!items) {
					console.log('没有items');
					return false;
				}
				if (html) {
					// 如果是 html 说明可能是网络图片 自带 网络链接
					console.log('有html', html);
					return false;
				}
				console.log('有items或者没有html');
				for (const item of items) {
					console.log('可能是本地图片');
					if (item.kind === 'file' && item.type.startsWith('image')) {
						const file = item.getAsFile();
						if (!file) continue;
						console.log('有file', file);
						// 1. 創建本地預覽 URL (避免 Base64 佔用內存)
						const localUrl = URL.createObjectURL(file);
						// 【关键】将文件存入缓存，等待提交
						imageFileCache.set(localUrl, file);
						// 2. 插入圖片節點（暫時使用本地 URL）
						const { image } = editorRef.current.state.schema.nodes;
						const tr = view.state.tr.replaceSelectionWith(
							image.create({ src: localUrl }),
						);
						console.log('插入图片节点');
						view.dispatch(tr);

						return true; // 拦截默认行為，防止生成 Base64
					}
				}
				return false;
			},
			[imageFileCache.set],
		);
		// 暴露给外部的方法
		useImperativeHandle(ref, () => ({
			save,
			getFileCache: () => imageFileCache,
		}));
		useEffect(() => {
			if (containerRef.current && !editorRef.current) {
				const state = EditorState.create({
					doc: jsonToDocument(initialValue),
					plugins: [
						inputRules({
							rules: [
								headingRule(6),
								blockQuoteRule(),
								bulletListRule(),
								orderedListRule(),
								codeBlockRule(), // 加入代碼塊規則
							],
						}),
						keymap({
							Enter: codeBlockEnter,
							Backspace: createCodeBlockBackspace(),
							ArrowUp: codeBlockBoundaryArrowUp,
							ArrowDown: codeBlockBoundaryArrowDown,
						}),
						...exampleSetup({ schema: documentSchema, menuBar: false }),
						highlightPlugin(hljs),
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
						paste: handlePaste,
					},
				});
			}
			return () => {
				if (editorRef.current) {
					editorRef.current.destroy();
					editorRef.current = null;
				}
			};
		}, [initialValue, handlePaste]);
		useEffect(() => {
			if (!editorRef.current) return;

			editorRef.current.setProps({
				dispatchTransaction: (transaction) => {
					const newState = editorRef.current!.state.apply(transaction);
					editorRef.current!.updateState(newState);
					// 3. 內容有變動時，觸發防抖
					if (transaction.docChanged) {
						debouncedSave();
					}
				},
			});
		}, [debouncedSave]);

		return (
			<div className={`relative max-w-none ${className}`} ref={containerRef} />
		); // <div>2</div>
	},
);
function areEqual(prevProps: EditorProps, nextProps: EditorProps) {
	return (
		prevProps.className === nextProps.className &&
		prevProps.onSave === nextProps.onSave
	);
}

export const Editor = memo(PureEditor, areEqual);
