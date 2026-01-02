import { Extension } from '@tiptap/core';
import { PluginKey, Plugin } from '@tiptap/pm/state';
import { Decoration, DecorationSet } from '@tiptap/pm/view';
import Suggestion, { type SuggestionOptions } from '@tiptap/suggestion';

export interface SlashCommand {
	title: string;
	description: string;
	icon: string;
	command: (props: { editor: any; range: any }) => void;
	aliases?: string[];
	category: 'basic' | 'media' | 'advanced' | 'database';
}

export const slashCommands: SlashCommand[] = [
	// Basic blocks
	{
		title: 'Text',
		description: 'Just start writing with plain text.',
		icon: 'T',
		category: 'basic',
		aliases: ['paragraph', 'p'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).setParagraph().run();
		}
	},
	{
		title: 'Heading 1',
		description: 'Large section heading.',
		icon: 'H1',
		category: 'basic',
		aliases: ['h1', 'title'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).setHeading({ level: 1 }).run();
		}
	},
	{
		title: 'Heading 2',
		description: 'Medium section heading.',
		icon: 'H2',
		category: 'basic',
		aliases: ['h2', 'subtitle'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).setHeading({ level: 2 }).run();
		}
	},
	{
		title: 'Heading 3',
		description: 'Small section heading.',
		icon: 'H3',
		category: 'basic',
		aliases: ['h3'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).setHeading({ level: 3 }).run();
		}
	},
	{
		title: 'Bullet List',
		description: 'Create a simple bulleted list.',
		icon: '•',
		category: 'basic',
		aliases: ['ul', 'unordered', 'list'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).toggleBulletList().run();
		}
	},
	{
		title: 'Numbered List',
		description: 'Create a list with numbering.',
		icon: '1.',
		category: 'basic',
		aliases: ['ol', 'ordered', 'numbered'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).toggleOrderedList().run();
		}
	},
	{
		title: 'To-do List',
		description: 'Track tasks with a to-do list.',
		icon: '☐',
		category: 'basic',
		aliases: ['todo', 'task', 'checkbox'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).toggleTaskList().run();
		}
	},
	{
		title: 'Quote',
		description: 'Capture a quote.',
		icon: '"',
		category: 'basic',
		aliases: ['blockquote', 'q'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).toggleBlockquote().run();
		}
	},
	{
		title: 'Divider',
		description: 'Visually divide blocks.',
		icon: '—',
		category: 'basic',
		aliases: ['hr', 'line', 'separator'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).setHorizontalRule().run();
		}
	},
	{
		title: 'Code Block',
		description: 'Capture a code snippet.',
		icon: '</>',
		category: 'basic',
		aliases: ['code', 'pre', 'codeblock'],
		command: ({ editor, range }) => {
			editor.chain().focus().deleteRange(range).toggleCodeBlock().run();
		}
	},
	// Media blocks
	{
		title: 'Image',
		description: 'Upload or embed an image.',
		icon: '🖼️',
		category: 'media',
		aliases: ['img', 'picture', 'photo'],
		command: ({ editor, range }) => {
			// TODO: Open image upload modal
			const url = window.prompt('Enter image URL:');
			if (url) {
				editor.chain().focus().deleteRange(range).setImage({ src: url }).run();
			}
		}
	}
];

export interface SlashCommandsOptions {
	suggestion: Partial<SuggestionOptions>;
}

export const SlashCommands = Extension.create<SlashCommandsOptions>({
	name: 'slashCommands',

	addOptions() {
		return {
			suggestion: {
				char: '/',
				startOfLine: false,
				command: ({ editor, range, props }: { editor: any; range: any; props: any }) => {
					props.command({ editor, range });
				}
			}
		};
	},

	addProseMirrorPlugins() {
		return [
			Suggestion({
				editor: this.editor,
				...this.options.suggestion
			})
		];
	}
});

export function filterCommands(query: string): SlashCommand[] {
	const lowerQuery = query.toLowerCase();
	return slashCommands.filter((cmd) => {
		if (cmd.title.toLowerCase().includes(lowerQuery)) return true;
		if (cmd.description.toLowerCase().includes(lowerQuery)) return true;
		if (cmd.aliases?.some((alias) => alias.toLowerCase().includes(lowerQuery))) return true;
		return false;
	});
}

export function groupCommandsByCategory(commands: SlashCommand[]): Map<string, SlashCommand[]> {
	const groups = new Map<string, SlashCommand[]>();

	for (const cmd of commands) {
		const existing = groups.get(cmd.category) || [];
		existing.push(cmd);
		groups.set(cmd.category, existing);
	}

	return groups;
}

export const categoryLabels: Record<string, string> = {
	basic: 'Basic blocks',
	media: 'Media',
	advanced: 'Advanced blocks',
	database: 'Database'
};
