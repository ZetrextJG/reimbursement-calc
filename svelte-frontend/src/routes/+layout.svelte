<script lang="ts">
	import '../app.postcss';

	import { AppShell, AppBar, type PopupSettings } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { onMount } from 'svelte';
	import { popup, Avatar } from '@skeletonlabs/skeleton';
	import { storePopup } from '@skeletonlabs/skeleton';

	import { currentUser, siteMode, SiteMode, alertVisible } from '$lib/stores';
	import { logout, getMe } from '$lib/api';
	import { page } from '$app/stores';

	onMount(async () => {
		currentUser.set(await getMe());
	});

	async function showLogin() {
		siteMode.set(SiteMode.Login);
	}

	const popupClick: PopupSettings = {
		event: 'click',
		target: 'popupUserInfo',
		placement: 'bottom'
	};

	async function logout_user() {
		let logoutSuccessful = await logout();
		if (logoutSuccessful) {
			currentUser.set(null);
		}
	}

	let message = 'Please verify you email address';
	async function toggleAlert() {
		alertVisible.update((val) => !val);
	}

	// Floating UI for Popups
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<AppShell>
	<svelte:fragment slot="header">
		<AppBar
			gridColumns="grid-cols-3"
			slotDefault="place-self-center"
			slotTrail="place-content-end"
			padding="py-2 px-10"
		>
			<svelte:fragment slot="lead">
				<span
					class="text-lg bg-gradient-to-br from-cyan-300 to-blue-500 bg-clip-text text-transparent box-decoration-clone"
				>
					Re-Calc
				</span>
			</svelte:fragment>
			<div class="hidden w-full md:block md:w-auto" id="navbar-default">
				<ul
					class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg md:flex-row md:space-x-8 md:mt-0 md:border-0"
				>
					<li>
						<a href="/" class="nav-btn" class:active={$page.url.pathname === '/'}> Home </a>
					</li>
					<li>
						<a href="/history" class="nav-btn" class:active={$page.url.pathname === '/history'}>
							History
						</a>
					</li>
					<li>
						<a href="/requests" class="nav-btn" class:active={$page.url.pathname === '/requests'}>
							Requests
						</a>
					</li>
					<li>
						<a href="/items" class="nav-btn" class:active={$page.url.pathname === '/items'}>
							Items
						</a>
					</li>
					<li>
						<a href="/users" class="nav-btn" class:active={$page.url.pathname === '/users'}>
							Users
						</a>
					</li>
				</ul>
			</div>
			<svelte:fragment slot="trail">
				{#if $currentUser === null}
					<Avatar
						on:click={showLogin}
						src="https://api.iconify.design/subway:admin.svg"
						background="bg-surface-200"
						width="w-12"
						border="border-2 border-surface-300-600-token hover:!border-primary-500"
						cursor="cursor-pointer"
					/>
				{:else}
					<div use:popup={popupClick}>
						<Avatar
							initials={$currentUser.username.slice(0, 2)}
							background="bg-primary-300"
							width="w-12"
							border="border-2 border-surface-300-600-token hover:!border-primary-500"
							cursor="cursor-pointer"
						/>
						<div class="bg-surface-700 rounded-lg p-4 w-72 shadow-xl" data-popup="popupUserInfo">
							<div class="arrow bg-surface-500-600-token" />
							<div class="flex flex-col space-y-5 justify-center items-center">
								<p>Logged as {$currentUser?.username}</p>
								<p>Role: {$currentUser?.role}</p>
								<button class="btn bg-error-600" on:click={logout_user}>Log out</button>
							</div>
						</div>
					</div>
				{/if}
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<div class="flex flex-col items-center w-full pt-10">
		{#if $alertVisible}
			<aside class="alert fixed variant-filled-secondary">
				<!-- Icon -->
				<div>ICON</div>
				<!-- Message -->
				<div class="alert-message">
					<h3 class="h3">Warning</h3>
					<p class="text-surface-700">{message}</p>
				</div>
				<div class="w-10" />
				<!-- Actions -->
				<div class="alert-actions">
					<button on:click={toggleAlert} class="btn bg-secondary-50">Close</button>
				</div>
			</aside>
		{/if}
	</div>
	<svelte:fragment slot="pageFooter">
		<div class="flex flex-row justify-end py-2 px-3">
			<p class="text-surface-700">Created By Jakub Grzywaczewski</p>
		</div>
	</svelte:fragment>
	<slot />
</AppShell>

<style lang="postcss">
	.nav-btn {
		@apply block py-2 pl-3 pr-4 text-white bg-transparent rounded  border-0 hover:text-blue-700 md:p-0;
	}
	.active {
		@apply text-blue-500;
	}

	#header {
		z-index: 9;
	}
</style>
