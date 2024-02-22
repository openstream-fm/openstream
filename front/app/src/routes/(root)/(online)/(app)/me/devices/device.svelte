<script lang="ts">
  type Item = import("./$types").PageData["devices"]["items"][number];
  export let device: Item;
  export let on_remove: ((() => void) | null) = null;

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { tooltip } from "$share/tooltip";
	import { mdiTrashCanOutline } from "@mdi/js";

  import icon_chrome from "$share/img/browser-icons/chrome.png";
  import icon_firefox from "$share/img/browser-icons/firefox.png";
  import icon_safari from "$share/img/browser-icons/safari.png";
  import icon_edge from "$share/img/browser-icons/edge.png";
  import icon_opera from "$share/img/browser-icons/opera.png";
  import icon_other from "$share/img/browser-icons/other.png";

  import icon_linux from "$share/img/os-icons/linux.png";
  import icon_android from "$share/img/os-icons/android.png";
  import icon_windows from "$share/img/os-icons/windows.png";
  import icon_osx from "$share/img/os-icons/osx.png";
  import icon_ios from "$share/img/os-icons/ios.png";

  import Page from "$lib/components/Page.svelte";
	import { locale, lang } from "$lib/locale";


  const format_date = (date: Date | string | number): string => {
    return new Date(date).toLocaleString($lang, {
      day: "numeric",
      weekday: "long",
      month: "long",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
  }

  $: data = get_data(device);
  const get_data = (..._args: any[]): Array<{ label: string, value: string }> => {
    const data: Array<{ label: string, value: string }> = [];
        
    if(device.ua.name) {
      data.push({ 
        label: $locale.pages["me/devices"].device.browser,
        value: device.ua.name
      })
    } else {
      data.push({
        label: $locale.pages["me/devices"].device.browser,
        value: $locale.pages["me/devices"].device.unkown,
      });
    }

    if(device.ua.os) {
      data.push({
        label: $locale.pages["me/devices"].device.os,
        value: device.ua.os
      })
    } else {
      data.push({
        label: $locale.pages["me/devices"].device.os,
        value: $locale.pages["me/devices"].device.unkown
      });
    }

    data.push({
      label: $locale.pages["me/devices"].device.ip,
      value: device.ip
    });

    data.push({
      label: $locale.pages["me/devices"].device.connected,
      value: format_date(device.created_at),
    })

    data.push({
      label: $locale.pages["me/devices"].device.last_used,
      value: format_date(device.last_used_at || device.created_at)
    })
    
    return data;
  }

  $: icon = get_icon(device);
  const get_icon = (..._args: any[]) => {
    const v = device.ua.name?.toLowerCase();
    if(v === "chrome") return icon_chrome;
    if(v === "firefox") return icon_firefox;
    if(v === "edge") return icon_edge;
    if(v === "safari") return icon_safari;
    if(v === "opera") return icon_opera;
    return icon_other;
  }

  $: os_icon = get_os_icon(device);
  const get_os_icon = (..._args: any[]) => {
    const v = device.ua.os?.toLowerCase();
    if(v?.includes("linux")) return icon_linux;
    if(v?.includes("windows")) return icon_windows;
    if(v?.includes("android")) return icon_android;
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
    inset-inline-end: 0;
    inset-block-end: 0;
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

<Page compact>
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
      <button 
        class="disconnect ripple-container"
        use:ripple
        use:tooltip={$locale.pages["me/devices"].device.tooltips.disconnect}
        aria-label={$locale.pages["me/devices"].device.tooltips.disconnect}
        on:click={on_remove}
      >
        <Icon d={mdiTrashCanOutline} />
      </button>
    {/if}
  </div>
</Page>