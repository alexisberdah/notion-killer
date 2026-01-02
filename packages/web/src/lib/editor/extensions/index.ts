import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import Highlight from '@tiptap/extension-highlight';
import TextAlign from '@tiptap/extension-text-align';
import Underline from '@tiptap/extension-underline';
import Typography from '@tiptap/extension-typography';
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight';
import { common, createLowlight } from 'lowlight';
import type { Extensions } from '@tiptap/core';

const lowlight = createLowlight(common);

export interface EditorExtensionsOptions {
	placeholder?: string;
	collaboration?: boolean;
}

export function createExtensions(options: EditorExtensionsOptions = {}): Extensions {
	const { placeholder = "Type '/' for commands..." } = options;

	return [
		StarterKit.configure({
			codeBlock: false, // Using CodeBlockLowlight instead
			heading: {
				levels: [1, 2, 3]
			}
		}),
		Placeholder.configure({
			placeholder,
			emptyEditorClass: 'is-editor-empty'
		}),
		Link.configure({
			openOnClick: false,
			HTMLAttributes: {
				class: 'text-primary-600 hover:underline cursor-pointer'
			}
		}),
		Image.configure({
			allowBase64: true,
			HTMLAttributes: {
				class: 'rounded-lg max-w-full'
			}
		}),
		TaskList.configure({
			HTMLAttributes: {
				class: 'task-list'
			}
		}),
		TaskItem.configure({
			nested: true,
			HTMLAttributes: {
				class: 'task-item'
			}
		}),
		Highlight.configure({
			multicolor: true
		}),
		TextAlign.configure({
			types: ['heading', 'paragraph']
		}),
		Underline,
		Typography,
		CodeBlockLowlight.configure({
			lowlight,
			HTMLAttributes: {
				class: 'code-block'
			}
		})
	];
}

export { lowlight };
