<script lang="ts">
	import { getCategories, createCategory, deleteCategory } from '$lib/api';
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

	async function updateCategories() {
		categories = await getCategories();
	}

	async function createNewCategory() {
		let creationSuccesful = await createCategory(
			nameStart,
			reimbursementPercentage,
			maxReimburstment
		);
		if (creationSuccesful) {
			alert('Category created successfully');
			updateCategories();
		} else {
			alert('Category creation failed');
		}
	}

	$: isPercentageValid = reimbursementPercentage >= 0 && reimbursementPercentage <= 100;
	$: isMaxValid = maxReimburstment > 0;

	$: isViewerAdmin = $currentUser?.role === 'Admin';
	$: visibleCategories = categories.filter((cat) =>
		cat.name.toLowerCase().includes(nameStart.toLowerCase())
	);

	let idSortForward = true;
	let nameSortForward = true;
	let reimbursementSortForward = true;
	let maxSortForward = true;
</script>

<div class="container h-full mx-auto flex justify-center items-start w-3/5">
	<!-- search the user here -->
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<form
			on:submit|preventDefault={createNewCategory}
			class="flex flex-col md:flex-row justify-start items-center w-full"
		>
			<label for="name" class="label flex flex-col space-y-3 items-start p-3">
				<span class="text-l pl-3">Category name</span>
				<input id="name" bind:value={nameStart} class="input" type="text" placeholder="Taxi" />
			</label>

			{#if $currentUser?.role === 'Admin'}
				<span class="divide-black md:divider-vertical h-20" />
				<label for="reimbursementPercentage" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Percentage reimbursed</span>
					<input
						id="reimbursementPercentage"
						bind:value={reimbursementPercentage}
						class="input"
						type="number"
						placeholder="50"
					/>
				</label>
				<label for="maxReimburstment" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Max reimbursement</span>
					<input
						id="maxReimburstment"
						bind:value={maxReimburstment}
						class="input"
						type="number"
						placeholder="100"
					/>
				</label>
				<div class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3"> &nbsp;</span>
					<button
						type="submit"
						disabled={!isMaxValid || !isPercentageValid || !nameStart}
						class="btn bg-success-700">Create</button
					>
				</div>
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
						<th
							role="button"
							on:click={() => {
								idSortForward = !idSortForward;
								if (idSortForward) {
									categories = categories.sort((a, b) => a.id - b.id);
								} else {
									categories = categories.sort((a, b) => b.id - a.id);
								}
							}}>ID</th
						>
						<th
							role="button"
							on:click={() => {
								nameSortForward = !nameSortForward;
								if (nameSortForward) {
									categories = categories.sort((a, b) => a.name.localeCompare(b.name));
								} else {
									categories = categories.sort((a, b) => b.name.localeCompare(a.name));
								}
							}}>Name</th
						>
						<th
							role="button"
							on:click={() => {
								reimbursementSortForward = !reimbursementSortForward;
								if (reimbursementSortForward) {
									categories = categories.sort(
										(a, b) => a.reimbursementPercentage - b.reimbursementPercentage
									);
								} else {
									categories = categories.sort(
										(a, b) => b.reimbursementPercentage - a.reimbursementPercentage
									);
								}
							}}
						>
							Reimburstment</th
						>
						<th
							role="button"
							on:click={() => {
								maxSortForward = !maxSortForward;
								if (maxSortForward) {
									categories = categories.sort((a, b) => a.maxReimburstment - b.maxReimburstment);
								} else {
									categories = categories.sort((a, b) => b.maxReimburstment - a.maxReimburstment);
								}
							}}>Max</th
						>
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
								<td>
									<!-- svelte-ignore a11y-click-events-have-key-events -->
									<!-- svelte-ignore a11y-no-static-element-interactions -->
									<!-- svelte-ignore a11y-missing-attribute -->
									<a
										on:click={() => {
											deleteCategory(cat.id);
											updateCategories();
										}}
										class="cursor-pointer text-error-600">Remove</a
									></td
								>
							{/if}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
