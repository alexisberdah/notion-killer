import { browser } from '$app/environment';

const API_BASE = '/api/v1';

interface ApiError {
	message: string;
	code: number;
}

class ApiClient {
	private getToken(): string | null {
		if (!browser) return null;
		return localStorage.getItem('accessToken');
	}

	private async request<T>(
		endpoint: string,
		options: RequestInit = {}
	): Promise<T> {
		const token = this.getToken();

		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...options.headers
		};

		if (token) {
			(headers as Record<string, string>)['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(`${API_BASE}${endpoint}`, {
			...options,
			headers
		});

		if (!response.ok) {
			const error: { error: ApiError } = await response.json();
			throw new Error(error.error.message);
		}

		return response.json();
	}

	// Generic methods for pages store
	async get<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint);
	}

	async post<T>(endpoint: string, data: unknown): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}

	async patch<T>(endpoint: string, data: unknown): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'PATCH',
			body: JSON.stringify(data)
		});
	}

	async delete<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'DELETE'
		});
	}

	auth = {
		login: (email: string, password: string) =>
			this.request<{
				user: { id: string; email: string; name: string; avatar_url?: string };
				access_token: string;
				refresh_token: string;
			}>('/auth/login', {
				method: 'POST',
				body: JSON.stringify({ email, password })
			}),

		register: (email: string, password: string, name: string) =>
			this.request<{
				user: { id: string; email: string; name: string; avatar_url?: string };
				access_token: string;
				refresh_token: string;
			}>('/auth/register', {
				method: 'POST',
				body: JSON.stringify({ email, password, name })
			}),

		refresh: (refreshToken: string) =>
			this.request<{
				user: { id: string; email: string; name: string; avatar_url?: string };
				access_token: string;
				refresh_token: string;
			}>('/auth/refresh', {
				method: 'POST',
				body: JSON.stringify({ refresh_token: refreshToken })
			}),

		logout: (refreshToken: string) =>
			this.request<{ message: string }>('/auth/logout', {
				method: 'POST',
				body: JSON.stringify({ refresh_token: refreshToken })
			})
	};

	workspaces = {
		list: () =>
			this.request<
				Array<{
					id: string;
					name: string;
					icon?: string;
					owner_id: string;
					created_at: string;
				}>
			>('/workspaces'),

		create: (name: string, icon?: string) =>
			this.request<{
				id: string;
				name: string;
				icon?: string;
				owner_id: string;
				created_at: string;
			}>('/workspaces', {
				method: 'POST',
				body: JSON.stringify({ name, icon })
			}),

		get: (id: string) =>
			this.request<{
				id: string;
				name: string;
				icon?: string;
				owner_id: string;
				created_at: string;
			}>(`/workspaces/${id}`),

		update: (id: string, data: { name?: string; icon?: string }) =>
			this.request<{
				id: string;
				name: string;
				icon?: string;
				owner_id: string;
				created_at: string;
			}>(`/workspaces/${id}`, {
				method: 'PATCH',
				body: JSON.stringify(data)
			}),

		delete: (id: string) =>
			this.request<{ message: string }>(`/workspaces/${id}`, {
				method: 'DELETE'
			})
	};

	pages = {
		list: (workspaceId: string) =>
			this.request<
				Array<{
					id: string;
					title: string;
					icon?: string;
					parent_id?: string;
					created_at: string;
				}>
			>(`/workspaces/${workspaceId}/pages`),

		create: (workspaceId: string, data: { title?: string; parent_id?: string }) =>
			this.request<{
				id: string;
				title: string;
				icon?: string;
				parent_id?: string;
				created_at: string;
			}>(`/workspaces/${workspaceId}/pages`, {
				method: 'POST',
				body: JSON.stringify(data)
			}),

		get: (id: string) =>
			this.request<{
				id: string;
				title: string;
				icon?: string;
				cover_url?: string;
				parent_id?: string;
				created_at: string;
			}>(`/pages/${id}`),

		update: (id: string, data: { title?: string; icon?: string; cover_url?: string }) =>
			this.request<{
				id: string;
				title: string;
				icon?: string;
				cover_url?: string;
			}>(`/pages/${id}`, {
				method: 'PATCH',
				body: JSON.stringify(data)
			}),

		delete: (id: string) =>
			this.request<{ message: string }>(`/pages/${id}`, {
				method: 'DELETE'
			})
	};
}

export const api = new ApiClient();
