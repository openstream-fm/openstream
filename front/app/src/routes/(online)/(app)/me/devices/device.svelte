<script lang="ts">
  type Item = import("./$types").PageData["devices"]["items"][number];
  export let device: Item;
  export let on_remove: ((() => void) | null) = null;

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { tooltip } from "$share/tooltip";
	import { mdiAndroid, mdiApple, mdiAppleIos, mdiAppleSafari, mdiFirefox, mdiGoogleChrome, mdiLanDisconnect, mdiLinux, mdiMicrosoftEdge, mdiMicrosoftWindows, mdiOpera, mdiTrashCanOutline, mdiWeb } from "@mdi/js";

  $: data = get_data(device);
  const get_data = (...args: any[]): Array<{ label: string, value: string }> => {
    const data: Array<{ label: string, value: string }> = [];
        
    if(device.ua.name) {
      data.push({ label: "Browser", value: device.ua.name })
    } else {
      data.push({ label: "Browser", value: "Unknown" });
    }

    if(device.ua.os) {
      data.push({ label: "OS", value: device.ua.os })
    } else {
      data.push({ label: "OS", value: "Unknown" });
    }

    data.push({ label: "IP", value: device.ip });

    data.push({ label: "Connected", value: new Date(device.created_at).toLocaleString() })
    data.push({ label: "Last used", value: new Date(device.last_used_at || device.created_at).toLocaleString() })

    return data;
  }

  $: icon = get_icon(device);
  const get_icon = (...args: any[]) => {
    const v = device.ua.name?.toLowerCase();
    if(v === "chrome") return mdiGoogleChrome;
    if(v === "firefox") return mdiFirefox;
    if(v === "edge") return mdiMicrosoftEdge;
    if(v === "safari") return mdiAppleSafari;
    if(v === "opera") return mdiOpera;
    return mdiWeb;
  }

  $: os_icon = get_os_icon(device);
  const get_os_icon = (...args: any[]) => {
    const v = device.ua.os?.toLowerCase();
    if(v === "android") return mdiAndroid;
    if(v === "ios" || v === "iphone" || v === "ipad" || v === "ipod") return mdiAppleIos;
    if(v === "windows") return mdiMicrosoftWindows;
    if(v === "linux") return mdiLinux;
    if(v === "osx") return mdiApple;
    return null;
  }
</script>



<style>
  .device {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    padding: 1rem 2rem;
  }

  .current {
    background: var(--selection-blue);
  }

  .icon {
    flex: none;
    display: flex;
    font-size: 4rem;
    margin-inline-end: 1rem;
    position: relative;
  }

  .os-icon {
    position: absolute;
    font-size: 1.5rem;
    display: flex;
    background: var(--red);
    box-shadow: rgba(0,0,0,0.8) 0 0 3px 0;
    color: #fff;
    border-radius: 50%;
    padding: 0.25rem;
    right: 0;
    bottom: 0;
  }

  .info {
    flex: 1;
  }

  .info-label {
    color: #888;
  }

  .disconnect {
    display: flex;
    color: #fff;
    background: var(--red);
    border-radius: 50%;
    box-shadow: rgba(0,0,0,0.5) 0 0 3px 0;
    font-size: 1rem;
    padding: 0.5rem;
    justify-self: flex-end;
  }
</style>

<div class="device" class:current={device.is_current}>
  <div class="icon">
    <Icon d={icon} />
    {#if os_icon}
      <div class="os-icon">
        <Icon d={os_icon} />
      </div>
    {/if}
  </div>
  <div class="info">
    {#each data as item}
      <div class="info-item">
        <span class="info-label">{item.label}:</span>
        <span class="info-value">{item.value}</span>
      </div>
    {/each}
  </div>
  {#if on_remove}
    <button class="disconnect ripple-container" use:ripple use:tooltip={"Disconnect"} aria-label="Disconnect" on:click={on_remove}>
      <Icon d={mdiTrashCanOutline} />
    </button>
  {/if}
</div>