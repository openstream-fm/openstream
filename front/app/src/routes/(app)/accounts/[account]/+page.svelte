<script lang="ts">
  import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
  import Page from "$lib/components/Page.svelte";
  import preetyBytes from "pretty-bytes";
	import type { PageData } from "./$types";

  export let data: PageData;

  $: account = data.account;
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
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 5%);
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
  
  /*
  .status-info {
    background: #fff;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 5%);
  }

  .status {
    padding: 1rem;
    gap: 0.75rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .status-on {
    --color: var(--green); 
  }
  
  .status-off {
    --color: var(--red);
  }

  .status-line {
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 5rem;
    user-select: none;
  }

  .status-label {
    font-weight: 500;
    color: #333;
    font-size: 1.15rem;
    padding: 0 0 0 1rem;
    white-space: nowrap;
  }

  .status-value {
    margin-inline-start: 0.75rem;
    padding: 0.75rem 1rem;
    color: #fff;
    font-size: 1.15rem;
    border-radius: 5rem;
    font-weight: 500;
    background: var(--color);
    box-shadow: 0 4px 8px 0 rgb(0 0 0 / 12%), 0 2px 4px 0 rgb(0 0 0 / 8%);
    white-space: nowrap;
  }

  .info {
    padding: 1.5rem 1.5rem 2.25rem 1.5rem;
  }

  .info-title {
    font-weight: 600;
    margin-bottom: 1rem;
    font-size: 1.4rem;
  }

  .info-items {
    display: flex;
    flex-wrap: wrap;
    column-gap: 3rem;
    row-gap: 1.75rem;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    height: 2rem;
  }

  .info-label {
    color: #888;
    font-size: 0.85rem;
  }

  .info-value {
    font-weight: 500;
    font-size: 1.1rem;
  }

  .sep {
    width: 10px;
    height: 80%;
    flex: none;
    background: #000;
  }

  @media screen and (max-width: 700px) {
    .info-title {
      text-align: center;
      margin-bottom: 2rem;
    }

    .info-items {
      justify-content: center;
    }

    .status-info {
      flex-direction: column;
      padding: 2rem 1rem;
    }

    .status {
      align-items: center;
    }
  }
  */
</style>

<Page>

  <!--
  <div class="status-info">
    <div class="status" class:status-on={data.on_air} class:status-off={!data.on_air}>
      {#if data.on_air}
        <div class="status-line">
          <div class="status-label">Status:</div>
          <div class="status-value">ON</div>
        </div>
        <div class="status-line">
          <div class="status-label">Playing:</div>
          <div class="status-value">{data.live_streaming ? "Live streaming" : "Auto DJ"}</div>
        </div>
      {:else}
        <div class="status-line status-off">
          <div class="status-label">Status:</div>
          <div class="status-value">OFF</div>
        </div>
      {/if}
    </div>

    <div class="sep" />

    <div class="info">
      <div class="info-title">Icecast settings</div>
      <div class="info-items">
        <div class="info-item">
          <div class="info-label">Server</div>
          <div class="info-value">{data.ice.host}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Port</div>
          <div class="info-value">{data.ice.port}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Mount Point</div>
          <div class="info-value">{data.ice.mount}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Username</div>
          <div class="info-value">{data.ice.user}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Password</div>
          <div class="info-value">{data.ice.password}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Encoding</div>
          <div class="info-value">{data.ice.encoding}</div>
        </div>
        <div class="info-item">
          <div class="info-label"></div>
          <div class="info-value"></div>
        </div>
      </div>
    </div>
  </div>
-->
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