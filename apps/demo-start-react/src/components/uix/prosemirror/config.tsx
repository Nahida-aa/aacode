import {
	InputRule,
	textblockTypeInputRule,
	undoInputRule,
	wrappingInputRule,
} from 'prosemirror-inputrules';
import { defaultMarkdownSerializer } from 'prosemirror-markdown';
import { DOMParser, type Node, Schema } from 'prosemirror-model';
import { schema } from 'prosemirror-schema-basic';
import { addListNodes } from 'prosemirror-schema-list';
import type { Command, Transaction } from 'prosemirror-state';
import type { EditorView } from 'prosemirror-view';
import type { RefObject } from 'react';

// bun add prosemirror-model prosemirror-state prosemirror-view prosemirror-inputrules prosemirror-schema-basic prosemirror-schema-list prosemirror-markdown

// 1. 擴展 Nodes 定義
const nodes = addListNodes(
	schema.spec.nodes,
	'paragraph block*',
	'block',
).append({
	code_block: {
		content: 'text*',
		marks: '',
		group: 'block',
		code: true,
		defining: true,
		attrs: { params: { default: '' } }, // 用於存儲語言名稱
		parseDOM: [
			{
				tag: 'pre',
				preserveWhitespace: 'full',
				getAttrs: (node) => ({
					params: (node as HTMLElement).getAttribute('data-params') || '',
				}),
			},
		],
		toDOM(node) {
			return ['pre', { 'data-params': node.attrs.params }, ['code', 0]];
		},
	},
	image: {
		attrs: {
			src: {},
			alt: { default: null },
			title: { default: null },
		},
		parseDOM: [
			{
				tag: 'img[src]',
				getAttrs: (dom) => ({
					src: (dom as HTMLElement).getAttribute('src'),
				}),
			},
		],
		toDOM(node) {
			console.log('image.toDOM');
			const { src: _src } = node.attrs;
			const src = getImgSrc(_src);
			console.log('getImgSrc', { _src, src });
			return ['img', { ...node.attrs, src: _src }];
		},
	},
});
/**
 * - 创建一个 ProseMirror Schema（文档模式）
 * - nodes：定义文档可以包含的块级元素
      - 基础节点来自 prosemirror-schema-basic
      - 通过 addListNodes 添加列表支持（有序/无序列表）
      - 结构：段落作为基础块，支持嵌套列表
 * - marks：定义文本标记（如 bold、italic、link 等）
 */
export const documentSchema = new Schema({
	nodes,
	marks: schema.spec.marks,
});

/**
 **标题快捷规则** (`headingRule`)
- 根据 Markdown 语法识别标题输入
- **参数**：`maxLevel` - 标题级别（1-6）
- **工作原理**：
  - 匹配正则表达式 `^(#{1,maxLevel})\s$`（行首多个 `#` 后跟空格）
  - 自动将输入转换为对应级别的标题
  - 例：输入 `### ` 自动转为三级标题
 */
export function headingRule(maxLevel: number = 6) {
	return textblockTypeInputRule(
		new RegExp(`^(#{1,${maxLevel}})\\s$`),
		documentSchema.nodes.heading,
		(match) => ({ level: match[1].length }),
	);
}
export function blockQuoteRule() {
	return wrappingInputRule(/^\s*>\s$/, documentSchema.nodes.blockquote);
}
export function bulletListRule() {
	return wrappingInputRule(/^\s*([-+*])\s$/, documentSchema.nodes.bullet_list);
}
export function orderedListRule() {
	return wrappingInputRule(
		/^(\d+)\.\s$/,
		documentSchema.nodes.ordered_list,
		(match) => ({ order: +match[1] }),
		(match, node) => node.childCount + node.attrs.order == +match[1],
	);
}
/**
 * 代码块快捷规则
 * 匹配 ``` 后直接回车，转换为代码块并设置语言属性
 */
