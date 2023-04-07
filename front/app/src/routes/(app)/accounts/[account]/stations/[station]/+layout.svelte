<script lang="ts">
  export let data: import("./$types").LayoutData;
  import { page } from "$app/stores";
	import { ripple } from "$share/ripple";
	import { fly } from "svelte/transition";
</script>

<style>

  .station-out {
    padding: 1rem 1rem 0 1rem ;
    display: flex;
    align-items: stretch;
    justify-content: stretch;
    flex-direction: column;
  }

  .station-scroll {
    overflow-y: visible;
    overflow-x: auto;
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    display: flex;
  }

  .station {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    flex: none;
    padding: 1rem 1.5rem 1rem 1.5rem;
  }

  .station-pic {
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 0.25rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
  }

  .station-name {
    max-width: 10rem;
    display: flex;
    flex-shrink: 1;
    min-width: 4rem;
    margin-inline-start: 0.75rem;
  }

  .station-name, .station-action {
    font-size: 1rem;
  }

  .station-actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
    margin-inline-start: 1rem;
  }

  .station-action {
    flex: none;
    color: var(--blue);
    text-decoration: none;
    transition: background-color 200ms ease;
    padding: 0.5rem 0.75rem;
    border-radius: 0.25rem;
    flex: none;
  }

  .station-action.current {
    background: rgba(var(--red-rgb),  0.05);
  }

  .station-name-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }
</style>


{#if data.station != null}
  <div class="station-out" in:fly={{ duration: 300, y: -25 }}>
    <div class="station-scroll super-thin-scroll">
      <div class="station">
        <div class="station-pic" style="background-image: url({ data.config.storagePublicURL }/station-pictures/webp/64/{data.station.picture_id}.webp)" />
        <div class="station-name">
          <div class="station-name-ellipsis">
            {data.station.name}
          </div>
        </div>
        <div class="station-actions">
          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}"
            class="station-action ripple-container"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}`}
            use:ripple
            >
            Dashboard
          </a>

          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}/profile"
            class="station-action ripple-container"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/profile`}
            use:ripple
            >
            Profile
          </a>

          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}/playlist"
            class="station-action ripple-container"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/playlist`}
            use:ripple
            >  
            Playlist
          </a>
        </div>
      </div>
    </div>
  </div>
{/if}

<slot />