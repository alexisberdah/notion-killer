// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		interface Locals {
			user: {
				id: string;
				email: string;
				name: string;
			} | null;
		}
		interface PageData {
			user?: {
				id: string;
				email: string;
				name: string;
				avatar_url?: string;
			};
		}
	}
}

export {};
