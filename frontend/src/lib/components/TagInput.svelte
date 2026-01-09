<script lang="ts">
	interface Props {
		tags: string[];
		placeholder?: string;
		onchange?: (tags: string[]) => void;
	}

	let { tags = $bindable([]), placeholder = 'Add tag...', onchange }: Props = $props();

	let inputValue = $state('');
	let inputRef: HTMLInputElement | undefined = $state();

	function addTag(value: string) {
		const trimmed = value.trim();
		if (trimmed && !tags.includes(trimmed)) {
			tags = [...tags, trimmed];
			onchange?.(tags);
		}
		inputValue = '';
	}

	function removeTag(index: number) {
		tags = tags.filter((_, i) => i !== index);
		onchange?.(tags);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ',') {
			e.preventDefault();
			addTag(inputValue);
		} else if (e.key === 'Backspace' && inputValue === '' && tags.length > 0) {
			removeTag(tags.length - 1);
		}
	}

	function handleBlur() {
		if (inputValue.trim()) {
			addTag(inputValue);
		}
	}

	function focusInput() {
		inputRef?.focus();
	}
</script>

<div class="tag-input" onclick={focusInput} onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter') focusInput(); }} role="group" aria-label="Tag input">
	{#each tags as tag, i}
		<span class="tag">
			{tag}
			<button
				type="button"
				class="tag-remove"
				onclick={(e: MouseEvent) => { e.stopPropagation(); removeTag(i); }}
				aria-label="Remove tag {tag}"
			>
				&times;
			</button>
		</span>
	{/each}
	<input
		bind:this={inputRef}
		bind:value={inputValue}
		type="text"
		class="tag-input-field"
		{placeholder}
		onkeydown={handleKeydown}
		onblur={handleBlur}
	/>
</div>

<style lang="scss">
	.tag-input {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.5rem;
		border: 2px solid var(--color-border);
		border-radius: 8px;
		background: var(--color-background);
		cursor: text;
		min-height: 44px;
		align-items: center;

		&:focus-within {
			border-color: var(--color-primary);
		}
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: var(--color-primary-light);
		color: white;
		border-radius: 9999px;
		font-size: 0.875rem;
		white-space: nowrap;
	}

	.tag-remove {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		padding: 0;
		background: rgba(255, 255, 255, 0.2);
		border: none;
		border-radius: 50%;
		color: white;
		font-size: 1rem;
		line-height: 1;
		cursor: pointer;
		transition: background 0.2s;

		&:hover {
			background: rgba(255, 255, 255, 0.4);
		}
	}

	.tag-input-field {
		flex: 1;
		min-width: 120px;
		border: none;
		outline: none;
		font-size: 1rem;
		background: transparent;

		&::placeholder {
			color: var(--color-text-light);
		}
	}
</style>
