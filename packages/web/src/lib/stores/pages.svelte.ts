import { api } from '$lib/api';

export interface Page {
	id: string;
	workspace_id: string;
	parent_id: string | null;
	title: string;
	icon: string | null;
	cover_url: string | null;
	is_database: boolean;
	created_by: string;
	last_edited_by: string | null;
	created_at: string;
	updated_at: string;
}

export interface PageTreeItem {
	id: string;
	parent_id: string | null;
	title: string;
	icon: string | null;
	is_database: boolean;
	children: PageTreeItem[];
}

export interface BreadcrumbItem {
	id: string;
	title: string;
	icon: string | null;
}

function createPagesStore() {
	let pages = $state<Page[]>([]);
	let pageTree = $state<PageTreeItem[]>([]);
	let currentPage = $state<Page | null>(null);
	let breadcrumbs = $state<BreadcrumbItem[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	async function loadPageTree(workspaceId: string) {
		isLoading = true;
		error = null;
		try {
			const response = await api.get<PageTreeItem[]>(`/workspaces/${workspaceId}/page-tree`);
			pageTree = response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load page tree';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function loadPages(workspaceId: string, parentId?: string) {
		isLoading = true;
		error = null;
		try {
			const params = new URLSearchParams({ workspace_id: workspaceId });
			if (parentId) {
				params.append('parent_id', parentId);
			}
			const response = await api.get<Page[]>(`/pages?${params.toString()}`);
			pages = response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load pages';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function loadPage(pageId: string) {
		isLoading = true;
		error = null;
		try {
			const response = await api.get<Page>(`/pages/${pageId}`);
			currentPage = response;
			return response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load page';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function loadBreadcrumbs(pageId: string) {
		try {
			const response = await api.get<BreadcrumbItem[]>(`/pages/${pageId}/breadcrumbs`);
			breadcrumbs = response;
			return response;
		} catch (e) {
			console.error('Failed to load breadcrumbs:', e);
			breadcrumbs = [];
		}
	}

	async function createPage(data: {
		workspace_id: string;
		parent_id?: string;
		title?: string;
		icon?: string;
		is_database?: boolean;
	}) {
		isLoading = true;
		error = null;
		try {
			const response = await api.post<Page>('/pages', data);
			// Refresh page tree
			await loadPageTree(data.workspace_id);
			return response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create page';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function updatePage(
		pageId: string,
		data: {
			title?: string;
			icon?: string;
			cover_url?: string;
			parent_id?: string;
		}
	) {
		error = null;
		try {
			const response = await api.patch<Page>(`/pages/${pageId}`, data);
			if (currentPage?.id === pageId) {
				currentPage = response;
			}
			return response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update page';
			throw e;
		}
	}

	async function deletePage(pageId: string, workspaceId: string) {
		isLoading = true;
		error = null;
		try {
			await api.delete(`/pages/${pageId}`);
			if (currentPage?.id === pageId) {
				currentPage = null;
			}
			// Refresh page tree
			await loadPageTree(workspaceId);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete page';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function duplicatePage(pageId: string, workspaceId: string) {
		isLoading = true;
		error = null;
		try {
			const response = await api.post<Page>(`/pages/${pageId}/duplicate`, {});
			// Refresh page tree
			await loadPageTree(workspaceId);
			return response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to duplicate page';
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function movePage(pageId: string, newParentId: string | null, workspaceId: string) {
		error = null;
		try {
			const response = await api.post<Page>(`/pages/${pageId}/move`, {
				new_parent_id: newParentId
			});
			// Refresh page tree
			await loadPageTree(workspaceId);
			return response;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to move page';
			throw e;
		}
	}

	function clearCurrentPage() {
		currentPage = null;
		breadcrumbs = [];
	}

	return {
		get pages() {
			return pages;
		},
		get pageTree() {
			return pageTree;
		},
		get currentPage() {
			return currentPage;
		},
		get breadcrumbs() {
			return breadcrumbs;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		loadPageTree,
		loadPages,
		loadPage,
		loadBreadcrumbs,
		createPage,
		updatePage,
		deletePage,
		duplicatePage,
		movePage,
		clearCurrentPage
	};
}

export const pagesStore = createPagesStore();
