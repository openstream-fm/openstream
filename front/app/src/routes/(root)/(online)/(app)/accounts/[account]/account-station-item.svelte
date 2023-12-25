<script lang="ts">
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import { get_now_playing_store } from '$lib/now-playing';
	import { intersect } from '$share/actions';
	import { ripple } from '$share/ripple';
	import { onMount } from 'svelte';
	import { STATION_PICTURES_VERSION } from '$defs/constants';

	export let station: import('$server/defs/PublicStation').PublicStation;
	export let session_count: number = 0;
	export let now_playing:
		| import('$api/stations/[station]/now-playing/GET/Output').Output
		| undefined = undefined;

	let current_now_playing: import('$api/stations/[station]/now-playing/GET/Output').Output | null = null;

	$: merged_now_playing = current_now_playing ?? now_playing;

	$: on_air = is_on_air(merged_now_playing ?? null);
	const is_on_air = (merged_now_playing: typeof current_now_playing) => {
		if (merged_now_playing == null) return null;
		if (merged_now_playing.kind === "external-relay") return true;
		if (merged_now_playing.kind === "live") return true;
		if (merged_now_playing.kind === "playlist") return true;
		if (merged_now_playing.kind === "none") {
			if (merged_now_playing.start_on_connect && merged_now_playing.external_relay_error == null) return true;
			else return false;
		};
	}

	let store: ReturnType<typeof get_now_playing_store> | null;
	let unsub: (() => void) | null = null;

	onMount(() => {
		store = get_now_playing_store(station._id, current_now_playing || null);
		unsub = store.subscribe((v) => {
			current_now_playing = v?.info || null;
		});

		return () => {
			if (unsub) unsub();
		};
	});

	const enter = () => {
		if (store != null) return;
		store = get_now_playing_store(station._id, current_now_playing || null);
		unsub = store.subscribe((v) => {
			current_now_playing = v?.info || null;
		});
	};

	const leave = () => {
		if (unsub == null) return;
		unsub();
		unsub = null;
		store = null;
	};
</script>

<style>
	.station {
		/* border-top: 1px var(--red) solid; */
		display: flex;
		flex-direction: row;
		align-items: center;
		background: #fff;
		box-shadow: var(--some-shadow);
		border-radius: 0.35rem;
	}

	.pic {
		flex: none;
		width: min(30%, 8rem);
		aspect-ratio: 1;
		margin: 1rem 1.5rem 1rem 1rem;
		border-radius: 0.5rem;
		background-size: contain;
		background-position: center;
		background-repeat: no-repeat;
	}

	.name {
		font-size: 1.125rem;
		font-weight: 500;
		margin-inline-end: 1rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		/* display: -webkit-box; */
		/* -webkit-line-clamp: 2; */
		/* -webkit-box-orient: vertical; */
	}

	.now-playing-sessions {
		margin-top: 0.15rem;
		font-size: 0.9rem;
	}

	.now-playing-state {
		font-weight: 800;
	}

	.on-air .now-playing-state {
		color: var(--green);
	}

	.off-air .now-playing-state {
		color: var(--red);
	}


  .sessions {
    margin-top: 0.15rem;
  }

	.now-playing-sub {
		margin-top: 0.15rem;
	}

	.now-playing-sub-error {
		color: var(--red);	
	}
</style>

<a
	href="/accounts/{station.account_id}/stations/{station._id}"
	class="na station ripple-container"
	use:ripple
	use:intersect={{ enter, leave }}
	class:on-air={on_air}
	class:off-air={!on_air}
>
	<div
		class="pic"
		style:background-image="url({$page.data.config.storage_public_url}/station-pictures/webp/128/{station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
	/>

	<div class="data">
		<div class="name">{station.name}</div>

		<div class="now-playing-sessions">
			{#if merged_now_playing}
				<div class="now-playing-state">
					{#if on_air}
						{$locale.pages['account/dashboard'].station_item.on_air}
					{:else}
						{$locale.pages['account/dashboard'].station_item.off_air}
					{/if}
				</div>

				{#if on_air}
					<div class="sessions">
						{#if session_count === 0}
							{$locale.misc['0_listeners']}
						{:else if session_count === 1}
							{$locale.misc['1_listener']}
						{:else}
							{$locale.misc.n_listeners.replace('@n', String(session_count))}
						{/if}
					</div>
				{/if}
    

				{#if merged_now_playing.kind === "none" && merged_now_playing.external_relay_error != null}
					<div class="now-playing-sub now-playing-sub-error">
						<!-- TODO: locale -->
						External relay error
						<!-- {merged_now_playing.external_relay_error} -->
					</div>
				{:else if merged_now_playing.kind === 'none'}
					{#if merged_now_playing.start_on_connect}
						<div class="now-playing-sub">
							{#if merged_now_playing.external_relay_url != null}
								{$locale.misc.Relay}
							{:else}
								{$locale.pages['account/dashboard'].station_item.playlist}
							{/if}
						</div>
					{/if}
				{:else}
					<div class="now-playing-sub">
						{#if merged_now_playing.kind === 'live'}
							{$locale.pages['account/dashboard'].station_item.live}
						{:else if merged_now_playing.kind === 'playlist'}
							{$locale.pages['account/dashboard'].station_item.playlist}
						{:else if merged_now_playing.kind === 'external-relay'}
							{$locale.misc.Relay}
						{/if}
					</div>
				{/if}
			{/if}
		</div>
	</div>
</a>
