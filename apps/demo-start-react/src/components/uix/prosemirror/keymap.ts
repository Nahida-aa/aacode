import {
	chainCommands,
	exitCode,
	joinDown,
	joinUp,
	lift,
	selectParentNode,
	setBlockType,
	toggleMark,
	wrapIn,
} from 'prosemirror-commands';
import { redo, undo } from 'prosemirror-history';
import { undoInputRule } from 'prosemirror-inputrules';
import type { MarkType, NodeType, Schema } from 'prosemirror-model';
import {
	liftListItem,
	sinkListItem,
	splitListItem,
	wrapInList,
} from 'prosemirror-schema-list';
import type { Command } from 'prosemirror-state';

const mac =
	typeof navigator != 'undefined'
		? /Mac|iP(hone|[oa]d)/.test(navigator.platform)
		: false;

/// Inspect the given schema looking for marks and nodes from the
/// basic schema, and if found, add key bindings related to them.
/// This will add:
///
/// * **Mod-b** for toggling [strong](#schema-basic.StrongMark)
/// * **Mod-i** for toggling [emphasis](#schema-basic.EmMark)
/// * **Mod-`** for toggling [code font](#schema-basic.CodeMark)
/// * **Ctrl-Shift-0** for making the current textblock a paragraph
/// * **Ctrl-Shift-1** to **Ctrl-Shift-Digit6** for making the current
///   textblock a heading of the corresponding level
/// * **Ctrl-Shift-Backslash** to make the current textblock a code block
/// * **Ctrl-Shift-8** to wrap the selection in an ordered list
/// * **Ctrl-Shift-9** to wrap the selection in a bullet list
/// * **Ctrl->** to wrap the selection in a block quote
/// * **Enter** to split a non-empty textblock in a list item while at
///   the same time splitting the list item
/// * **Mod-Enter** to insert a hard break
/// * **Mod-_** to insert a horizontal rule
/// * **Backspace** to undo an input rule
/// * **Alt-ArrowUp** to `joinUp`
/// * **Alt-ArrowDown** to `joinDown`
/// * **Mod-BracketLeft** to `lift`
/// * **Escape** to `selectParentNode`
///
/// You can suppress or map these bindings by passing a `mapKeys`
/// argument, which maps key names (say `"Mod-B"` to either `false`, to
/// remove the binding, or a new key name string.
export function buildKeymap(
	schema: Schema,
	mapKeys?: { [key: string]: false | string },
) {
	const keys: { [key: string]: Command } = {};
	let schemaType: MarkType | NodeType | undefined;
	function bind(key: string, cmd: Command) {
		if (mapKeys) {
			const mapped = mapKeys[key];
			if (mapped === false) return;
			if (mapped) key = mapped;
		}
		keys[key] = cmd;
	}

	bind('Mod-z', undo);
	bind('Shift-Mod-z', redo);
	bind('Backspace', undoInputRule);
	if (!mac) bind('Mod-y', redo);

	bind('Alt-ArrowUp', joinUp);
	bind('Alt-ArrowDown', joinDown);
	bind('Mod-BracketLeft', lift);
	bind('Escape', selectParentNode);
	if (schema.marks.strong) {
		schemaType = schema.marks.strong;
		bind('Mod-b', toggleMark(schemaType));
		bind('Mod-B', toggleMark(schemaType));
	}
	if (schema.marks.em) {
		schemaType = schema.marks.em;
		bind('Mod-i', toggleMark(schemaType));
		bind('Mod-I', toggleMark(schemaType));
	}
	if (schema.marks.code) {
		schemaType = schema.marks.code;
		bind('Mod-`', toggleMark(schemaType));
	}

	if (schema.nodes.bullet_list) {
		schemaType = schema.nodes.bullet_list;
		bind('Shift-Ctrl-8', wrapInList(schemaType));
	}
	if (schema.nodes.ordered_list) {
		schemaType = schema.nodes.ordered_list;
		bind('Shift-Ctrl-9', wrapInList(schemaType));
	}
	if (schema.nodes.blockquote) {
		schemaType = schema.nodes.blockquote;
		bind('Ctrl->', wrapIn(schemaType));
	}
	if (schema.nodes.hard_break) {
		schemaType = schema.nodes.hard_break;
		const br = schemaType,
			cmd = chainCommands(exitCode, (state, dispatch) => {
				if (dispatch)
					dispatch(state.tr.replaceSelectionWith(br.create()).scrollIntoView());
				return true;
			});
		bind('Mod-Enter', cmd);
		bind('Shift-Enter', cmd);
		if (mac) bind('Ctrl-Enter', cmd);
	}
	if (schema.nodes.list_item) {
		schemaType = schema.nodes.list_item;
		bind('Enter', splitListItem(schemaType));
		bind('Mod-[', liftListItem(schemaType));
		bind('Mod-]', sinkListItem(schemaType));
	}
	if (schema.nodes.paragraph) {
		schemaType = schema.nodes.paragraph;
		bind('Shift-Ctrl-0', setBlockType(schemaType));
	}
	if (schema.nodes.code_block) {
		schemaType = schema.nodes.code_block;
		bind('Shift-Ctrl-\\', setBlockType(schemaType));
	}
	if (schema.nodes.heading) {
		schemaType = schema.nodes.heading;
		for (let i = 1; i <= 6; i++)
			bind('Shift-Ctrl-' + i, setBlockType(schemaType, { level: i }));
	}
	if (schema.nodes.horizontal_rule) {
		schemaType = schema.nodes.horizontal_rule;
		const hr = schemaType;
		bind('Mod-_', (state, dispatch) => {
			if (dispatch)
				dispatch(state.tr.replaceSelectionWith(hr.create()).scrollIntoView());
			return true;
		});
	}

	return keys;
}
