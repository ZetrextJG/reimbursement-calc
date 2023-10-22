<script lang="ts">
	import { getUsers } from '$lib/api';
	import type { User } from '$lib/models';
	import { currentUser } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let users: User[] = [];
	onMount(async () => {
		users = await getUsers('');
	});

	let usernameStart = '';
	async function updateUsers() {
		users = await getUsers(usernameStart);
	}
	$: usernameStart, updateUsers();

	$: isViewerAdmin = $currentUser?.role === 'Admin';
</script>

<div class="container h-full mx-auto flex justify-center items-center w-3/5">
	<!-- search the user here -->
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<label for="username" class="label flex flex-col space-y-3 items-start p-3">
			<span class="text-l pl-3">Username</span>
			<input
				id="username"
				bind:value={usernameStart}
				class="input"
				type="text"
				placeholder="user"
			/>
		</label>
		<div class="h-10" />
		<hr class="!border-t-2" />
		<div class="h-10" />
		<h1>Users</h1>
		<div class="h-4" />
		<div class="flex flex-col space-y-3 p-3 max-h-[40%] overflow-auto">
			{#each users as user, index}
				<div
					class="flex flex-col md:flex-row items-center md:justify-between w-full border rounded-2xl border-surface-500 p-5"
				>
					<div class="flex flex-row items-center space-x-2 p-2">
						<Avatar initials={user.username.slice(0, 2)} class="w-11 h-11" alt={user.username} />
						{#if user.role === 'admin'}
							<p class="md:text-xl text-red-700">{user.role}</p>
						{:else if user.role == 'manager'}
							<p class="md:text-xl text-orange-300">{user.role}</p>
						{:else}
							<p class="md:text-xl text-gray-300">{user.role}</p>
						{/if}
						<p class="md:text-xl">{user.username}</p>
						{#if user.username == $currentUser?.username}
							<p class="md:text-xl text-green-500">You</p>
						{/if}
					</div>
					<div class="w-3 h-3" />
					<div class="flex flex-col lg:flex-row p-2 items-center">
						<button
							disabled={!isViewerAdmin ||
								user.role == 'Manager' ||
								user.role == 'Admin' ||
								user.username == $currentUser?.username}
							type="button"
							class="btn bg-secondary-800">Make manager</button
						>
						<div class="w-5 h-5" />
						<button
							disabled={!isViewerAdmin ||
								user.role == 'Admin' ||
								user.username == $currentUser?.username}
							type="button"
							class="btn bg-tertiary-800">Make admin</button
						>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
