<script lang="ts">
	import { type DatePreset, getDateRangeForPreset } from '$lib/types';

	interface Props {
		activePreset: DatePreset | null;
		onselect: (preset: DatePreset | null, range: { from: string; until: string } | null) => void;
	}

	let { activePreset = $bindable(null), onselect }: Props = $props();

	const presets: { key: DatePreset; label: string }[] = [
		{ key: 'today', label: 'Today' },
		{ key: 'tomorrow', label: 'Tomorrow' },
		{ key: 'this_week', label: 'This Week' },
		{ key: 'next_week', label: 'Next Week' },
		{ key: 'this_month', label: 'This Month' }
	];

	function handleClick(preset: DatePreset) {
		if (activePreset === preset) {
			activePreset = null;
			onselect(null, null);
		} else {
			activePreset = preset;
			onselect(preset, getDateRangeForPreset(preset));
		}
	}
</script>

<div class="date-filters">
	{#each presets as { key, label }}
		<button
			type="button"
			class="filter-btn"
			class:active={activePreset === key}
			onclick={() => handleClick(key)}
		>
			{label}
		</button>
	{/each}
</div>

<style lang="scss">
	.date-filters {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.filter-btn {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		background: var(--color-surface);
		color: var(--color-text);
		border: 1px solid var(--color-border);
		border-radius: 20px;
		cursor: pointer;
		transition: all 0.2s;

		&:hover {
			background: var(--color-primary);
			color: white;
			border-color: var(--color-primary);
		}

		&.active {
			background: var(--color-primary);
			color: white;
			border-color: var(--color-primary);
		}
	}
</style>
