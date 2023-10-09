<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar class="py-0 m-3">
			<svelte:fragment slot="lead">
				<div class="flex flex-row items-center space-x-3 space-y-0">
					<div class="basis-1/2 drop-shadow-md">
						<strong class="text-l uppercase">Reimbursement Calc</strong>
					</div>
					<spacer type="horizontal" width="2em" height="2em" class="px-5"> ♢ </spacer>
					<div class="basis-1/12 w-100 bg-inherit hover:bg-gray-700 py-1">
						<button class="btn text-l w-100"> Home </button>
					</div>
					<div class="basis-1/12 w-100 bg-inherit hover:bg-gray-700 py-1">
						<button class="btn text-l w-100"> History </button>
					</div>
					<div class="basis-1/12 w-100 bg-inherit hover:bg-gray-700 py-1">
						<button class="btn text-l w-100"> Admin </button>
					</div>
				</div>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://discord.gg/EXqV7W8MtY"
					target="_blank"
					rel="noreferrer"
				>
					Discord
				</a>
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://twitter.com/SkeletonUI"
					target="_blank"
					rel="noreferrer"
				>
					Twitter
				</a>
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://github.com/skeletonlabs/skeleton"
					target="_blank"
					rel="noreferrer"
				>
					GitHub
				</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
