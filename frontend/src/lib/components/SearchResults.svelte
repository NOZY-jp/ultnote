<script lang="ts">
	import type { SearchResult } from '$lib/types';
	import MemoCard from './MemoCard.svelte';

	interface Props {
		results: SearchResult[];
		loading?: boolean;
		onselect?: (memo: SearchResult) => void;
	}

	let { results, loading = false, onselect }: Props = $props();
</script>

<div class="search-results">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Searching...</p>
		</div>
	{:else if results.length === 0}
		<div class="empty">
			<p>No memos found</p>
		</div>
	{:else}
		<div class="results-grid">
			{#each results as memo (memo.id)}
				<MemoCard {memo} onclick={() => onselect?.(memo)} />
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.search-results {
		min-height: 200px;
	}

	.loading,
	.empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		color: var(--color-text-secondary);
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--color-border);
		border-top-color: var(--color-primary);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.results-grid {
		display: grid;
		gap: 1rem;
		grid-template-columns: 1fr;

		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}

		@media (min-width: 1024px) {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
