<script lang="ts">
  import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
  import Page from "$lib/components/Page.svelte";
	import Icon from "$share/Icon.svelte";
	import { _get } from "$share/net.client";
	import { mdiMicrophoneOutline } from "@mdi/js";
  import preetyBytes from "pretty-bytes";
	import { onMount } from "svelte";

  export let data: import("./$types").PageData;

  $: account = data.account;

  const INTERVAL = 1_000;

  onMount(() => {

    const update_limits = async () => {
      try {
        const limits: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"]["limits"] = await _get(`/api/accounts/${account._id}/limits`);
        account.limits = limits;
      } catch(e) {
        console.warn(`error updating limits ${e}`)
      } finally {
        timer = setTimeout(update_limits, INTERVAL,);
      }
    }

    let timer = setTimeout(update_limits, INTERVAL);
    
    return () => clearTimeout(timer);
  })
</script>

<style>

  .meters {
    display: flex;
    flex-direction: row;
    gap: 1rem;
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

  @media screen and (max-width: 1160px) {
    .meter {
      font-size: 0.8rem;
    }
  }

  @media screen and (max-width: 700px) {
    .meters {
      flex-direction: column;
    }

    .meter-graph {
      max-width: 10rem;
    }
  } 

  .top-boxes {
    margin-bottom: 1.5rem;
    display: flex;
    flex-direction: row;
    gap: 2rem;
  }

  .top-box {
    padding: 2rem;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
    background: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .top-box-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #EAF2E0;
    padding: 1rem;
    border-radius: 50%;
    color: var(--green);
    font-size: 3rem;
  }

  .top-box-title {
    font-weight: 700;
    font-size: 2rem;
    margin-top: 1rem;
  }

  .top-box-subtitle {
    color: #444;
    margin-top: 1rem;
  }

  .top-box[data-on] .top-box-icon {
    color: var(--green);
  }

  .top-box[data-on] .top-box-title {
    color: var(--green);
  }
</style>

<svelte:head>
  <title>Dashboard</title>
</svelte:head>

<Page>

  <div class="top-boxes">
    <div class="top-box" data-on>
      <div class="top-box-icon">
        <Icon d={mdiMicrophoneOutline} />
      </div>
      <div class="top-box-title">
        ON AIR
      </div>
      <div class="top-box-subtitle">
        Auto DJ
      </div>
    </div>
  </div>

    <!-- <div class="sep" />

    <div class="info">
      <div class="info-title">Icecast settings</div>
      <div class="info-items">
        <div class="info-item">
          <div class="info-label">Server</div>
          <div class="info-value">source.openstream.test</div>
        </div>
        <div class="info-item">
          <div class="info-label">Port</div>
          <div class="info-value">80</div>
        </div>
        <div class="info-item">
          <div class="info-label">Mount Point</div>
          <div class="info-value">{account._id}/source</div>
        </div>
        <div class="info-item">
          <div class="info-label">Username</div>
          <div class="info-value">source</div>
        </div>
        <div class="info-item">
          <div class="info-label">Password</div>
          <div class="info-value">{account.sourcePassword}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Encoding</div>
          <div class="info-value">MP3 / AAC / OGG</div>
        </div>
      </div>
    </div> 
  </div>-->

  <div class="meters">
    <div class="meter">
      <div class="meter-title">
        Listeners
      </div>
      <div class="meter-graph">
        <CircularMeter start={0} end={account.limits.listeners.used / account.limits.listeners.total} />
      </div>
      <div class="meter-text">
        <span class="used">{account.limits.listeners.used}</span>
        <span class="of">of</span>
        <span class="avail">{account.limits.listeners.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Transfer
      </div>
      <div class="meter-graph">
        <CircularMeter start={0} end={account.limits.transfer.used / account.limits.transfer.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preetyBytes(account.limits.transfer.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preetyBytes(account.limits.transfer.total)}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Storage
      </div>
      <div class="meter-graph">
        <CircularMeter start={0} end={account.limits.storage.used / account.limits.storage.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preetyBytes(account.limits.storage.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preetyBytes(account.limits.storage.total)}</span>
      </div>
    </div>
  </div>
</Page>