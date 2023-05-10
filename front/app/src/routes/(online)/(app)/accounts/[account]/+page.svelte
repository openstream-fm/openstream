<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import type { Station } from "$server/defs/db/Station";
	import StatsMap from "$share/Map/StatsMap.svelte";
	import type { Stats } from "$share/Map/StatsMap.svelte";
	import { click_out, intersect } from "$share/actions";
	import { _get, _patch, action } from "$share/net.client";
	import { ripple } from "$share/ripple";
	import { fly } from "svelte/transition";

  let selector_state: { kind: "account" | "station", record_id: string, data: Stats, station: typeof data.stations.items[number] | null } = {
    kind: "account",
    record_id: data.account._id,
    data: data.stats,
    station: null,
  };

  import type { View } from "$share/Map/StatsMap.svelte";
	import { default_logger } from "$share/logger";
	import { onMount } from "svelte";
	import { sleep } from "$share/util";
	import type { AccountLimits } from "$server/defs/AccountLimits";
	import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
	import { tooltip } from "$share/tooltip";
	import Icon from "$share/Icon.svelte";
	import { mdiCircleEditOutline, mdiFileEditOutline } from "@mdi/js";
	import Dialog from "$share/Dialog.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
  let view: View = "now";

  let _token = 0;

  const select = action(async (station: typeof data.stations.items[number] | null) => {
    selector_open = false;
    if(station?._id === selector_state.station?._id) return;
    const token = ++_token;
    if(station) {
      const { stats }: import("$server/defs/api/stations/[station]/stream-stats/GET/Output").Output =
        await _get(`/api/stations/${station._id}/stream-stats`);
      if(token === _token) {
        selector_state = {
          kind: "station",
          record_id: station._id,
          data: stats,
          station,
        }
      }
    } else {
      const { stats }: import("$server/defs/api/accounts/[account]/stream-stats/GET/Output").Output =
        await _get(`/api/accounts/${data.account._id}/stream-stats`);
      if(token === _token) {
        selector_state = {
          kind: "account",
          record_id: data.account._id,
          data: stats,
          station: null,
        }
      }
    }
  })

  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

  let selector_open = false;
  
  const close_selector = () => {
    selector_open = false
  }

  const toggle_selector = () => {
    selector_open = !selector_open
  }

  const selector_menu_click_out = () => {
    setTimeout(close_selector, 2);  
  }


  const units = [ "B", "KB", "MB", "GB", "TB" ];
  
  const to_fixed_2 = (v: number): number => Math.round(v * 100) / 100; 

  const preety_bytes = (_v: number): string => {
    
    let v = _v;

    for(const unit of units) {
      if(v < 1000) {
        return `${to_fixed_2(v)} ${unit}`;
      } 
      v = v / 1000;
    }

    return `${to_fixed_2(v)} PB`;
  }

  const LIMITS_UPDATE_INTERVAL = 5_000;
  let limits_on_screen = true;

  const limits = (node: HTMLElement) => {
    
    const logger = default_logger.scoped("limits");

    let mounted = true;
        
    (async () => {
      let _prev_skip: boolean | null = null;
      let last = Date.now();
      while(true) {
        await sleep(100)
        const skip = document.visibilityState === "hidden" || limits_on_screen === false;
        if(!mounted) return;
        const prev_skip = _prev_skip;
        _prev_skip = skip;
        if(skip) {
          if(skip !== prev_skip) {
            logger.info(`pausing limits update (document: ${document.visibilityState}, on_screen: ${limits_on_screen})`);
          }
        } else {
          if(skip !== prev_skip) {
            logger.info(`(re)starting limits update (document: ${document.visibilityState}, on_screen: ${limits_on_screen})`);
          }
          if(Date.now() - last < LIMITS_UPDATE_INTERVAL) continue;
          try {
            const limits: AccountLimits = await _get(`/api/accounts/${data.account._id}/limits`);
            logger.info(`account limits updated`);
            data.account.limits = limits;
          } catch(e) {
            logger.warn(`error updating station limits: ${e}`);
          } finally {
            last = Date.now();
          }
        }
      }
    })()
    
    return {
      destroy: () => mounted = false
    }
  }

  let edit_open = false;
  let current_account_name = data.account.name;
  const edit = action(async () => {
    let payload: import("$server/defs/api/accounts/[account]/PATCH/Payload").Payload = {
      name: current_account_name,
    };
    await _patch(`/api/accounts/${data.account._id}`, payload);
    data.account.name = current_account_name;
    edit_open = false;
})
</script>

