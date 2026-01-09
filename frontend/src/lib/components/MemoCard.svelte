<script lang="ts">
	import type { SearchResult } from '$lib/types';

	interface Props {
		memo: SearchResult;
		onclick?: () => void;
	}

	let { memo, onclick }: Props = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			onclick?.();
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('ja-JP', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getScoreClass(score: number): string {
		if (score >= 0.8) return 'high';
		if (score >= 0.5) return 'medium';
		return 'low';
	}
</script>

<button type="button" class="memo-card" onclick={onclick} onkeydown={handleKeydown}>
	<div class="memo-header">
		<span class="memo-type" class:flash={memo.type === 'flash'}>
			{memo.type === 'flash' ? 'Flash' : 'Permanent'}
		</span>
		<span class="memo-score {getScoreClass(memo.score)}">
			{(memo.score * 100).toFixed(0)}%
		</span>
	</div>

	<p class="memo-content">{memo.content}</p>

	{#if memo.tags.length > 0}
		<div class="memo-tags">
			{#each memo.tags as tag}
				<span class="tag">{tag}</span>
			{/each}
		</div>
	{/if}

	<div class="memo-footer">
		{#if memo.from}
			<span class="memo-date">
				{formatDate(memo.from)}
				{#if memo.until && memo.until !== memo.from}
					- {formatDate(memo.until)}
				{/if}
			</span>
		{/if}
		<span class="memo-added">Added {formatDate(memo.date_added)}</span>
	</div>

	{#if memo.completed}
		<div class="completed-badge">Completed</div>
	{/if}
</button>

<style lang="scss">
	.memo-card {
		position: relative;
		padding: 1rem;
		background: var(--color-background);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		cursor: pointer;
		transition: box-shadow 0.2s, transform 0.2s;

		&:hover {
			box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
			transform: translateY(-2px);
		}

		&:focus {
			outline: 2px solid var(--color-primary);
			outline-offset: 2px;
		}
	}

	.memo-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.memo-type {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		color: var(--color-text-secondary);
		padding: 0.125rem 0.5rem;
		background: var(--color-surface);
		border-radius: 4px;

		&.flash {
			color: #f59e0b;
			background: #fef3c7;
		}
	}

	.memo-score {
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;

		&.high {
			color: #059669;
			background: #d1fae5;
		}
		&.medium {
			color: #d97706;
			background: #fef3c7;
		}
		&.low {
			color: #6b7280;
			background: #f3f4f6;
		}
	}

	.memo-content {
		font-size: 1rem;
		line-height: 1.5;
		color: var(--color-text);
		margin-bottom: 0.75rem;
		text-align: left;
		display: -webkit-box;
		line-clamp: 3;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.memo-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		margin-bottom: 0.75rem;
	}

	.tag {
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		background: var(--color-primary-light);
		color: white;
		border-radius: 9999px;
	}

	.memo-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.75rem;
		color: var(--color-text-light);
	}

	.memo-date {
		font-weight: 500;
		color: var(--color-text-secondary);
	}

	.completed-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
		padding: 0.125rem 0.375rem;
		background: #059669;
		color: white;
		border-radius: 4px;
	}
</style>
