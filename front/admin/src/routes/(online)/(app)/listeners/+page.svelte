<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { mdiConnection } from "@mdi/js";
  import { now } from "$share/now";
	import { slide } from "svelte/transition";
	import { onMount } from "svelte";
	import { default_logger } from "$share/logger";
	import { sleep } from "$share/util";
	import { afterNavigate, beforeNavigate } from "$app/navigation";
	import { qss } from "$share/qs";
	import { _get } from "$share/net.client";
  import { STATION_PICTURES_VERSION } from "$defs/constants";

  type Item = typeof data.stream_connections.items[number];

  const item_station = (item: Item): typeof data.stations[number] | null => {
    const id = item.station_id;
    return data.all_stations.find(item => item._id === id) ?? null;
  }

  const website = (item: Item): string | null => {
    const ref = item.request.headers.referer || item.request.headers.origin;
    if(ref == null) return null

    try {
      return new URL(ref).host;
    } catch(e) {}

    return null
  }

  const qs = (qs: URLSearchParams | string) => {
    const s = String(qs);
    return s === "" ? "" : `?${s}`
  }

  const make_params = ({
    deployment = data.stream_connections_query.deployment_id,
    station = data.stream_connections_query.station_id,
    referer = data.stream_connections_query.referer
  }: {
    deployment?: string | null,
    station?: string | null,
    referer?: string | null
  }) => {
    const params = new URLSearchParams();
    deployment && params.append("deployment", deployment);
    station && params.set("station", station);
    referer && params.set("referer", referer);
    return params;
  }

  const station_toggle_link = (item: Item): string => {
    return `/listeners${qs(make_params({
      station: data.stream_connections_query.station_id === item.station_id ? null : item.station_id
    }))}`
  }

  const deployment_toggle_link = (item: Item): string => {
    return `/listeners${qs(make_params({
      deployment: data.stream_connections_query.deployment_id === item.deployment_id ? null : item.deployment_id
    }))}`
  }

  const referer_toggle_link = (ref: string | null): string => {
    return `/listeners${qs(make_params({
      referer: data.stream_connections_query.referer === ref ? null : String(ref) 
    }))}`
  }

  const SEC = 1_000;
  const MIN = SEC * 60;
  const HOUR = MIN * 60;
  const DAY = HOUR * 24;
  const duration = (item: Item, $now: Date): string => {
    const ms = item.duration_ms != null ? item.duration_ms : (+$now - +new Date(item.created_at));
    if(ms >= DAY) {
      const d = Math.floor(ms / DAY);
      const h = Math.round((ms % DAY) / HOUR);
      return `${d}d ${h}h`;
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
              let _token = ++token;

              const params: import("$api/stream-connections/GET/Query").Query = {
                show: "open",
                limit: 100_000,
                sort: "creation-desc",
                stations: data.stream_connections_query.station_id ? [data.stream_connections_query.station_id] : undefined,
              };


              let stream_connections = await _get<import("$api/stream-connections/GET/Output").Output>(`/api/stream-connections${qss(params)}`);

              if(_token === token) {

                if(data.stream_connections_query.deployment_id != null) {
                  const items = stream_connections.items.filter(item => item.deployment_id === data.stream_connections_query.deployment_id);
                  stream_connections = {
                    skip: stream_connections.skip,
                    limit: stream_connections.limit,
                    total: items.length,
                    items,
                  }
                }

                if(data.stream_connections_query.referer != null) {
                  let items = stream_connections.items;
                  if(data.stream_connections_query.referer === "null") {
                    items = items.filter(item => item.request.headers.referer == null && item.request.headers.origin == null);
                  } else {
                    const r = `//${data.stream_connections_query.referer}`;
                    items = items.filter(item => (item.request.headers.referer || item.request.headers.origin || "").includes(r))
                  }
                  
                  stream_connections = {
                    skip: stream_connections.skip,
                    limit: stream_connections.limit,
                    total: items.length,
                    items,
                  }
                }
                
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
    font-weight: 600;
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
      {data.stream_connections.total} {data.stream_connections.total === 1 ? "listener" : "listeners"}
    </svelte:fragment>
  </PageTop>

  <div class="list">
    {#each data.stream_connections.items as item (item._id)}
      {@const station = item_station(item)}
      {@const referer = website(item)}      
      <div class="item" class:open={item.is_open} class:closed={!item.is_open} transition:slide|local={{ duration: 400 }}>
        <div class="pic" class:empty={station == null} style:background-image={
            station != null ? 
            `url(${data.config.storage_public_url}/url(station-pictures/webp/128/${station?.picture_id}.webp?v=${STATION_PICTURES_VERSION})` :
            undefined
          }
        /> 
        <div class="data">
          <a class="na station-name" data-sveltekit-replacestate href={station_toggle_link(item)}>
            {#if data.stream_connections_query.station_id === item.station_id}
              «
            {/if}

            {#if station != null}
              {station.name}
            {:else}
              #{item.station_id}
            {/if}
          </a>
          <div class="ip">
            {item.request.local_addr.ip}:{item.request.local_addr.port} » {item.request.real_ip}
          </div>
          <a class="na deployment" data-sveltekit-replacestate href="{deployment_toggle_link(item)}">
            {#if data.stream_connections_query.deployment_id === item.deployment_id}
              «
            {/if}
            Deployment #{item.deployment_id}
          </a>
          <div class="platform">
            {#if item.request.user_agent.name && item.request.user_agent.os}
              {item.request.user_agent.name} on {item.request.user_agent.os}
            {:else if item.request.user_agent.name}
              {item.request.user_agent.name}
            {:else if item.request.user_agent.os}
              {item.request.user_agent.name}
            {:else}
              Unknown platform
            {/if}
          </div>
          <a class="na referer" data-sveltekit-replacestate href={referer_toggle_link(referer)} >
            {#if data.stream_connections_query.referer === String(referer)}
              «
            {/if}
            {referer ?? "Unknown referer"}
          </a>
          <div class="duration">
            {duration(item, $now)}
          </div>
        </div>
      </div>
    {/each}
  </div>
</Page>