<script lang="ts">
	import { SiteMode, siteMode } from '$lib/stores';
	import { signup } from '$lib/api';
	import { alertVisible } from '$lib/stores';
	import '../app.postcss';

	interface SignupForm {
		username: string;
		email: string;
		password: string;
		repPassword: string;
	}

	function validateEmail(email: string) {
		const re = /\S+@\S+\.\S+/;
		return re.test(email);
	}

	function getPasswordViolation(password: string) {
		const hasNumber = /\d/;
		const hasUpper = /[A-Z]/;
		const hasLower = /[a-z]/;
		const hasSpecial = /[!@#$%^&*(),.?":{}|<>]/;

		if (password.length < 8) {
			return 'Password must be at least 8 characters long';
		}
		if (!hasNumber.test(password)) {
			return 'Password must contain at least one number';
		}
		if (!hasUpper.test(password)) {
			return 'Password must contain at least one uppercase letter';
		}
		if (!hasLower.test(password)) {
			return 'Password must contain at least one lowercase letter';
		}
		if (!hasSpecial.test(password)) {
			return 'Password must contain at least one special character';
		}
		return null;
	}

	function signupPossible(form: SignupForm) {
		return (
			form.username !== '' &&
			form.email !== '' &&
			form.password !== '' &&
			form.repPassword !== '' &&
			form.password === form.repPassword &&
			getPasswordViolation(form.password) === null
		);
	}

	let currentForm: SignupForm = {
		username: '',
		email: '',
		password: '',
		repPassword: ''
	};

	$: validEmail = validateEmail(currentForm.email);
	$: passwordViolations = getPasswordViolation(currentForm.password);
	$: validRepPassword = currentForm.password === currentForm.repPassword;
	$: validSignup = signupPossible(currentForm);

	function swtichToLogin() {
		siteMode.set(SiteMode.Login);
	}

	function swtichToDefault() {
		siteMode.set(SiteMode.Default);
	}

	async function signupUser() {
		let signupSuccessful = await signup(
			currentForm.username,
			currentForm.email,
			currentForm.password
		);
		if (signupSuccessful) {
			swtichToDefault();
			alertVisible.set(true);
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
		<form
			on:submit|preventDefault={signupUser}
			class="flex flex-col items-center space-y-10 bg-surface-800 bg-opacity-90 rounded-xl p-10 z-20"
		>
			<h1 class="text-4xl">Sign Up</h1>
			<div class="grid grid-cols-1 md:grid-cols-2">
				<label for="username" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Username</span>
					<input
						id="username"
						bind:value={currentForm.username}
						class="input"
						type="text"
						placeholder="janek"
					/>
				</label>
				<label for="email" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Email</span>
					<input
						id="email"
						bind:value={currentForm.email}
						class:validEmail
						class:input-warning={currentForm.email !== '' && !validEmail}
						class:input-success={currentForm.email !== '' && validEmail}
						class="input"
						type="email"
						placeholder="jan.kowalski@gmail.com"
					/>
				</label>
				<label for="password" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Password</span>
					<input
						id="password"
						bind:value={currentForm.password}
						class="input"
						class:passwordViolations
						class:input-warning={currentForm.password !== '' && passwordViolations !== null}
						class:input-success={currentForm.password !== '' && passwordViolations === null}
						data-tooltip={passwordViolations}
						type="password"
						placeholder="*****"
					/>
				</label>
				<label for="repPassword" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Repeat password</span>
					<input
						id="repPassword"
						bind:value={currentForm.repPassword}
						class="input"
						class:validRepPassword
						class:input-warning={currentForm.repPassword !== '' && !validRepPassword}
						class:input-success={currentForm.repPassword !== '' && validRepPassword}
						type="password"
						placeholder="***"
					/>
				</label>
			</div>
			<div class="flex flex-row items-center justify-around w-full">
				<div class="flex flex-row justify-center items-center">
					<button
						on:click={swtichToDefault}
						type="button"
						class="btn md:px-12 md:py-3 px-8 py-4 bg-surface-500">Cancel</button
					>
				</div>
				<div class="flex flex-row justify-center items-center">
					<button
						class:validSignup
						disabled={!validSignup}
						type="submit"
						class="btn md:px-12 md:py-3 px-8 py-4 bg-secondary-600">Sign Up</button
					>
				</div>
			</div>
			<div class="flex flex-col items-center justify-center space-y-3">
				<label for="">Already have an account: </label>
				<button on:click={swtichToLogin} type="button" class="btn px-8 py-3 bg-tertiary-600"
					>Login</button
				>
			</div>
		</form>
	</div>
</div>

<style lang="postcss">
</style>
