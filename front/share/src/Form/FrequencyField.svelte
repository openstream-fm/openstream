<script lang="ts">
	export let value: StationFrequency | null;

	let freq = value?.freq ?? null;
	let kind: 'am' | 'fm' | 'web' = value?.kind || "web";
	$: if (typeof freq === 'number' && Number.isNaN(freq)) freq = null;

	$: if (kind === 'am' || kind === 'fm') {
		if (typeof freq === 'number') {
			value = { kind, freq };
		}
	} else if (kind === 'web') {
		value = null;
	} else {
		assert_never(kind, 'FrequencyField.kind');
	}

	const kinds = [
		{ id: 'web', locale: "Web_only" },
		{ id: 'am', locale: "Frequency_AM" },
		{ id: 'fm', locale: "Frequency_FM" },
	] as const;

  import { locale } from "$lib/locale";
	import type { StationFrequency } from '$server/defs/StationFrequency';
	import { _number } from '$share/formy/validate';
	import Validator from '$share/formy/Validator.svelte';
	import { ripple } from '$share/ripple';
	import { slide } from 'svelte/transition';
	import { assert_never } from '$share/assert-never';
	import NullNumberField from './Nullable/NullNumberField.svelte';
	import { mdiSineWave } from '@mdi/js';
	import { VALIDATE_STATION_FREQUENCY_MAX, VALIDATE_STATION_FREQUENCY_MIN } from "$server/defs/constants";
</script>

<style>
	.frequency-field {
		display: flex;
		flex-direction: column;
	}

	.kind-selector {
		display: flex;
		flex-direction: row;
		align-items: stretch;
		align-self: flex-start;
		gap: 0.5rem;
		margin-top: 0.65rem;
	}

	.kind-btn {
		padding: 0.75rem;
		font-weight: var(--font-bold);
		background: rgba(255, 255, 255, 0.8);
		transition: background-color 350ms ease, color 350ms ease;
    border-radius: 0.35rem;
	}

	.kind-btn.selected {
		background: rgba(var(--blue-rgb), 0.1);
		color: var(--blue);
	}

	.freq-field {
		margin-top: 1rem;
	}

  .label {
    font-size: 0.9rem;
    color: #aaa;
  }
</style>

<div class="frequency-field">
	<div class="label">
		{$locale.station_profile.frequency.Main_frequency}
	</div>
	<div class="kind-selector">
		{#each kinds as item (item.id)}
			<button
				class="kind-btn ripple-container"
				class:selected={item.id === kind}
				use:ripple
				on:click|preventDefault={() => (kind = item.id)}
			>
				{$locale.station_profile.frequency[item.locale]}
			</button>
		{/each}
	</div>
	{#if kind === 'am' || kind === 'fm'}
		<div class="freq-field" transition:slide|local={{ duration: 200 }}>
			<NullNumberField
				label={$locale.station_profile.frequency.Frequency}
				icon={mdiSineWave}
				min={0}
				max={100_000}
				step={kind === 'am' ? 1 : 0.1}
				bind:value={freq}
			/>
			<Validator value={freq} fn={_number({
					required: true,
					min: VALIDATE_STATION_FREQUENCY_MIN,
					max: VALIDATE_STATION_FREQUENCY_MAX
				})}
			/>
		</div>
	{/if}
</div>
