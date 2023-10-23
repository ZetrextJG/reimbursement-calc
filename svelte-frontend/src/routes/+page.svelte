<script lang="ts">
	import { getCategories, estimateItem, createClaim } from '$lib/api';
	import type { Category } from '$lib/models';
	import { currentUser } from '$lib/stores';
	import {
		Autocomplete,
		popup,
		type AutocompleteOption,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let categoryOptions: AutocompleteOption<string>[] = [];
	async function updateCategories() {
		let categories = await getCategories();
		categoryOptions = categories.map((cat) => ({
			label: cat.name,
			value: cat.name,
			meta: cat
		}));
	}
	onMount(updateCategories);

	let selectedCategoryLabel = '';
	let selectedCategory: Category | null = null;
	function onCatSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		selectedCategoryLabel = event.detail.label;
		selectedCategory = event.detail.meta as Category;
	}
	let currentCost = 0;

	type DisplayItem = {
		categoryId: number;
		category: string;
		cost: number;
		estimatedReimbursement: number;
	};
	let displayItems: DisplayItem[] = [];

	$: totalCost = displayItems.reduce((acc, item) => acc + item.cost, 0);
	$: totalReimbursement = displayItems.reduce((acc, item) => acc + item.estimatedReimbursement, 0);
	$: items = displayItems.map((item) => ({
		categoryId: item.categoryId,
		cost: item.cost
	}));

	async function addDisplayItem() {
		if (!selectedCategory) return;
		let estimatedReimbursement = await estimateItem(selectedCategory.id, currentCost);
		displayItems.push({
			categoryId: selectedCategory.id,
			category: selectedCategory.name,
			cost: currentCost,
			estimatedReimbursement
		});
		// HACK: To make it reactive
		displayItems = [...displayItems];
		updateCategories();
	}

	async function createNewClaim() {
		if (!$currentUser) return;
		let creationSuccesful = await createClaim($currentUser.id, items);
		if (creationSuccesful) {
			alert('Claim created successfully');
			displayItems = [];
			currentCost = 0;
			selectedCategoryLabel = '';
			selectedCategory = null;
		} else {
			alert('Claim creation failed');
		}
	}

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom'
	};
</script>

<div class="container h-full mx-auto flex justify-center items-start">
	<div class="flex flex-col bg-surface-800 p-5 w-full">
		<div class="flex flex-col justify-start w-full">
			<form
				id="inner"
				on:submit|preventDefault={addDisplayItem}
				class="flex flex-col md:flex-row justify-start items-center w-full"
			>
				<label for="category" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Category name</span>
					<input
						id="category"
						class="input autocomplete"
						type="search"
						name="autocomplete-search"
						bind:value={selectedCategoryLabel}
						placeholder="Search..."
						use:popup={popupSettings}
					/>
					<div
						class="bg-surface-900 rounded-2xl w-full max-w-sm max-h-48 p-4 overflow-y-scroll z-30"
						data-popup="popupAutocomplete"
						tabindex="-1"
					>
						<Autocomplete
							bind:input={selectedCategoryLabel}
							options={categoryOptions}
							on:selection={onCatSelection}
						/>
					</div>
				</label>
				<label for="cost" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3">Cost</span>
					<input id="cost" class="input" type="number" placeholder="0" bind:value={currentCost} />
				</label>
				<label for="button" class="label flex flex-col space-y-3 items-start p-3">
					<span class="text-l pl-3"> &nbsp;</span>
					<button
						type="submit"
						class="btn bg-success-700 px-10"
						disabled={!selectedCategoryLabel || !currentCost}
					>
						Add
					</button>
				</label>
			</form>

			<div class="h-10" />
			<hr class="!border-t-2" />
			<div class="h-10" />

			<!-- <label for="<div class="table-container">
	<!-- Native Table Element -->
			<div class="overflow-y-auto md:h-96">
				<table class="table table-hover">
					<thead class="sticky top-0">
						<tr>
							<th>Id</th>
							<th>Category</th>
							<th>Cost</th>
							<th>Estimated Reimbursement</th>
							<th>Action</th>
						</tr>
					</thead>
					<tbody class="max-h-40 overflow-y-auto w-full">
						{#if displayItems.length === 0}
							<tr>
								<td colspan="5" class="text-center">No items added</td>
							</tr>
						{/if}
						{#each displayItems as row, i}
							<tr>
								<td>{i}</td>
								<td>{row.category}</td>
								<td>{row.cost}</td>
								<td>{row.estimatedReimbursement}</td>
								<td>
									<button
										type="button"
										class="btn bg-error-700"
										on:click={() => {
											displayItems.splice(i, 1);
											// HACK: To make it reactive
											displayItems = [...displayItems];
										}}
									>
										Delete
									</button>
								</td></tr
							>
						{/each}
					</tbody>
					<div class="h-10" />

					<tfoot class="sticky bottom-0">
						<tr class="bg-surface-700">
							<th colspan="2">Calculated Total</th>
							<td>{totalCost}</td>
							<td>{totalReimbursement}</td>
							<td />
						</tr>
					</tfoot>
				</table>
			</div>
		</div>
		<div class="h-7" />
		<hr class="!border-t-2" />
		<div class="h-5" />
		<form on:submit|preventDefault={createNewClaim}>
			<label for="button" class="label flex flex-col space-y-3 items-start p-3">
				<button
					type="submit"
					class="btn bg-secondary-500 px-10"
					disabled={!displayItems.length || $currentUser === null}
				>
					Create Claim
				</button>
			</label>
		</form>
	</div>
</div>
