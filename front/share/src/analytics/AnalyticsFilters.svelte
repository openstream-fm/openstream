<script lang="ts" context="module">
  export type QueryKind =
    | "today"
    | "yesterday"
    | "last-24h"
    | "this-week"
    | "previous-week"
    | "last-7d"
    | "this-month"
    | "previous-month"
    | "last-30d"
    | "custom";

  export type StationItem = {
    _id: string;
    name: string;
    picture_id: string;
  };

  export type ResolvedQuery = {
    since: Date | null;
    until: Date | null;
    stations: StationItem[];
  };

  export type OnSubmitEvent = ResolvedQuery & { qs: URLSearchParams | null };

  export const to_querystring = (
    query: ResolvedQuery
  ): URLSearchParams | null => {
    if (query.since == null) return null;
    if (query.until == null) return null;

    const qs = new URLSearchParams();
    qs.append("since", formatISO(query.since));
    qs.append("until", formatISO(query.until));
    for (const station of query.stations) {
      qs.append("stations[]", station._id);
    }

    return qs;
  };
</script>

<script lang="ts">
  export let stations: StationItem[];

  export let selected_stations: "all" | StationItem[];
  export let kind: QueryKind;
  export let custom_since: Date | null = null;
  export let custom_until: Date | null = null;
  export let loading: boolean = false;
  export let locale: AnalyticsLocale["filters"];

  export const get_resolved_since = (now = new Date()) => {
    if (kind === "today") {
      return startOfDay(now);
    } else if (kind === "last-24h") {
      return sub(now, { hours: 24 });
    } else if (kind === "yesterday") {
      return sub(startOfDay(now), { hours: 24 });
    } else if (kind === "this-week") {
      return startOfWeek(now);
    } else if (kind === "last-7d") {
      return sub(now, { days: 7 });
    } else if (kind === "previous-week") {
      return sub(startOfWeek(now), { days: 7 });
    } else if (kind === "this-month") {
      return startOfMonth(now);
    } else if (kind === "last-30d") {
      return sub(now, { days: 30 });
    } else if (kind === "previous-month") {
      return startOfMonth(sub(startOfMonth(now), { days: 1 }));
    } else if (kind === "custom") {
      return custom_since;
    } else {
      return assert_never(kind, "AnalytisFilter get_resolved_since() kind");
    }
  };

  export const get_resolved_until = (now = new Date()) => {
    if (kind === "today") {
      return now;
    } else if (kind === "last-24h") {
      return now;
    } else if (kind === "yesterday") {
      return startOfDay(now);
    } else if (kind === "this-week") {
      return now;
    } else if (kind === "last-7d") {
      return now;
    } else if (kind === "previous-week") {
      return startOfWeek(now);
    } else if (kind === "this-month") {
      return now;
    } else if (kind === "last-30d") {
      return now;
    } else if (kind === "previous-month") {
      return startOfMonth(now);
    } else if (kind === "custom") {
      return custom_until;
    } else {
      return assert_never(kind, "AnalytisFilter get_resolved_until() kind");
    }
  };

  export const get_resolved_stations = () =>
    selected_stations === "all" ? stations : selected_stations;
  
  export const get_resolved_query = (): ResolvedQuery => {
    return {
      since: get_resolved_since(),
      until: get_resolved_until(),
      stations: get_resolved_stations(),
    };
  };

  export const get_resolved_qs = (): URLSearchParams | null =>
    to_querystring(get_resolved_query());

  export let on_submit: (event: OnSubmitEvent) => void;

  import {
    formatISO,
    startOfDay,
    startOfMonth,
    startOfWeek,
    sub,
  } from "date-fns";
  import { assert_never } from "../../assert-never";
  import { page } from "$app/stores";
  import Icon from "$share/Icon.svelte";
  import {
    mdiCheckBold,
    mdiCheckboxBlankOutline,
    mdiClose,
    mdiPoll,
    mdiRadioboxBlank,
  } from "@mdi/js";
  import { ripple } from "$share/ripple";
  import { scale, slide } from "svelte/transition";
  import { logical_fly } from "$share/transition";
  import { click_out } from "$share/actions";
  import CircularProgress from "$share/CircularProgress.svelte";
  import { AnalyticsLocale } from "$server/locale/share/analytics/analytics.locale";

  const unselect_station = (id: string) => {
    if (selected_stations === "all") {
      selected_stations = stations.filter((station) => station._id !== id);
    } else {
      selected_stations = selected_stations.filter(
        (station) => station._id !== id
      );
    }
  };

  const select_station = (item: StationItem) => {
    if (selected_stations === "all") {
      selected_stations = [item];
    } else if (!selected_stations.some((station) => station._id === item._id)) {
      selected_stations = [...selected_stations, item];
    }
  };

  const toggle_station = (item: StationItem) => {
    if (
      selected_stations === "all" ||
      !selected_stations.some(station => station._id === item._id)
    ) {
      select_station(item);
    } else {
      unselect_station(item._id);
    }
  };

  $: if (selected_stations !== "all" && selected_stations.length === 0)
    selected_stations = "all";

  let stations_menu_open = false;
  let time_menu_open = false;

  const stations_menu_click_out = () => {
    setTimeout(() => {
      stations_menu_open = false;
    }, 2)
  }

  const time_menu_click_out = () => {
    setTimeout(() => {
      time_menu_open = false;
    }, 2)
  }

  const temporal_keys = [
    "last-24h",
    "last-7d",
    "last-30d",
    "today",
    "this-week",
    "this-month",
    "yesterday",
    "previous-week",
    "previous-month",
    "custom",
  ] as const;

  const submit = () => {
    const query = get_resolved_query();
    const qs = to_querystring(query);
    on_submit({ ...query, qs });
  }
