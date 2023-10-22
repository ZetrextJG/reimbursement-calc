<script lang="ts">
	import { getCategories } from '$lib/api';
	import type { Category } from '$lib/models';
	import { currentUser } from '$lib/stores';
	import { onMount } from 'svelte';

	let categories: Category[] = [];
	onMount(async () => {
		categories = await getCategories();
	});

	let nameStart = '';
	let reimbursementPercentage = 0;
	let maxReimburstment = 0;

	$: isViewerAdmin = $currentUser?.role === 'Admin';
	$: visibleCategories = categories.filter((cat) =>
		cat.name.toLowerCase().includes(nameStart.toLowerCase())
	);

	function createNewCategory() {
		visibleCategories = categories.filter((cat) =>
			cat.name.toLowerCase().includes(nameStart.toLowerCase())
		);
	}
</script>

<div class="container h-full mx-auto flex justify-center items-center w-3/5">
	<!-- search the user here -->
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<form
			on:submit|preventDefault={createNewCategory}
			class="flex flex-col md:flex-row justify-start items-center w-full"
		>
			<label for="username" class="label flex flex-col space-y-3 items-start p-3">
				<span class="text-l pl-3">Category name</span>
				<input id="name" bind:value={nameStart} class="input" type="text" placeholder="taxi" />
			</label>

			{#if $currentUser?.role === 'Admin'}
				<span class="divide-black md:divider-vertical h-20" />
				<label for="username" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Percentage reimbursed</span>
					<input id="name" bind:value={nameStart} class="input" type="number" placeholder="50" />
				</label>
				<label for="username" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Max reimbursement</span>
					<input id="name" bind:value={nameStart} class="input" type="number" placeholder="taxi" />
				</label>
				<label for="button" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3"> &nbsp;</span>
					<button type="submit" class="btn bg-success-700">Create</button>
				</label>
			{/if}
		</form>
		<div class="h-10" />
		<hr class="!border-t-2" />
		<div class="h-10" />
		<div class="table-container">
			<!-- Native Table Element -->
			<table class="table table-hover">
				<thead>
					<tr>
						<th>ID</th>
						<th>Name</th>
						<th>Reimburstment</th>
						<th>Max</th>
						{#if isViewerAdmin}
							<th>Actions</th>
						{/if}
					</tr>
				</thead>
				<tbody>
					{#each visibleCategories as cat, i}
						<tr>
							<td>{cat.id}</td>
							<td>{cat.name}</td>
							<td>{cat.reimbursementPercentage}%</td>
							<td>${cat.maxReimburstment}</td>
							{#if isViewerAdmin}
								<td><a href="/" on:click={() => {}} class="text-error-600">Remove</a></td>
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
