<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { mdiConnection } from "@mdi/js";
  import { now } from "$share/now";
	import { slide, type TransitionConfig } from "svelte/transition";
	import { onMount } from "svelte";
	import { default_logger } from "$share/logger";
	import { sleep } from "$share/util";
	import { afterNavigate, beforeNavigate } from "$app/navigation";
	import { _get } from "$share/net.client";
  import { STATION_PICTURES_VERSION } from "$defs/constants";
	import { page } from "$app/stores";
	import { GET, unwrap } from "$lib/client";

  type Item = typeof data.stream_connections.items[number];

  let searchParams = $page.url.searchParams;

  $: q_deployment_id = searchParams.get("deployment") ?? null;
  $: q_station_id = searchParams.get("station") ?? null;
  $: q_ip = searchParams.get("ip") ?? null;
  
  $: qs_referer = searchParams.get("referer");
  $: q_referer = qs_referer == null ? undefined : qs_referer === "null" ? null : qs_referer;
  
  $: qs_os = searchParams.get("os");
  $: q_os = qs_os == null ? undefined : qs_os === "null" ? null : qs_os;
  
  $: qs_browser = searchParams.get("browser");
  $: q_browser = qs_browser == null ? undefined : qs_browser === "null" ? null : qs_browser;
  
  $: show_items = get_show_items(data, q_referer, q_deployment_id, q_station_id, q_browser, q_os, q_ip);
  const get_show_items = (...args: any[]) => {
    let items = data.stream_connections.items;
    if(q_referer !== undefined) {
      items = items.filter(item => item.do === q_referer);
    }

    if(q_os !== undefined) {
      items = items.filter(item => item.os === q_os);
    }

    if(q_browser !== undefined) {
      items = items.filter(item => item.br === q_browser);
    }

    if(q_ip != null) {
      items = items.filter(item => item.ip === q_ip);
    }

    if(q_deployment_id != null) {
      items = items.filter(item => item.dp === q_deployment_id);
    }

    if(q_station_id != null) {
      items = items.filter(item => item.st === q_station_id);
    }

    return items;
  }

  const item_station = (item: Item): typeof data.stations[number] | null => {
    const id = item.st;
    return data.all_stations.find(item => item._id === id) ?? null;
  }

  const qs = (qs: URLSearchParams | string) => {
    const s = String(qs);
    return s === "" ? "" : `?${s}`
  }

  const make_params = ({
    deployment = q_deployment_id,
    station = q_station_id,
    referer = q_referer,
    os = q_os,
    browser = q_browser,
    ip = q_ip
  }: {
    deployment?: string | null,
    station?: string | null,
    referer?: string | null | undefined,
    os?: string | null | undefined,
    browser?: string | null | undefined,
    ip?: string | null,
  }) => {
    const params: Record<string, any> = {};
    if(deployment) params.deployment = deployment;
    if(station) params.station = station;
    if(referer) params.referer = referer;
    if(os) params.os = os;
    if(browser) params.browser = browser;
    if(ip) params.ip = ip;
    
    let qs = Object.entries(params).map(([k,v]) => {
      return `${k}=${encodeURIComponent(v)}`;
    }).join("&");

    if(qs !== "") qs = `?${qs}`;

    return qs;
  }

  const station_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      station: q_station_id === item.st ? null : item.st
    })}`
  }

  const deployment_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      deployment: q_deployment_id === item.dp ? null : item.dp,
    })}`
  }
  
  const referer_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      referer: q_referer === item.do ? null : item.do === null ? "null" : item.do, 
    })}`
  }

  const ip_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      ip: q_ip === item.ip ? null : item.ip 
    })}`
  }

  const os_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      os: q_os === item.os ? null : item.os === null ? "null" : item.os, 
    })}`
  }

  const browser_toggle_link = (item: Item): string => {
    return `/listeners${make_params({
      browser: q_browser === item.br ? null : item.br === null ? "null" : item.br, 
    })}`
  }

  let navigating = false;

  const go = (target: string) => {
    history.replaceState(history.state, "", target);
    navigating = true;
    token++;
    last_update = Date.now();
    searchParams = new URLSearchParams(location.search)
    sleep(5).then(() => navigating = false);
  }

  const get_anchor = (node: Element): HTMLAnchorElement | null => {
    if(node instanceof HTMLAnchorElement && node.href) return node;
    if(node.parentElement) return get_anchor(node.parentElement);
    return null;
  }

  const item_click = (event: MouseEvent) => {
    const target = event.target;
    if(target == null) return;
    if(!(target instanceof Element)) return;
    const anchor = get_anchor(target);
    if(anchor == null) return;
    event.stopPropagation();
    event.preventDefault();
    go(anchor?.href);
  }
 
  const SEC = 1_000;
  const MIN = SEC * 60;
  const HOUR = MIN * 60;
  const DAY = HOUR * 24;
  const duration = (item: Item, $now: Date): string => {
    const ms = item.du != null ? item.du : (+$now - +new Date(item.ca));
    if(ms >= DAY) {
      const d = Math.floor(ms / DAY);
      const h = Math.floor((ms % DAY) / HOUR);
      const m = Math.round((ms % HOUR) / MIN);
      return `${d}d ${h}h ${m}m`;
    } else if (ms >= HOUR) {
      const h = Math.floor(ms / HOUR);
      const m = Math.round((ms % HOUR) / MIN);
      return `${h}h ${m}m`;
    } else if(ms >= MIN) {
      const m = Math.floor(ms / MIN);
      const s = Math.round((ms % MIN) / SEC);
      return `${m}m ${s}s`;
    } else {
      const s = Math.round(ms / SEC);
      return `${s}s`;
    }
  }

  let token = 0;


  let last_update = Date.now();
  afterNavigate(() => {
    token++;
    last_update = Date.now();
    if(String(new URLSearchParams(location.search)) !== String(searchParams)) {
      searchParams = new URLSearchParams(location.search);
    }
  })

  beforeNavigate(() => {
    token++;
  })
    
  onMount(() => {
    const UPDATE_INTERVAL = 1_000;
    let on_screen = true;
    let mounted = true;
    const logger = default_logger.scoped("listeners-update");
    (async () => {
      let _prev_skip: boolean | null = null;
        while(true) {
          await sleep(100)
          const skip = document.visibilityState === "hidden" || !on_screen;
          if(!mounted) return;
          const prev_skip = _prev_skip;
          _prev_skip = skip;
          if(skip) {
            if(skip !== prev_skip) {
              logger.info(`pausing listeners update (document: ${document.visibilityState}, on_screen: ${on_screen})`);
            }
          } else {
            if(skip !== prev_skip) {
              logger.info(`(re)starting listeners update (document: ${document.visibilityState}, on_screen: ${on_screen})`);
            }
            
            if(Date.now() - last_update < UPDATE_INTERVAL) continue;

            try {
              const _token = ++token;

              const stream_connections = unwrap(await GET("/stream-connections-lite", {
                params: {
                  query: {
                    show: "open",
                    limit: 100_000,
                    sort: "creation-desc",
                    // stations: q_station_id ? [q_station_id] : undefined,
                  }
                }
              }));

              if(_token === token) {
                data.stream_connections = stream_connections;
                logger.info(`stream connections updated`);
              } else {
                logger.info(`stream connection update skipped, token mismatch`)
              }
            } catch(e) {
              logger.warn(`error updating listeners: ${e}`);
            } finally {
              last_update = Date.now();
            }
          }
        }
      })()

    return () => {
      mounted = false;
    }
  })

  const transition_item = (node: HTMLElement, dir: boolean) => {
    if(navigating) {    
      node.style.opacity = dir ? "0" : "";
      return () => {
        sleep(3).then(() => node.style.opacity = dir ? "" : "0");
        return {
          duration: 500,
        }
      }
    } else {
      return slide(node, { duration: 400 }) 
    }
   }

  const enter_item = ((node: HTMLElement) => transition_item(node, true)) as (node: HTMLElement, args: any) => TransitionConfig;
  const leave_item = ((node: HTMLElement) => transition_item(node, false)) as (node: HTMLElement, args: any) => TransitionConfig;
</script>

<style>
  .list {
    box-shadow: var(--some-shadow);
    display: flex;
    flex-direction: column;
    align-items: stretch;
    background: #fff;
    padding: 0.5rem;
    border-radius: 0.5rem;
    margin-top: 1rem;
  }

  .item {
    display: flex;
    flex-direction: row;
    align-items: center;
    /* transition: background-color 200ms ease; */
    padding: 1rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.9rem;
    transition: opacity 500ms ease;
  }

  .pic {
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    width: 5rem;
    height: 5rem;
    margin-inline-end: 1.5rem;
    flex: none;
    border-radius: 0.5rem;
  }

  .pic.empty {
    background: #eee;
  }

  .data {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .station-name {
    font-size: 1rem;
    font-weight: var(--font-bold);
  }

  a:hover {
    text-decoration: underline;
  }

  .duration:before {
    display: inline-block;
    width: 0.6rem;
    height: 0.6rem;
    border-radius: 50%;
    content: "";
    vertical-align: middle;
    margin-inline-end: 0.5rem;
    margin-block: -0.25rem;
  }

  .item.open .duration:before {
    background-color: var(--green)
  }

  .item.closed .duration:before {
    background-color: #bbb;
  }
</style>

<svelte:head>
  <title>Listeners</title>
</svelte:head>

<Page>
  <PageTop icon={mdiConnection}>
    <svelte:fragment slot="title">
      Listeners
    </svelte:fragment>
    <svelte:fragment slot="subtitle">
      {show_items.length} {show_items.length === 1 ? "listener" : "listeners"}
      {#if show_items.length !== data.stream_connections.total}
        of {data.stream_connections.total} total
      {/if}
    </svelte:fragment>
  </PageTop>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="list" on:click={item_click}>
    {#each show_items as item (item._id)}
      {@const station = item_station(item)}
      <div class="item" class:open={item.op} class:closed={!item.op} in:enter_item={{}} out:leave_item={{}}>
        <div class="pic" class:empty={station == null} style:background-image={
            station != null ? 
            `url(${data.config.storage_public_url}/station-pictures/webp/128/${station?.picture_id}.webp?v=${STATION_PICTURES_VERSION})` :
            undefined
          }
        /> 
        <div class="data">
          <a class="na station-name" data-sveltekit-replacestate href={station_toggle_link(item)}>
            {#if q_station_id === item.st}
              «
            {/if}

            {#if station != null}
              {station.name}
            {:else}
              #{item.st}
            {/if}
          </a>
          <div class="ip">
            <a
              class="na ip-link"
              data-sveltekit-replacestate
              href={ip_toggle_link(item)}
            >
              {#if item.ip === q_ip}
                «
              {/if}
              {item.ip}
            </a>
          </div>
          <a class="na deployment" data-sveltekit-replacestate href="{deployment_toggle_link(item)}">
            {#if q_deployment_id === item.dp}
              «
            {/if}
            Deployment #{item.dp}
          </a>
          <div class="platform">
            {#if item.br && item.os}
              <a class="na browser" data-sveltekit-replacestate href="{browser_toggle_link(item)}">
                {#if item.br === q_browser}
                  «
                {/if}
                {item.br}
              </a>
                on
              <a class="na os" data-sveltekit-replacestate href="{os_toggle_link(item)}">
                {#if item.os === q_os}
                  «
                {/if}
                {item.os}
              </a>
            
            {:else if item.br}
              <a class="na browser" data-sveltekit-replacestate href="{browser_toggle_link(item)}">
                {#if item.br === q_browser}
                  «
                {/if}
                {item.br}
              </a>
            {:else if item.os}
              <a class="na os" data-sveltekit-replacestate href="{os_toggle_link(item)}">
                {#if item.os === q_os}
                  «
                {/if}
                {item.os}
              </a>
            {:else}
              Unknown 
                <a class="na browser" data-sveletkit-replacestate href="{browser_toggle_link(item)}">  
                  {#if item.br === q_browser}
                    «
                  {/if}
                  browser
                </a>
              and
              <a class="na os" data-sveletkit-replacestate href="{os_toggle_link(item)}">  
                {#if item.os === q_os}
                  «
                {/if}
                platform
              </a>
            {/if}
          </div>
          <a class="na referer" data-sveltekit-replacestate href="{referer_toggle_link(item)}">
            {#if q_referer === item.do}
              «
            {/if}
            {item.do ?? "Unknown referer"}
          </a>
          <div class="duration">
            {duration(item, $now)}
          </div>
        </div>
      </div>
    {/each}
  </div>
</Page>