</script>

<style>
  .analytics-filters {
    display: flex;
    flex-direction: column;
    align-self: stretch;
    align-items: stretch;
    gap: 1rem;
  }

  .field-out {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    position: relative;
  }

  .field {
    display: block;
    width: auto;
    border: rgba(0, 0, 0, 0.25) 1px solid;
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    text-align: start;
    transition: border-color 200ms ease;
  }

  .field:focus-within {
    border-color: var(--blue);
  }

  .field-text {
    line-height: 37px;
    height: 37px;
    font-size: 1.1rem;
    color: rgba(0, 0, 0, 0.8);
  }

  .menu {
    z-index: 1;
    position: absolute;
    width: 100%;
    inset-block-start: 100%;
    inset-inline-start: 0;
    display: flex;
    flex-direction: column;
    padding: 0.5rem;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    background: #fff;
    gap: 0.5rem;
  }

  .menu-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0.75rem 0.5rem;
    border-radius: 0.25rem;
  }
  
  .menu-item.selected {
    background: rgba(var(--blue-rgb), 0.1);
  }

  .menu-check {
    width: 1.5rem;
    height: 1.5rem;
    position: relative;
    display: flex;
    margin-inline-end: 0.75rem;
  }

  .menu-check-icon {
    display: flex;
    font-size: 1.5rem;
    position: absolute;
    top: 0;
    left: 0;
    transition: color 200ms ease;
  }

  .menu-item.selected .menu-check-icon {
    color: var(--green);
  }

  .menu-pìc {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 0.25rem;
    margin-inline-end: 0.75rem;
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
  }

  .no-stations-message {
    padding: 1rem;
  }

  .chips {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .chip {
    background: #eee;
    border-radius: 3rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    padding: 0.5rem;
  }

  .chip-pic {
    width: 1.35rem;
    height: 1.35rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
    border-radius: 50%;
    margin-inline-end: 0.5rem;
    flex: none;
  }

  .chip-btn {
    display: flex;
    padding: 1.25rem;
    margin: -1rem;
    border-radius: 50%;
    flex: none;
  }

  .submit-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .submit {
    background: var(--blue);
    color: #fff;
    font-weight: 600;
    padding: 0.75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow); 
    position: relative;
  }

  .submit-text {
    transition: opacity 200ms ease;
  }

  .submit-icon {
    font-size: 1.25rem;
    display: flex;
    margin-inline-end: 0.5rem;
    transition: opacity 200ms ease;
  }

  .loading .submit-text, .loading .submit-icon {
    opacity: 0;
  }

  .submit-loading {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 1.4rem;
  }