<style>

  .title {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  h1 {
    font-weight: 600;
  }

  .edit-btn {
    flex: none;
    font-size: 1.5rem;
    border-radius: 50%;
    padding: 0.75rem;
    display: flex;
    margin-inline-start: 0.5rem;
    transition: background-color 200ms ease;
  }

  .edit-btn:hover {
    background: rgba(0,0,0,0.05);
  }



  .stats {
    margin-top: 2rem;
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }

  .stats-selector-out {
    padding: 0.5rem;
    margin-bottom: -1rem;
  }

  .stats-selector {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .stats-selector-btn, .stats-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0 1rem;
    height: 3rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
  }

  .stats-selector-btn:hover, .stats-selector-btn.open, .stats-selector-item:hover {
    background: rgba(0,0,0,0.025);
  }

  .stats-selector-btn-text {
    margin-inline-end: 0.75rem;
  }

  .stats-selector-item.current {
    background: rgba(var(--blue-rgb), 0.1);
  }

  .stats-selector-menu {
    min-width: min(80vw, 20rem);
  }
  
  .stats-selector-btn-icon, .stats-selector-icon {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 0.25rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
    flex: none;
    margin-inline-end: 1rem;
    margin-inline-start: -0.5rem;
  }

  .stats-selector-anchor {
    position: absolute;
    left: 0;
    bottom: 0;
    width: 0;
    height: 0;
    z-index: 1;
  }

  .stats-selector-menu {
    display: flex;
    flex-direction: column;
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
    background: #fff;
    padding: 0.5rem;
    border-radius: 0.5rem;
  }

  .stats-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
  }


  .meters {
    --spacing: 1.5rem;
    display: flex;
    flex-direction: row;
    gap: var(--spacing);
    margin-top: var(--spacing);
    align-items: stretch;
  }

  .meter {
    background: #fff;
    flex: 1;
    padding: 2rem 1rem;
    border-radius: 0.5rem;
    text-align: center;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }

  .meter-title {
    font-weight: 600;
    font-size: 2em;
  }

  .meter-text {
    color: #333;
    font-size: 1.5em;
  }

  .used, .avail {
    font-weight: 600;
  }

  .used {
    color: var(--red);
  }

  .avail {
    color: #333;
  }

  .of {
    color: #999;
    font-size: 0.8em;
  }

  .meter-graph {
    max-width: 15rem;
    margin: 0 auto;
  }

  @media screen and (max-width: 1460px) {
    .meters {
      display: grid;
      grid-template-columns: 1fr 1fr;
    }    
  }

  @media screen and (max-width: 600px) {
    .meters {
      grid-template-columns: 1fr;
    }

    .meters {
      flex-direction: column;
    }

    .meter-graph {
      max-width: 10rem;
    }
  } 

  .edit-dialog-btn-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-top: 2rem;
  }

  .edit-dialog-btn {
    color: #fff;
    background: var(--blue);
    font-weight: 600;
    padding: 0.75rem;
    box-shadow: var(--some-shadow);
  }
</style>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  
  <div class="title">
    <h1>{data.account.name}</h1>
    <button class="edit-btn ripple-container" use:ripple use:tooltip={"Edit"} on:click={() => edit_open = true}>
      <Icon d={mdiCircleEditOutline} />
    </button>
  </div>

  <div class="stats">
    <div class="stats-selector-out">
      <div class="stats-selector">
        <button class="stats-selector-btn ripple-container" class:open={selector_open} use:ripple aria-label={"Select one station or all"} on:click={toggle_selector}>
          {#if selector_state.station != null}
            <div
              class="stats-selector-btn-icon"
              style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{selector_state.station.picture_id}.webp)"
            />
          {/if}
          <div class="stats-selector-btn-text">
            {#if selector_state.station}
              {selector_state.station.name}
            {:else}
              All stations
            {/if}
          </div>
          ▼
        </button>
        <div class="stats-selector-anchor">
          {#if selector_open}
            <div 
              class="stats-selector-menu"
              use:click_out={selector_menu_click_out}
              transition:fly|local={{ duration: 125, y: -10 }}
            >
              <button class="stats-selector-item" class:current={selector_state.station == null} on:click={() => select(null)}>
                <div class="stats-selector-name">
                  All stations
                </div>
              </button>
              {#each account_stations as station (station._id)}
                <button class="stats-selector-item" class:current={selector_state.station?._id === station._id} on:click={() => select(station)}>
                  <div class="stats-selector-icon" style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)" />
                  <div class="stats-selector-name">
                    {station.name}
                  </div>
                </button>
              {/each}  
            </div>
          {/if}
        </div>
      </div>
    </div>
    
    <StatsMap bind:view kind={selector_state.kind} record_id={selector_state.record_id} bind:data={selector_state.data} />
  </div>

  <div class="meters" use:limits use:intersect={{ enter: () => limits_on_screen = true, leave: () => limits_on_screen = false}}>
    <div class="meter">
      <div class="meter-title">
        Stations
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.stations.used / data.account.limits.stations.total} />
      </div>
      <div class="meter-text">
        <span class="used">{data.account.limits.stations.used}</span>
        <span class="of">of</span>
        <span class="avail">{data.account.limits.stations.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Listeners
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.listeners.used / data.account.limits.listeners.total} />
      </div>
      <div class="meter-text">
        <span class="used">{data.account.limits.listeners.used}</span>
        <span class="of">of</span>
        <span class="avail">{data.account.limits.listeners.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Transfer
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.transfer.used / data.account.limits.transfer.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.account.limits.transfer.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preety_bytes(data.account.limits.transfer.total)}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Storage
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.storage.used / data.account.limits.storage.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.account.limits.storage.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preety_bytes(data.account.limits.storage.total)}</span>
      </div>
    </div>
  </div>
</Page>

{#if edit_open}
  <Dialog width="500px" on_close={() => edit_open = false} title="Edit your account name">
    <Formy action={edit} let:submit>
      <form novalidate class="edit-dialog" on:submit={submit}>
        <div class="edit-dialog-fields">
          <div class="edit-dialog-field">
            <TextField label="Account name" maxlength={50} trim bind:value={current_account_name} />
            <Validator value={current_account_name} fn={_string({ required: true, maxlen: 50 })} />
          </div>
        </div>
        <div class="edit-dialog-btn-out">
          <button type="submit" class="edit-dialog-btn ripple-container" use:ripple>
            Save
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}