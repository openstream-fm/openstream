<script lang="ts">
  type Item = import("./$types").PageData["api_keys"]["items"][number];
  export let key: Item;
  export let on_remove: ((() => void) | null) = null;

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { tooltip } from "$share/tooltip";
	import { mdiKeyOutline, mdiTrashCanOutline } from "@mdi/js";

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

  $: data = get_data(key);
  const get_data = (..._args: any[]): Array<{ label: string, value: string }> => {
    const data: Array<{ label: string, value: string }> = [];
    
    data.push({
      // TODO: locale
      label: "Title",
      value: key.title,
    })
   
    data.push({
      // TODO: locale
      // label: $locale.pages["me/devices"].device.connected,
      label: "Created",
      value: format_date(key.created_at),
    })

    data.push({
      // TODO: locale,
      // label: $locale.pages["me/devices"].device.last_used,
      label: "Last used",
      value: format_date(key.last_used_at || key.created_at)
    })
    
    return data;
  }
</script>



<style>
  .key {
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
    font-size: 2rem;
    align-items: center;
    justify-content: center;
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
  <div class="key" class:current={key.is_current}>
    <div class="icon">
      <Icon d={mdiKeyOutline} />
    </div>
    <div class="id">
      <span class="id-box">
        {key._id}-...
      </span>
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
        use:tooltip={
          // TODO: locale
          // $locale.pages["me/devices"].device.tooltips.disconnect
          "Delete"
        }
        aria-label={
          // TODO: locale
          // $locale.pages["me/devices"].device.tooltips.disconnect
          "Delete API key"
        }
        on:click={on_remove}
      >
        <Icon d={mdiTrashCanOutline} />
      </button>
    {/if}
  </div>
</Page>