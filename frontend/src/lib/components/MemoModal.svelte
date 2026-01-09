<script lang="ts">
	import type { CreateMemoRequest, MemoType } from '$lib/types';
	import TagInput from './TagInput.svelte';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsave: (data: CreateMemoRequest) => Promise<void>;
	}

	let { open = $bindable(false), onclose, onsave }: Props = $props();

	let content = $state('');
	let memoType = $state<MemoType>('flash');
	let fromDate = $state('');
	let untilDate = $state('');
	let tags = $state<string[]>([]);
	let saving = $state(false);
	let error = $state('');

	function reset() {
		content = '';
		memoType = 'flash';
		fromDate = '';
		untilDate = '';
		tags = [];
		error = '';
	}

	function handleClose() {
		reset();
		onclose();
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		
		if (!content.trim()) {
			error = 'Content is required';
			return;
		}

		saving = true;
		error = '';

		try {
			await onsave({
				content: content.trim(),
				type: memoType,
				from: fromDate || undefined,
				until: untilDate || undefined,
				tags: tags.length > 0 ? tags : undefined
			});
			reset();
			onclose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save memo';
		} finally {
			saving = false;
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}
</script>

{#if open}
	<div class="modal-backdrop" onclick={handleBackdropClick} onkeydown={handleKeydown} role="dialog" aria-modal="true" tabindex="-1">
		<div class="modal">
			<header class="modal-header">
				<h2>New Memo</h2>
				<button type="button" class="close-btn" onclick={handleClose} aria-label="Close">
					&times;
				</button>
			</header>

			<form class="modal-body" onsubmit={handleSubmit}>
				{#if error}
					<div class="error-message">{error}</div>
				{/if}

				<div class="form-group">
					<label for="content">Content</label>
					<textarea
						id="content"
						bind:value={content}
						placeholder="What's on your mind?"
						rows="4"
						required
					></textarea>
				</div>

				<div class="form-group">
					<span class="label-text">Type</span>
					<div class="type-toggle" role="radiogroup" aria-label="Memo type">
						<button
							type="button"
							class="type-btn"
							class:active={memoType === 'flash'}
							onclick={() => memoType = 'flash'}
						>
							Flash
						</button>
						<button
							type="button"
							class="type-btn"
							class:active={memoType === 'permanent'}
							onclick={() => memoType = 'permanent'}
						>
							Permanent
						</button>
					</div>
					<p class="type-hint">
						{memoType === 'flash' ? 'Temporary memo - will be archived after date expires' : 'Permanent memo - stays forever'}
					</p>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="from">From Date</label>
						<input type="date" id="from" bind:value={fromDate} />
					</div>
					<div class="form-group">
						<label for="until">Until Date</label>
						<input type="date" id="until" bind:value={untilDate} />
					</div>
				</div>

				<div class="form-group">
					<span class="label-text" id="tags-label">Tags</span>
					<TagInput bind:tags placeholder="Add tags (comma to confirm)" />
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={handleClose} disabled={saving}>
						Cancel
					</button>
					<button type="submit" class="btn-primary" disabled={saving}>
						{saving ? 'Saving...' : 'Save Memo'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style lang="scss">
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		z-index: 100;
	}

	.modal {
		background: var(--color-background);
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--color-border);

		h2 {
			margin: 0;
			font-size: 1.25rem;
		}
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 1.5rem;
		color: var(--color-text-secondary);
		cursor: pointer;
		padding: 0.25rem;
		line-height: 1;

		&:hover {
			color: var(--color-text);
		}
	}

	.modal-body {
		padding: 1.5rem;
	}

	.error-message {
		padding: 0.75rem;
		background: #fef2f2;
		color: #dc2626;
		border-radius: 8px;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;

		label,
		.label-text {
			display: block;
			font-weight: 500;
			margin-bottom: 0.5rem;
			font-size: 0.875rem;
			color: var(--color-text-secondary);
		}
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	textarea,
	input[type='date'] {
		width: 100%;
		padding: 0.75rem;
		border: 2px solid var(--color-border);
		border-radius: 8px;
		font-size: 1rem;
		resize: vertical;

		&:focus {
			outline: none;
			border-color: var(--color-primary);
		}
	}

	.type-toggle {
		display: flex;
		gap: 0.5rem;
	}

	.type-btn {
		flex: 1;
		padding: 0.5rem 1rem;
		border: 2px solid var(--color-border);
		background: var(--color-background);
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.2s;

		&:hover {
			border-color: var(--color-primary);
		}

		&.active {
			background: var(--color-primary);
			color: white;
			border-color: var(--color-primary);
		}
	}

	.type-hint {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--color-text-light);
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
		margin-top: 1.5rem;
	}

	.btn-primary,
	.btn-secondary {
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;

		&:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}
	}

	.btn-primary {
		background: var(--color-primary);
		color: white;
		border: none;

		&:hover:not(:disabled) {
			background: var(--color-primary-dark);
		}
	}

	.btn-secondary {
		background: var(--color-background);
		color: var(--color-text);
		border: 1px solid var(--color-border);

		&:hover:not(:disabled) {
			background: var(--color-surface);
		}
	}
</style>