</style>

<div class="analytics-filters" class:loading>
  <div class="field-out">
    <button
      class="field ripple-container"
      use:ripple
      on:click={() => {
        stations_menu_open = !stations_menu_open
      }}
    >
      {#if selected_stations === "all"}
        <div class="field-text" transition:slide|local={{ duration: 200 }}>
          {#if stations.length}
            {locale.All_stations}
          {:else}
            {locale.No_stations}
          {/if}
        </div>
      {:else}
        <div class="chips" transition:slide|local={{ duration: 200 }}>
          {#each selected_stations as station (station._id)}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="chip" transition:scale|local={{ duration: 200 }}>
              <div
                class="chip-pic"
                style:background-image="url({$page.data.config
                  .storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)"
              />
              <div class="chip-name">
                {station.name}
              </div>
              <button class="chip-btn ripple-container" use:ripple on:click|stopPropagation|preventDefault={() => unselect_station(station._id)}>
                <Icon d={mdiClose} />
              </button>
            </div>
          {/each}
        </div>
      {/if}
    </button>

    {#if stations_menu_open}
      <div class="menu" transition:logical_fly={{ y: -25, duration: 200 }} use:click_out={() => stations_menu_click_out()}>
        {#each stations as station (station._id)}
          {@const selected =
            selected_stations !== "all" &&
            selected_stations.some(item => item._id === station._id)}
          <button
            class="menu-item ripple-container"
            class:selected
            use:ripple
            on:click|stopPropagation|preventDefault={() => toggle_station(station)}
          >
            <div class="menu-check">
              {#if selected}
                <div
                  class="menu-check-icon"
                  transition:scale|local={{ duration: 300 }}
                >
                  <Icon d={mdiCheckBold} />
                </div>
              {:else}
                <div
                  class="menu-check-icon"
                  transition:scale|local={{ duration: 300 }}
                >
                  <Icon d={mdiCheckboxBlankOutline} />
                </div>
              {/if}
            </div>
            <div
              class="menu-pìc"
              style:background-image="url({$page.data.config
                .storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)"
            />
            <div class="menu-name">
              {station.name}
            </div>
          </button>
        {:else}
          <div class="no-stations-message">
            {locale.no_stations_message}
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <div class="field-out">
    <button
      class="field ripple-container"
      use:ripple
      on:click={() => {
        time_menu_open = !time_menu_open
      }}
    >
      <div class="field-text">
        {locale.query_kind[kind]}
      </div>
    </button>

    {#if time_menu_open}
      <div class="menu" transition:logical_fly={{ y: -25, duration: 200 }} use:click_out={() => time_menu_click_out()}>
        {#each temporal_keys as key (key)}
          {@const selected = kind === key}
          {@const name = locale.query_kind[key]}
          <button
            class="menu-item ripple-container"
            class:selected
            use:ripple
            on:click|stopPropagation|preventDefault={() => {
              kind = key;
              time_menu_open = false;
            }}
          >
            <div class="menu-check">
              {#if selected}
                <div
                  class="menu-check-icon"
                  transition:scale|local={{ duration: 300 }}
                >
                  <Icon d={mdiCheckBold} />
                </div>
              {:else}
                <div
                  class="menu-check-icon"
                  transition:scale|local={{ duration: 300 }}
                >
                  <Icon d={mdiRadioboxBlank} />
                </div>
              {/if}
            </div>
            <div class="menu-name">
              {name}
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="submit-out">
    <button class="submit ripple-container" use:ripple on:click={submit}>
      <div class="submit-icon">
        <Icon d={mdiPoll} />
      </div>
      <div class="submit-text">
        {locale.submit}
      </div>
      {#if loading}
        <div class="submit-loading" transition:scale|local={{ duration: 200 }}>
          <CircularProgress />
        </div>
      {/if}
    </button>
  </div>
</div>
