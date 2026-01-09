<script lang="ts">
	import { api } from '$lib/api';
	import type { SearchResult, DatePreset, CreateMemoRequest, SearchFilters } from '$lib/types';
	import DateFilters from '$lib/components/DateFilters.svelte';
	import SearchResults from '$lib/components/SearchResults.svelte';
	import MemoModal from '$lib/components/MemoModal.svelte';

	// State
	let searchQuery = $state('');
	let results = $state<SearchResult[]>([]);
	let loading = $state(false);
	let searched = $state(false);
	let activePreset = $state<DatePreset | null>(null);
	let dateRange = $state<{ from: string; until: string } | null>(null);
	let modalOpen = $state(false);
	let searchError = $state('');

	// Derived
	let hasQuery = $derived(searchQuery.trim().length > 0);
	let showResults = $derived(searched || hasQuery);

	async function handleSearch() {
		if (!searchQuery.trim()) return;

		loading = true;
		searched = true;
		searchError = '';

		try {
			const filters: SearchFilters = {};
			if (dateRange) {
				filters.from_gte = dateRange.from;
				filters.until_lte = dateRange.until;
			}

			const response = await api.search({
				query: searchQuery.trim(),
				filters: Object.keys(filters).length > 0 ? filters : undefined,
				limit: 20
			});
			results = response.results;
		} catch (err) {
			searchError = err instanceof Error ? err.message : 'Search failed';
			results = [];
		} finally {
			loading = false;
		}
	}

	function handleDateSelect(preset: DatePreset | null, range: { from: string; until: string } | null) {
		activePreset = preset;
		dateRange = range;
		// Re-search if there's a query
		if (hasQuery) {
			handleSearch();
		}
	}

	async function handleSaveMemo(data: CreateMemoRequest) {
		await api.createMemo(data);
		// Optionally refresh search results
		if (hasQuery) {
			await handleSearch();
		}
	}

	function handleMemoSelect(memo: SearchResult) {
		// For now, just log - could open detail view
		console.log('Selected memo:', memo);
	}
</script>

<svelte:head>
	<title>UltNote - Semantic Memo Search</title>
</svelte:head>

<div class="home">
	<header class="header">
		<h1>UltNote</h1>
		<p class="tagline">Semantic memo search for your thoughts</p>
	</header>

	<section class="search-section">
		<form class="search-form" onsubmit={(e: SubmitEvent) => { e.preventDefault(); handleSearch(); }}>
			<input
				type="text"
				class="search-input"
				placeholder="Search your memos..."
				bind:value={searchQuery}
			/>
			<button type="submit" class="search-button" disabled={loading || !hasQuery}>
				{loading ? 'Searching...' : 'Search'}
			</button>
		</form>

		<DateFilters
			bind:activePreset
			onselect={handleDateSelect}
		/>
	</section>

	<section class="results-section">
		{#if searchError}
			<div class="error-message">{searchError}</div>
		{:else if showResults}
			<SearchResults
				{results}
				{loading}
				onselect={handleMemoSelect}
			/>
		{:else}
			<div class="placeholder">
				<p class="placeholder-text">Enter a search query to find your memos</p>
				<p class="placeholder-hint">Try searching for concepts like "clinic" to find related memos</p>
			</div>
		{/if}
	</section>

	<button class="fab" aria-label="Add memo" onclick={() => modalOpen = true}>
		<span class="fab-icon">+</span>
	</button>
</div>

<MemoModal
	bind:open={modalOpen}
	onclose={() => modalOpen = false}
	onsave={handleSaveMemo}
/>

<style lang="scss">
	.home {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		padding-bottom: 5rem; /* Space for FAB */
	}

	.header {
		text-align: center;
		padding: 2rem 0;

		h1 {
			font-size: 2.5rem;
			margin: 0;
			color: var(--color-primary);
			font-weight: 700;
		}

		.tagline {
			color: var(--color-text-secondary);
			margin-top: 0.5rem;
			font-size: 1.125rem;
		}
	}

	.search-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.search-form {
		display: flex;
		gap: 0.5rem;
	}

	.search-input {
		flex: 1;
		padding: 0.875rem 1.25rem;
		font-size: 1rem;
		border: 2px solid var(--color-border);
		border-radius: 12px;
		transition: border-color 0.2s, box-shadow 0.2s;

		&:focus {
			outline: none;
			border-color: var(--color-primary);
			box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
		}

		&::placeholder {
			color: var(--color-text-light);
		}
	}

	.search-button {
		padding: 0.875rem 1.5rem;
		font-size: 1rem;
		font-weight: 500;
		background: var(--color-primary);
		color: white;
		border: none;
		border-radius: 12px;
		cursor: pointer;
		transition: background 0.2s, transform 0.2s;
		white-space: nowrap;

		&:hover:not(:disabled) {
			background: var(--color-primary-dark);
		}

		&:active:not(:disabled) {
			transform: scale(0.98);
		}

		&:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}
	}

	.results-section {
		min-height: 200px;
	}

	.error-message {
		padding: 1rem;
		background: #fef2f2;
		color: #dc2626;
		border-radius: 8px;
		text-align: center;
	}

	.placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		text-align: center;
	}

	.placeholder-text {
		color: var(--color-text-secondary);
		font-size: 1.125rem;
	}

	.placeholder-hint {
		color: var(--color-text-light);
		font-size: 0.875rem;
		margin-top: 0.5rem;
	}

	.fab {
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		width: 56px;
		height: 56px;
		border-radius: 50%;
		background: var(--color-primary);
		color: white;
		border: none;
		cursor: pointer;
		box-shadow: 0 4px 12px rgba(79, 70, 229, 0.4);
		transition: transform 0.2s, box-shadow 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;

		&:hover {
			transform: scale(1.1);
			box-shadow: 0 6px 16px rgba(79, 70, 229, 0.5);
		}

		&:active {
			transform: scale(1.05);
		}
	}

	.fab-icon {
		font-size: 1.75rem;
		font-weight: 300;
		line-height: 1;
	}

	@media (max-width: 768px) {
		.header {
			padding: 1.5rem 0;

			h1 {
				font-size: 2rem;
			}

			.tagline {
				font-size: 1rem;
			}
		}

		.search-form {
			flex-direction: column;
		}

		.search-button {
			width: 100%;
		}

		.fab {
			bottom: 1.5rem;
			right: 1.5rem;
		}
	}
</style>
