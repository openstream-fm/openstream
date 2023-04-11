<script lang="ts">
  type Item = import("./$types").PageData["devices"]["items"][number];
  export let device: Item;
  export let on_remove: ((() => void) | null) = null;

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { tooltip } from "$share/tooltip";
	import { mdiTrashCanOutline } from "@mdi/js";

  import icon_chrome from "$share/img/browser-icons/chrome.svg";
  import icon_firefox from "$share/img/browser-icons/firefox.svg";
  import icon_safari from "$share/img/browser-icons/safari.svg";
  import icon_edge from "$share/img/browser-icons/edge.svg";
  import icon_opera from "$share/img/browser-icons/opera.svg";
  import icon_other from "$share/img/browser-icons/other.svg";

  import icon_linux from "$share/img/os-icons/linux.svg";
  import icon_android from "$share/img/os-icons/android.svg";
  import icon_windows from "$share/img/os-icons/windows.svg";
  import icon_osx from "$share/img/os-icons/osx.svg";
  import icon_ios from "$share/img/os-icons/ios.svg";
	import NullEmail from "$lib/components/Form/Nullable/NullEmail.svelte";
	import { browser } from "$app/environment";


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

    data.push({ label: "Last used", value: browser ? new Date(device.last_used_at || device.created_at).toLocaleString() : "" })
    data.push({ label: "Connected", value: browser ? new Date(device.created_at).toLocaleString() : "" })

    return data;
  }

  $: icon = get_icon(device);
  const get_icon = (...args: any[]) => {
    const v = device.ua.name?.toLowerCase();
    if(v === "chrome") return icon_chrome;
    if(v === "firefox") return icon_firefox;
    if(v === "edge") return icon_edge;
    if(v === "safari") return icon_safari;
    if(v === "opera") return icon_opera;
    return icon_other;
  }

  $: os_icon = get_os_icon(device);
  const get_os_icon = (...args: any[]) => {
    const v = device.ua.os?.toLowerCase();
    if(v === "linux") return icon_linux;
    if(v === "windows") return icon_windows;
    if(v === "android") return icon_android;
    if(v === "ios" || v === "iphone" || v === "ipad" || v === "ipod") return icon_ios;
    if(v === "osx" || v === "mac" || v === "mac osx") return icon_osx;
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
    margin-inline-end: 1rem;
    padding: 0.5rem;
    position: relative;
  }

  .icon-bg {
    flex: 1;
    width: 3rem;
    height: 3rem;
    flex: none;
    background-size: contain;
    background-repeat: no-repeat;
    background-position: center;
  }

  .os-icon {
    position: absolute;
    font-size: 1.5rem;
    width: 2rem;
    height: 2rem;
    background-size: 80% 80%;
    background-position: center;
    background-repeat: no-repeat;
    filter: drop-shadow(rgba(0,0,0,0.25) 0 0 5px);
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
    <div class="icon-bg" style="background-image: url({ icon })" />
    <!--<Icon d={icon} />-->
    {#if os_icon}
      <div class="os-icon" style="background-image: url({ os_icon })" />
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