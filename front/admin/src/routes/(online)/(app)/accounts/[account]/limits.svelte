<script lang="ts">
  export let data: import("./$types").PageData;
  import { intersect } from "$share/actions";
	import { _get, _patch } from "$share/net.client";
	import { default_logger } from "$share/logger";
	import { sleep } from "$share/util";
	import type { AccountLimits } from "$server/defs/AccountLimits";
	import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
	import { GET, unwrap } from "$lib/client";

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
            const { account: { limits } } = unwrap(await GET("/accounts/{account}", { params: { path: { account: data.account._id } } }));
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
</script>

<style>
  .meters {
    --spacing: 1rem;
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
    font-weight: var(--font-bold);
    font-size: 1.4em;
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meter-text {
    color: #333;
    font-size: 1.1em;
  }

  .used, .avail {
    font-weight: var(--font-bold);
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
    max-width: 8rem;
    margin: 0 auto;
  }

  @media screen and (max-width: 1100px) {
    .meters {
      display: grid;
      grid-template-columns: 1fr 1fr;
    }    
  }

  @media screen and (max-width: 450px) {
    .meters {
      grid-template-columns: 1fr;
    }

    .meters {
      flex-direction: column;
    }
  } 
</style>

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

