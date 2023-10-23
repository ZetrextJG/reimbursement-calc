<script lang="ts">
	import { getMyClaims } from '$lib/api';
	import type { Claim } from '$lib/models';
	import { onMount } from 'svelte';

	let claims: Claim[] = [];
	onMount(async () => {
		claims = await getMyClaims();
	});
</script>

<div class="container h-full mx-auto flex justify-center items-start w-3/5">
	<!-- search the user here -->
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<div class="overflow-y-auto md:max-h-96">
			<!-- Native Table Element -->
			<table class="table table-hover">
				<thead>
					<tr>
						<th role="button">ID</th>
						<th role="button">Cost</th>
						<th role="button">Reimbursement</th>
						<th role="button">Status</th>
					</tr>
				</thead>
				<tbody>
					{#if claims.length === 0}
						<tr>
							<td colspan="4" class="text-center">No claims added</td>
						</tr>
					{/if}
					{#each claims as claim, i}
						<tr>
							<td>{claim.id}</td>
							<td>${claim.totalCost}</td>
							<td>${claim.reimbursement}</td>
							{#if claim.status === 'Pending'}
								<td class="text-blue-400">
									{claim.status}
								</td>
							{:else if claim.status === 'Approved'}
								<td class="text-green-400">
									{claim.status}
								</td>
							{:else if claim.status === 'Denied'}
								<td class="text-red-400">
									{claim.status}
								</td>
							{/if}
						</tr>
					{/each}
				</tbody>
				<!-- <tfoot> -->
				<!-- 	<tr> -->
				<!-- 		<th colspan="3">Calculated Total Weight</th> -->
				<!-- 		<td>{totalWeight}</td> -->
				<!-- 	</tr> -->
				<!-- </tfoot> -->
			</table>
		</div>
	</div>
</div>
