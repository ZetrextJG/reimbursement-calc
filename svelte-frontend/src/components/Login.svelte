<script lang="ts">
	import '../app.postcss';

	import { currentUser, siteMode, SiteMode } from '$lib/stores';
	import { login, getMe } from '$lib/api';

	let username = '';
	let password = '';
	let invalidLogin = false;

	$: username, (invalidLogin = false);
	$: password, (invalidLogin = false);

	function swtichToSignup() {
		siteMode.set(SiteMode.SignUp);
	}

	function swtichToDefault() {
		siteMode.set(SiteMode.Default);
	}

	async function loginUser() {
		let loginSuccessful = await login(username, password);
		if (loginSuccessful) {
			currentUser.set(await getMe());
			swtichToDefault();
		} else {
			invalidLogin = true;
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	role="button"
	tabindex="-1"
	on:click={swtichToDefault}
	class="fixed top-0 left-0 h-full w-screen bg-opacity-80 bg-black z-10"
/>
<div class="absolute top-20 w-full justify-center items-center">
	<div class="flex w-full h-full py-3 px-3 flex-col justify-center items-center align-middle">
		<div
			class="flex flex-col items-center space-y-10 bg-surface-800 bg-opacity-90 rounded-xl p-10 z-20"
		>
			<h1 class="text-4xl">Login</h1>
			{#if invalidLogin}
				<p class="text-error-500">Invalid username or password</p>
			{/if}
			<form on:submit|preventDefault={loginUser} class="grid grid-cols-1">
				<label for="username" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Username</span>
					<input
						id="username"
						bind:value={username}
						class="input"
						type="text"
						placeholder="janek"
					/>
				</label>
				<label for="password" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Password</span>
					<input
						id="password"
						bind:value={password}
						class="input"
						type="password"
						placeholder="*****"
					/>
				</label>
				<div class="grid grid-cols-1 md:grid-cols-2 items-center justify-around w-full pt-4">
					<div class="flex flex-row justify-center items-center py-3">
						<button type="button" on:click={swtichToDefault} class="btn px-8 py-4 bg-surface-500"
							>Go back</button
						>
					</div>
					<div class="flex flex-row justify-center items-center py-3">
						<button type="submit" class="btn px-8 py-4 bg-secondary-600">Login</button>
					</div>
				</div>
			</form>
			<div class="flex flex-col items-center justify-center space-y-3">
				<label for="">If you don't have an account: </label>
				<button type="button" on:click={swtichToSignup} class="btn px-8 py-3 bg-tertiary-600"
					>Sign Up</button
				>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">
</style>
