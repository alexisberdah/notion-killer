import type { Editor, Range } from '@tiptap/core';
import type { SuggestionOptions } from '@tiptap/suggestion';
import { slashCommands, filterCommands, type SlashCommand } from './slash-commands';

export interface SuggestionState {
	isOpen: boolean;
	query: string;
	range: Range | null;
	items: SlashCommand[];
	selectedIndex: number;
	clientRect: (() => DOMRect | null) | null;
	editor: Editor | null;
}

export function createSuggestionState(): SuggestionState {
	return {
		isOpen: false,
		query: '',
		range: null,
		items: slashCommands,
		selectedIndex: 0,
		clientRect: null,
		editor: null
	};
}

export interface CreateSlashSuggestionOptions {
	onStateChange: (state: SuggestionState) => void;
}

export function createSlashSuggestion(options: CreateSlashSuggestionOptions): Partial<SuggestionOptions<SlashCommand>> {
	let state = createSuggestionState();

	function updateState(updates: Partial<SuggestionState>) {
		state = { ...state, ...updates };
		options.onStateChange(state);
	}

	return {
		char: '/',
		startOfLine: false,
		items: ({ query }) => {
			return filterCommands(query);
		},
		render: () => {
			let currentEditor: Editor | null = null;

			return {
				onStart: (props) => {
					currentEditor = props.editor;
					updateState({
						isOpen: true,
						query: props.query,
						range: props.range,
						items: props.items as SlashCommand[],
						selectedIndex: 0,
						clientRect: props.clientRect,
						editor: props.editor
					});
				},
				onUpdate: (props) => {
					currentEditor = props.editor;
					updateState({
						query: props.query,
						range: props.range,
						items: props.items as SlashCommand[],
						selectedIndex: Math.min(state.selectedIndex, (props.items as SlashCommand[]).length - 1),
						clientRect: props.clientRect,
						editor: props.editor
					});
				},
				onKeyDown: ({ event }: { event: KeyboardEvent }) => {
					if (event.key === 'ArrowDown') {
						event.preventDefault();
						const newIndex = Math.min(state.selectedIndex + 1, state.items.length - 1);
						updateState({ selectedIndex: newIndex });
						return true;
					}

					if (event.key === 'ArrowUp') {
						event.preventDefault();
						const newIndex = Math.max(state.selectedIndex - 1, 0);
						updateState({ selectedIndex: newIndex });
						return true;
					}

					if (event.key === 'Enter') {
						event.preventDefault();
						const selectedItem = state.items[state.selectedIndex];
						if (selectedItem && state.range && currentEditor) {
							selectedItem.command({
								editor: currentEditor,
								range: state.range
							});
						}
						updateState({ isOpen: false });
						return true;
					}

					if (event.key === 'Escape') {
						event.preventDefault();
						updateState({ isOpen: false });
						return true;
					}

					return false;
				},
				onExit: () => {
					currentEditor = null;
					updateState({
						isOpen: false,
						query: '',
						range: null,
						items: [],
						selectedIndex: 0,
						clientRect: null,
						editor: null
					});
				}
			};
		},
		command: ({ editor, range, props }) => {
			(props as SlashCommand).command({ editor, range });
		}
	};
}
