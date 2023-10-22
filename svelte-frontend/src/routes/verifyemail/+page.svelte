<script lang="ts">
	import { verifyEmail } from '$lib/api';
	import { onMount } from 'svelte';
	let message = 'Veryfing your email...';

	onMount(async () => {
		let params = new URLSearchParams(window.location.search);
		let verificationCode = params.get('code');

		if (verificationCode) {
			if (await verifyEmail(verificationCode)) {
				message = 'Your email has been verified';
			} else {
				message = 'Your email could not be verified';
			}
		} else {
			message = 'Missing verification code';
		}
	});
</script>

<div class="flex flex-col justify-center p-10">
	<h1 class="text-3xl font-bold text-center mb-5">
		{message}
	</h1>
	<div class="flex justify-center">
		<a href="/" class="btn variant-filled"> Go to Home </a>
	</div>
</div>
