<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import StatsMap from "$share/Map/StatsMap.svelte";
	import { intersect } from "$share/actions";
	import { _get, _patch, action } from "$share/net.client";
	import { ripple } from "$share/ripple";

  let current_account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

  let map_selector_data: import("$share/Map/StationSelector.svelte").Data = {
    all_kind: "account",
    kind: "account",
    account_id: data.account._id,
    record_id: data.account._id,
    station: null,
    stations: current_account_stations,
    storage_public_url: data.config.storage_public_url,
    stats: data.stats,
  }

  import type { View } from "$share/Map/StatsMap.svelte";
	import { default_logger } from "$share/logger";
	import { sleep } from "$share/util";
	import type { AccountLimits } from "$server/defs/AccountLimits";
	import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
	import { tooltip } from "$share/tooltip";
	import Icon from "$share/Icon.svelte";
	import { mdiCircleEditOutline } from "@mdi/js";
	import Dialog from "$share/Dialog.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
	import AccountStationItem from "./account-station-item.svelte";
	import { locale } from "$lib/locale";
	import StationSelector from "$share/Map/StationSelector.svelte";
  
  let view: View = "now";

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

  const limits = (_node: HTMLElement) => {
    
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
    let payload: import("$api/accounts/[account]/PATCH/Payload").Payload = {
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
    font-size: 1.5em;
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meter-text {
    color: #333;
    font-size: 1.25em;
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
    max-width: 12rem;
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

  .stations {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 18rem), 1fr));
    gap: 1rem;
  }
</style>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  
  <div class="title">
    <h1>{data.account.name}</h1>
    <button class="edit-btn ripple-container" use:ripple use:tooltip={$locale.pages["account/dashboard"].edit.tooltip} on:click={() => edit_open = true}>
      <Icon d={mdiCircleEditOutline} />
    </button>
  </div>

  {#if current_account_stations.length}
    <div class="stations">
      {#each current_account_stations as station (station._id)}
        <div class="station">
          <AccountStationItem {station} now_playing={data.now_playing_record[station._id]} />
        </div>
      {/each}
    </div>
  {/if}

  <div class="stats">
    <div class="selector">
      <StationSelector bind:data={map_selector_data} />
    </div>
    
    <StatsMap
      bind:view
      kind={map_selector_data.kind}
      record_id={map_selector_data.record_id}
      locale={$locale.stats_map}
      country_names={$locale.countries}
      bind:data={map_selector_data.stats}
    />
  </div>

  <div class="meters" use:limits use:intersect={{ enter: () => limits_on_screen = true, leave: () => limits_on_screen = false}}>
    <div class="meter">
      <div class="meter-title">
        {$locale.limits.stations}
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.stations.used / data.account.limits.stations.total} />
      </div>
      <div class="meter-text">
        <span class="used">{data.account.limits.stations.used}</span>
        <span class="of">{$locale.limits.of}</span>
        <span class="avail">{data.account.limits.stations.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        {$locale.limits.listeners}
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.listeners.used / data.account.limits.listeners.total} />
      </div>
      <div class="meter-text">
        <span class="used">{data.account.limits.listeners.used}</span>
        <span class="of">{$locale.limits.of}</span>
        <span class="avail">{data.account.limits.listeners.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        {$locale.limits.transfer}
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.transfer.used / data.account.limits.transfer.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.account.limits.transfer.used)}</span>
        <span class="of">{$locale.limits.of}</span>
        <span class="avail">{preety_bytes(data.account.limits.transfer.total)}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        {$locale.limits.storage}
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.account.limits.storage.used / data.account.limits.storage.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.account.limits.storage.used)}</span>
        <span class="of">{$locale.limits.of}</span>
        <span class="avail">{preety_bytes(data.account.limits.storage.total)}</span>
      </div>
    </div>
  </div>
</Page>

{#if edit_open}
  <Dialog width="500px" on_close={() => edit_open = false} title={$locale.pages["account/dashboard"].edit.dialog.title}>
    <Formy action={edit} let:submit>
      <form novalidate class="edit-dialog" on:submit={submit}>
        <div class="edit-dialog-fields">
          <div class="edit-dialog-field">
            <TextField label={$locale.pages["account/dashboard"].edit.dialog.field_label} maxlength={50} trim bind:value={current_account_name} />
            <Validator value={current_account_name} fn={_string({ required: true, maxlen: 50 })} />
          </div>
        </div>
        <div class="edit-dialog-btn-out">
          <button type="submit" class="edit-dialog-btn ripple-container" use:ripple>
            {$locale.pages["account/dashboard"].edit.dialog.save}
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}