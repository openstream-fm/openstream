<svelte:options immutable={true} />

<script lang="ts" context="module">
  export type DataGridField<T> = {
    name: string,
    format: (v: T) => string
    sort?: ((a: T, b: T) => number) | null
    numeric?: boolean,
  }

  export type DataGridData<T, R extends Record<string, any>> = {
    title: string,
    fields: Record<keyof R, DataGridField<T>>,
    items: T[],
    sorted_by?: {
      key: keyof R,
      direction: "asc" | "desc",
    }
  }
</script>

<script lang="ts">
  type Item = $$Generic;

  type Fields = $$Generic<Record<string, any>>;

  type Key = keyof Fields;

  export let data: DataGridData<Item, Fields>;
  export let locale: import("$server/locale/share/analytics/analytics.locale").AnalyticsLocale["data_grid"];

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
  import { mdiFileDownloadOutline, mdiTriangle } from "@mdi/js";
  import { stringify } from "csv-stringify/browser/esm/sync";
      

  const inverse = <T>(fn: (a: T, b: T) => number) => {
    return (a: T, b: T) => fn(a, b) * -1;
  }

  $: items = get_items(data);
  const get_items = (...args: any[]): typeof data.items => {
    if(data.sorted_by?.key == null) return data.items;
    let sort_fn = data.fields[data.sorted_by.key].sort;
    if(sort_fn == null) return data.items;
    if(data.sorted_by.direction === "desc") sort_fn = inverse(sort_fn);
    return data.items.slice().sort(sort_fn);
  }

  // const as_key = (k: string) => {
  //   return k as Key;
  // }

  const toggle_sort = (key: Key) => {
    const field = data.fields[key];
    if(field == null) return;
    if(data.sorted_by?.key === key && data.sorted_by?.direction === "asc") {
      data = { ...data, sorted_by: { key, direction: "desc" } };
    } else {
      data = { ...data, sorted_by: { key, direction: "asc" } };
    }
  }

  let fields: [Key, DataGridField<Item>][];
  $: fields = Object.entries(data.fields);

  const do_export = () => {
    const fs = Object.values(data.fields);
    const headers = fs.map(field => field.name);
    const rows = items.map(item => fs.map(field => field.format(item)));
    const out = stringify([ headers, ...rows ]);
    const url = `data:text/csv;charset=utf-8,${encodeURIComponent(out)}`;
    const a = document.createElement("a");
    a.href = url;
    a.download = `${data.title}.csv`;
    a.click();
  }
</script>

<style>
  table {
    min-width: 100%;  
    border-collapse: collapse;
  }

  thead {
    background: rgba(0,0,0,0.06);
  }

  tbody > tr:nth-child(even) {
    background: rgba(0,0,0,0.03);
  }

  th {
    text-align: left;
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
    transition: transform 150ms ease;
    margin-inline-start: 0.25rem;
    margin-inline-end: -1rem;
    font-size: 0.75rem;
    transform: scaleX(0.75);
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
    background: rgba(0,0,0,0.05);
  }

  .value {
    padding: 0.5rem 1rem;
  }

  .numeric {
    text-align: right;
  }

  .export-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-top: 1rem;
  }

  .export {
    background: #3D9970;
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
  <div class="grid thin-scroll">
    <table>
      <thead>
        <tr>
          {#each fields as [key, field] (key)}
            <th class:numeric={field.numeric}>
              {#if field.sort != null}
                <button class="header sortable ripple-container" use:ripple on:click={() => toggle_sort(key)}>
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
            {#each fields as [key, field] (key)}
              <td class:numeric={field.numeric}>
                <div class="value">
                  {field.format(item)}
                </div>
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