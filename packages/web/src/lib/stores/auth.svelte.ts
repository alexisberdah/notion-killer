import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { api } from '$lib/api';

interface User {
	id: string;
	email: string;
	name: string;
	avatar_url?: string;
}

interface AuthState {
	user: User | null;
	accessToken: string | null;
	isLoading: boolean;
	isAuthenticated: boolean;
}

function createAuthStore() {
	let user = $state<User | null>(null);
	let accessToken = $state<string | null>(null);
	let isLoading = $state(true);

	// Initialize from localStorage
	if (browser) {
		const storedToken = localStorage.getItem('accessToken');
		const storedUser = localStorage.getItem('user');
		if (storedToken && storedUser) {
			accessToken = storedToken;
			user = JSON.parse(storedUser);
		}
		isLoading = false;
	}

	async function login(email: string, password: string): Promise<void> {
		isLoading = true;
		try {
			const response = await api.auth.login(email, password);
			user = response.user;
			accessToken = response.access_token;

			if (browser) {
				localStorage.setItem('accessToken', response.access_token);
				localStorage.setItem('refreshToken', response.refresh_token);
				localStorage.setItem('user', JSON.stringify(response.user));
			}

			await goto('/');
		} finally {
			isLoading = false;
		}
	}

	async function register(email: string, password: string, name: string): Promise<void> {
		isLoading = true;
		try {
			const response = await api.auth.register(email, password, name);
			user = response.user;
			accessToken = response.access_token;

			if (browser) {
				localStorage.setItem('accessToken', response.access_token);
				localStorage.setItem('refreshToken', response.refresh_token);
				localStorage.setItem('user', JSON.stringify(response.user));
			}

			await goto('/');
		} finally {
			isLoading = false;
		}
	}

	async function logout(): Promise<void> {
		const refreshToken = browser ? localStorage.getItem('refreshToken') : null;
		if (refreshToken) {
			try {
				await api.auth.logout(refreshToken);
			} catch {
				// Ignore logout errors
			}
		}

		user = null;
		accessToken = null;

		if (browser) {
			localStorage.removeItem('accessToken');
			localStorage.removeItem('refreshToken');
			localStorage.removeItem('user');
		}

		await goto('/login');
	}

	async function refreshTokens(): Promise<boolean> {
		const refreshToken = browser ? localStorage.getItem('refreshToken') : null;
		if (!refreshToken) return false;

		try {
			const response = await api.auth.refresh(refreshToken);
			user = response.user;
			accessToken = response.access_token;

			if (browser) {
				localStorage.setItem('accessToken', response.access_token);
				localStorage.setItem('refreshToken', response.refresh_token);
				localStorage.setItem('user', JSON.stringify(response.user));
			}

			return true;
		} catch {
			await logout();
			return false;
		}
	}

	return {
		get user() {
			return user;
		},
		get accessToken() {
			return accessToken;
		},
		get isLoading() {
			return isLoading;
		},
		get isAuthenticated() {
			return !!user && !!accessToken;
		},
		login,
		register,
		logout,
		refreshTokens
	};
}

export const auth = createAuthStore();