export function codeBlockRule() {
	return new InputRule(
		/^```([a-z]+)?\s$/, // 這裡的 \s 匹配空格或回車觸發的空白字符
		(state, match, start, end) => {
			const lang = match[1] || '';
			const { code_block } = documentSchema.nodes;

			const tr = state.tr;

			// 1. 刪除用戶輸入的 ```ts 和後面的空格/回車 (start 到 end 的範圍)
			tr.delete(start, end);

			// 2. 將當前所在的段落節點轉換為 code_block
			// 註：delete 之後，原本的行依然存在，只是內容空了
			// 我們使用 setBlockType 將其轉為代碼塊
			return tr.setBlockType(start, start, code_block, { params: lang });
		},
	);
}
export const codeBlockEnter: Command = (state, dispatch) => {
	const { $from, empty } = state.selection;
	console.log('codeBlockEnter', {
		parentType: $from.parent.type.name,
		'$from.node(-1).type': $from.node(-1)?.type.name,
	});
	// 1. 確保選區是空的（光標狀態）且在段落中
	if (!empty || $from.parent.type.name !== 'paragraph') return false;

	// 2. 獲取當前行的文本內容
	const lineText = $from.parent.textContent;

	// 3. 匹配 ```lang 格式
	const match = lineText.match(/^```([a-z]+)?$/);

	if (match) {
		console.log('Matched code block input:', {
			lang: match[1] || 'none',
		});
		if (dispatch) {
			const lang = match[1] || '';
			const { code_block } = state.schema.nodes;
			// 直接用一個新的 code_block 替換掉整個段落
			const tr = state.tr.replaceWith(
				$from.before(),
				$from.after(),
				code_block.create({ params: lang }),
			);

			// 設置選區到新代碼塊內部
			const newPos = tr.doc.resolve($from.before() + 1);
			tr.setSelection(TextSelection.near(newPos));

			dispatch(tr.scrollIntoView());
		}
		return true; // 拦截回車，不執行原生的換行
	}

	return false;
};
interface BackspaceOptions {
	/** 是否保留 ```ts 文本，若為 false 則直接刪除整行並轉為空段落 */
	keepMarkdownSyntax?: boolean;
}

export const createCodeBlockBackspace = (
	options: BackspaceOptions = {},
): Command => {
	return (state, dispatch) => {
		const { $from, empty } = state.selection;

		if (!empty) return false;

		const parent = $from.parent;
		// if (parent.type.name !== 'code_block')
		// 	return undoInputRule(state, dispatch);
		// 當代碼塊為空時
		if (parent.content.size === 0) {
			if (dispatch) {
				const { paragraph } = state.schema.nodes;
				const tr = state.tr;

				if (parent.type.name === 'code_block' && options.keepMarkdownSyntax) {
					// 模式 A: 保留 ```ts 語法
					const lang = parent.attrs.params || '';
					const textContent = `\`\`\`${lang} `;

					tr.setBlockType($from.before(), $from.after(), paragraph).insertText(
						textContent,
						$from.before() + 1,
					);
				} else {
					// 1. 先尝试 undoInputRule
					if (undoInputRule(state, dispatch)) return true;

					// 直接刪除节点，转为普通空段落
					tr.setBlockType($from.before(), $from.after(), paragraph);
				}

				dispatch(tr.scrollIntoView());
			}
			return true;
		}

		return undoInputRule(state, dispatch);
	};
};
/**
 * 處理代碼塊邊界的自動換行邏輯
 */
export const codeBlockBoundaryArrowUp: Command = (state, dispatch) => {
	const { $from, $to } = state.selection;

	// 1. 確保是光標，且在代碼塊內
	if ($from.parent.type.name !== 'code_block') return false;

	// 判斷光標是否在代碼塊的第一行
	// $from.parentOffset 是光標相對於當前代碼塊開頭的距離
	// 如果光標位置之前沒有換行符，說明在第一行
	const isFirstLine = !$from.parent.textContent
		.slice(0, $from.parentOffset)
		.includes('\n');

	if (isFirstLine) {
		console.log('isFirstLine', {
			'$from.before()': $from.before(),
		});
		// 如果上方沒有節點了（它是文檔第一個節點）
		if ($from.before() === 0) {
			if (dispatch) {
				const { paragraph } = state.schema.nodes;
				// 在最前面插入空行
				const tr = state.tr.insert(0, paragraph.create());
				// 設置光標：新插入的段落起點是 1
				const resolvedPos = tr.doc.resolve(1);
				const selection = TextSelection.near(resolvedPos);
				tr.setSelection(selection);

				dispatch(tr.scrollIntoView()); // 確保視圖跟隨
			}
			return true;
		}
	}
	return false;
};
export const codeBlockBoundaryArrowDown: Command = (state, dispatch) => {
	const { $from, $to } = state.selection;
	// 1. 確保是光標，且在代碼塊內
	if ($from.parent.type.name !== 'code_block') return false;
	console.log('codeBlockBoundaryArrowDown');

	// 判斷光標是否在代碼塊的最後一行
	const isLastLine = !$from.parent.textContent
		.slice($from.parentOffset)
		.includes('\n');

	if (isLastLine) {
		console.log('isLastLine', {
			'$from.after()': $from.after(),
			'state.doc.content.size': state.doc.content.size,
		});
		// 如果下方沒有節點了（它是文檔最後一個節點）
		if ($from.after() === state.doc.content.size) {
			console.log('isLastNode');
			if (dispatch) {
				const { paragraph } = state.schema.nodes;
				// 在最後面插入空行
				const insertPos = state.doc.content.size;
				console.log('insert paragraph');
				const tr = state.tr.insert(insertPos, paragraph.create());
				// 設置光標：跳轉到新插入的段落內部
				// 位置計算：insertPos 是原本的末尾，+1 進入新段落
				const resolvedPos = tr.doc.resolve(insertPos + 1);
				const selection = TextSelection.near(resolvedPos);
				tr.setSelection(selection);

				dispatch(tr.scrollIntoView());
			}
			return true;
		}
	}
	return false;
};

import { baseKeymap } from 'prosemirror-commands';
import { dropCursor } from 'prosemirror-dropcursor';
import { gapCursor } from 'prosemirror-gapcursor';
import { history } from 'prosemirror-history';
import { keymap } from 'prosemirror-keymap';
import { type MenuElement, menuBar } from 'prosemirror-menu';
import { Plugin, TextSelection } from 'prosemirror-state';
import { getImgSrc } from '#/components/uix/img/img.utils.ts';
import { buildInputRules } from './inputrules';
import { buildKeymap } from './keymap';
import { buildMenuItems } from './menu';

export { buildInputRules, buildKeymap, buildMenuItems };

/// Create an array of plugins pre-configured for the given schema.
/// The resulting array will include the following plugins:
///
///  * Input rules for smart quotes and creating the block types in the
///    schema using markdown conventions (say `"> "` to create a
///    blockquote)
///
///  * A keymap that defines keys to create and manipulate the nodes in the
///    schema
///
///  * A keymap binding the default keys provided by the
///    prosemirror-commands module
///
///  * The undo history plugin
///
///  * The drop cursor plugin
///
///  * The gap cursor plugin
///
///  * A custom plugin that adds a `menuContent` prop for the
///    prosemirror-menu wrapper, and a CSS class that enables the
///    additional styling defined in `style/style.css` in this package
///
/// Probably only useful for quickly setting up a passable
/// editor—you'll need more control over your settings in most
/// real-world situations.
export function exampleSetup(options: {
	/// The schema to generate key bindings and menu items for.
	schema: Schema;

	/// Can be used to [adjust](#example-setup.buildKeymap) the key bindings created.
	mapKeys?: { [key: string]: string | false };

	/// Set to false to disable the menu bar.
	menuBar?: boolean;

	/// Set to false to disable the history plugin.
	history?: boolean;

	/// Set to false to make the menu bar non-floating.
	floatingMenu?: boolean;

	/// Can be used to override the menu content.
	menuContent?: MenuElement[][];
}) {
	const plugins = [
		// buildInputRules(options.schema),
		keymap(buildKeymap(options.schema, options.mapKeys)),
		keymap(baseKeymap),
		dropCursor(),
		gapCursor(),
	];
	if (options.menuBar !== false)
		plugins.push(
			menuBar({
				floating: options.floatingMenu !== false,
				content: options.menuContent || buildMenuItems(options.schema).fullMenu,
			}),
		);
	if (options.history !== false) plugins.push(history());

	return plugins.concat(
		new Plugin({
			props: {
				attributes: { class: 'ProseMirror-example-setup-style' },
			},
		}),
	);
}
