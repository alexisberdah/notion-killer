<script lang="ts">
	import { auth } from '$stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { Button } from '$components/ui';

	onMount(() => {
		// Redirect to login if not authenticated
		if (!auth.isAuthenticated && !auth.isLoading) {
			goto('/login');
		}
	});
</script>

{#if auth.isLoading}
	<div class="flex items-center justify-center min-h-screen">
		<div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-600"></div>
	</div>
{:else if auth.isAuthenticated}
	<div class="min-h-screen flex">
		<!-- Sidebar -->
		<aside class="w-64 border-r bg-surface-50 dark:bg-surface-900 p-4">
			<div class="flex items-center gap-3 mb-6">
				<div
					class="w-8 h-8 rounded bg-primary-600 flex items-center justify-center text-white font-bold"
				>
					N
				</div>
				<span class="font-semibold">Notion Killer</span>
			</div>

			<nav class="space-y-1">
				<a
					href="/"
					class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-surface-100 dark:hover:bg-surface-800"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
						/>
					</svg>
					Home
				</a>
				<a
					href="/settings"
					class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-surface-100 dark:hover:bg-surface-800"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
						/>
					</svg>
					Settings
				</a>
			</nav>

			<div class="absolute bottom-4 left-4 right-4">
				<div class="flex items-center gap-3 p-3 rounded-lg bg-surface-100 dark:bg-surface-800">
					<div
						class="w-8 h-8 rounded-full bg-primary-600 flex items-center justify-center text-white text-sm font-medium"
					>
						{auth.user?.name?.charAt(0).toUpperCase()}
					</div>
					<div class="flex-1 min-w-0">
						<p class="text-sm font-medium truncate">{auth.user?.name}</p>
						<p class="text-xs text-surface-500 truncate">{auth.user?.email}</p>
					</div>
					<button onclick={() => auth.logout()} class="p-1 hover:bg-surface-200 rounded">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
							/>
						</svg>
					</button>
				</div>
			</div>
		</aside>

		<!-- Main content -->
		<main class="flex-1 p-8">
			<div class="max-w-4xl mx-auto">
				<h1 class="text-3xl font-bold mb-6">Welcome, {auth.user?.name}!</h1>

				<div class="card p-6">
					<h2 class="text-xl font-semibold mb-4">Getting Started</h2>
					<p class="text-surface-600 dark:text-surface-400 mb-4">
						Notion Killer is your fast, offline-first workspace. Create pages, organize your
						thoughts, and collaborate in real-time.
					</p>
					<Button>Create your first page</Button>
				</div>
			</div>
		</main>
	</div>
{:else}
	<div class="flex items-center justify-center min-h-screen">
		<p>Redirecting to login...</p>
	</div>
{/if}
