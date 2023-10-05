<svelte:options immutable={true} />

<script>
  //type Item = $$Generic;
  //type Fields = $$Generic<Record<string, any>>;
  //type Key = keyof Fields;
  // import type { DataGridData, DataGridField  } from "./types";

  //export let data: DataGridData<Item, Fields>;
  //export let locale: import("$server/locale/share/analytics/analytics.locale").AnalyticsLocale["data_grid"];
  
  /** @type any */
  export let data;
  /** @type any */
  export let locale;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
  import { mdiClose, mdiFileDownloadOutline, mdiTriangle } from "@mdi/js";
  import { stringify } from "csv-stringify/browser/esm/sync";

  // const inverse = <T>(fn: (a: T, b: T) => number) => {
  //   return (a: T, b: T) => fn(a, b) * -1;
  // };

  // @ts-ignore
  const inverse = (fn) => (a, b) => fn(a, b) * - 1; 
  //   return (a: T, b: T) => fn(a, b) * -1;
  // };

  // @ts-ignore
  $: items = get_items(data);
  // const get_items = (...args: any[]): typeof data.items => {
  const get_items = () => {
    if (data.sorted_by?.key == null) return data.items;
    let sort_fn = data.fields[data.sorted_by.key].sort;
    if (sort_fn == null) return data.items;
    if (data.sorted_by.direction === "desc") sort_fn = inverse(sort_fn);
    return data.items.slice().sort(sort_fn);
  };

  // const as_key = (k: string) => {
  //   return k as Key;
  // }

  // const toggle_sort = (key: Key) => {
  // @ts-ignore
  const toggle_sort = (key) => {
    const field = data.fields[key];
    if (field == null) return;
    if (data.sorted_by?.key === key && data.sorted_by?.direction === "asc") {
      data = { ...data, sorted_by: { key, direction: "desc" } };
    } else {
      data = { ...data, sorted_by: { key, direction: "asc" } };
    }
  };

  // let fields: [Key, DataGridField<Item>][];
  $: fields = Object.entries(data.fields);
  $: display_fields = fields.filter(([key, field]) => !field.csv_only);

  const do_export = () => {
    const fs = Object.values(data.fields);
    const headers = fs.map((field) => field.name);
    // @ts-ignore
    const rows = items.map((item) => fs.map((field) => field.format(item)));
    const out = stringify([headers, ...rows]);
    const url = `data:text/csv;charset=utf-8,${encodeURIComponent(out)}`;
    const a = document.createElement("a");
    a.href = url;
    a.download = `${data.title}.csv`;
    a.click();
  };

  let scrollX = 0;
</script>

<style>
  table {
    min-width: 100%;
    border-collapse: collapse;
  }

  thead > tr > th {
    background-color: #f0f0f0;
  }

  tbody > tr:nth-child(even) > td {
    background-color: #f7f7f7;
  }

  tbody > tr:nth-child(odd) > td {
    background-color: #ffffff;
  }

  th {
    text-align: left;
    font-weight: 400;
    font-size: 0.9rem;
    font-weight: 500;
  }

  td {
    font-size: 0.9rem;
  }
  
  .value {
    padding: 0.5rem 1rem;
  }

  .value.clickable {
    cursor: pointer;
    text-align: inherit;
  }

  .value.clickable:hover {
    text-decoration: underline;
  }

  .grid {
    width: 100%;
    overflow-y: auto;
    overflow-x: auto;
  }

  .header {
    min-width: 100%;
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    text-align: left;
  }

  .sort-chevron {
    display: flex;
    flex: none;
    transition: transform 150ms ease;
    margin-inline-start: 0.25rem;
    margin-inline-end: -1rem;
    font-size: 0.75rem;
    transform: scaleX(0.75);
  }

  :global([dir=rtl]) .value-selected-icon {
    transform: scaleX(-1);
  }

  .sort-chevron.asc {
    transform: scaleX(0.75) scaleY(-1);
    color: #000;
  }

  .numeric .header {
    justify-content: flex-end;
    text-align: right;
  }

  .header:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .numeric {
    text-align: right;
  }

  td.numeric {
    font-family: var(--monospace);
    font-size: 0.75rem;
  }

  td:not(.numeric) {
    font-weight: 500;
  }

  th:first-child, td:first-child {
    position: sticky;
    z-index: 1;
    left: 0;
    max-width: min(18rem, 35vw);
    overflow-wrap: break-word;
    padding-inline-end: 0.5rem;
  }

  th:first-child > *, td:first-child > * {
    max-width: 100%;
  }
  
  
  .scrolled-x th:first-child, .scrolled-x td:first-child {
    background-image: linear-gradient(
      to right, transparent 0%, transparent calc(100% - 0.75rem), rgba(0,0,0,0.025) 100% 
    );
  }

  .export-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-top: 1rem;
  }

  .export {
    background: #3d9970;
    color: #fff;
    padding: 0.75rem;
    font-weight: 600;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .export-icon {
    display: flex;
    margin-inline-end: 0.5rem;
    font-size: 1.25rem;
  }
</style>

<div class="grid-out">
  <div class="grid thin-scroll" class:scrolled-x={scrollX !== 0} on:scroll={event => scrollX = event.currentTarget.scrollLeft} >
    <table>
      <thead>
        <tr>
          {#each display_fields as [key, field] (key)}
            <th class:numeric={field.numeric}>
              {#if field.sort != null}
                <button
                  class="header sortable ripple-container"
                  use:ripple
                  on:click={() => toggle_sort(key)}
                >
                  {field.name}
                  {#if data.sorted_by?.key === key}
                    <div
                      class="sort-chevron"
                      class:asc={data.sorted_by?.direction === "asc"}
                      class:desc={data.sorted_by?.direction === "desc"}
                    >
                      <Icon d={mdiTriangle} />
                    </div>
                  {/if}
                </button>
              {:else}
                <div class="header">
                  {field.name}
                </div>
              {/if}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each items as item}
          <tr>
            {#each display_fields as [key, field] (key)}
              <td class:numeric={field.numeric}>
                {#if field.on_click}
                  <button class="value clickable" on:click={() => field.on_click(item)}>
                    {#if field.is_selected?.(item)}
                      <span class="value-selected-icon">
                        «
                      </span>
                    {/if}
                    {field.format(item)}
                  </button>  
                {:else}
                  <div class="value">
                    {field.format(item)}
                  </div>
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="export-out">
    <button class="export ripple-container" use:ripple on:click={do_export}>
      <div class="export-icon">
        <Icon d={mdiFileDownloadOutline} />
      </div>
      {locale.export_as_csv}
    </button>
  </div>
</div>