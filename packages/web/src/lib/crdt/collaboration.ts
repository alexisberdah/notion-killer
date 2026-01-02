import Collaboration from '@tiptap/extension-collaboration';
import CollaborationCursor from '@tiptap/extension-collaboration-cursor';
import type { Extension } from '@tiptap/core';
import type { CRDTDocument } from './document';

export interface CollaborationOptions {
	document: CRDTDocument;
	user?: {
		name: string;
		color: string;
	};
}

/**
 * Create collaboration extensions for the editor
 */
export function createCollaborationExtensions(options: CollaborationOptions): Extension<any, any>[] {
	const { document, user } = options;

	const extensions: Extension<any, any>[] = [
		Collaboration.configure({
			document: document.doc,
			field: 'content'
		})
	];

	// Add cursor extension if user info is provided
	if (user) {
		extensions.push(
			CollaborationCursor.configure({
				provider: null, // We'll add WebSocket provider later
				user: {
					name: user.name,
					color: user.color
				}
			})
		);
	}

	return extensions;
}

/**
 * Generate a random color for cursor
 */
export function generateUserColor(): string {
	const colors = [
		'#FF6B6B', // Red
		'#4ECDC4', // Teal
		'#45B7D1', // Blue
		'#96CEB4', // Green
		'#FFEAA7', // Yellow
		'#DDA0DD', // Plum
		'#98D8C8', // Mint
		'#F7DC6F', // Gold
		'#BB8FCE', // Purple
		'#85C1E9'  // Light Blue
	];
	return colors[Math.floor(Math.random() * colors.length)];
}

/**
 * Get initials from name
 */
export function getInitials(name: string): string {
	return name
		.split(' ')
		.map(part => part[0])
		.join('')
		.toUpperCase()
		.slice(0, 2);
}
