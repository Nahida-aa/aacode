import { textblockTypeInputRule } from 'prosemirror-inputrules';
import { defaultMarkdownSerializer } from 'prosemirror-markdown';
import { DOMParser, type Node, Schema } from 'prosemirror-model';
import { schema } from 'prosemirror-schema-basic';
import { addListNodes } from 'prosemirror-schema-list';
import type { Transaction } from 'prosemirror-state';
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
- **参数**：`level` - 标题级别（1-6）
- **工作原理**：
  - 匹配正则表达式 `^(#{1,level})\s$`（行首多个 `#` 后跟空格）
  - 自动将输入转换为对应级别的标题
  - 例：输入 `### ` 自动转为三级标题
 */
export function headingRule(level: number) {
	return textblockTypeInputRule(
		new RegExp(`^(#{1,${level}})\\s$`),
		documentSchema.nodes.heading,
		() => ({ level }),
	);
}

/**
 * 代码块快捷规则
 * 匹配 ```javascript 后跟空格，自动转换为代码块并设置语言属性
 */
export function codeBlockRule() {
	return textblockTypeInputRule(
		/^```([a-z]+)?\s$/,
		documentSchema.nodes.code_block,
		(match) => ({ params: match[1] || '' }),
	);
}
