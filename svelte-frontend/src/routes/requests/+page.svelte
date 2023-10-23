<script lang="ts">
	import { getPendingClaims, approveClaim } from '$lib/api';
	import { currentUser } from '$lib/stores';
	import type { Claim } from '$lib/models';
	import { onMount } from 'svelte';

	let claims: Claim[] = [];
	let message = 'No pending claims';
	onMount(async () => {
		let tempClaim = await getPendingClaims();
		message = 'Loading...';
		setTimeout(() => {
			claims = tempClaim.filter((claim) => $currentUser?.id && claim.userId !== $currentUser?.id);
			message = 'No pending claims';
		}, 500);
	});
</script>

<div class="container h-full mx-auto flex justify-center items-start w-3/5">
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<div class="overflow-y-auto md:max-h-96">
			<table class="table table-hover">
				<thead>
					<tr>
						<th role="button">ID</th>
						<th role="button">Cost</th>
						<th role="button">Reimbursement</th>
						<th role="button">Status</th>
						<th role="button">Reject</th>
						<th role="button">Approve</th>
					</tr>
				</thead>
				<tbody>
					{#if claims.length === 0}
						<tr>
							<td colspan="6" class="text-center">{message}</td>
						</tr>
					{/if}
					{#each claims as claim, i}
						<tr>
							<td>{claim.id}</td>
							<td>${claim.totalCost}</td>
							<td>${claim.reimbursement}</td>
							<td class="text-blue-400">
								{claim.status}
							</td>
							<td>
								<button
									class="btn bg-error-600"
									on:click={() => {
										approveClaim(claim.id, false);
										claims.splice(i, 1);
										claims = [...claims];
									}}
								>
									Reject
								</button>
							</td>
							<td>
								<button
									class="btn bg-success-600"
									on:click={() => {
										approveClaim(claim.id, true);
										claims.splice(i, 1);
										claims = [...claims];
									}}
									>Accept
								</button></td
							></tr
						>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
