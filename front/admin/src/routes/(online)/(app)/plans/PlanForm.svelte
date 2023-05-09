<script lang="ts">
	import BooleanField from "$lib/components/Form/BooleanField.svelte";
	import NullNumberField from "$lib/components/Form/Nullable/NullNumberField.svelte";
  export let current: {
    identifier: string
    display_name: string
    price: number | null
    stations: number | null
    listeners: number | null
    transfer: number | null 
    storage: number | null
    is_user_selectable: boolean,
  }
  import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _number, _string } from "$share/formy/validate";
</script>

<style>
  .field {
    padding: 1rem;
  }
</style>

<div class="field">
  <TextField label="Identifier" trim bind:value={current.identifier} />
  <Validator value={current.identifier} fn={_string({ required: true })} />
</div>

<div class="field">
  <TextField label="Display Name" trim bind:value={current.display_name} />
  <Validator value={current.display_name} fn={_string({ required: true })} />
</div>

<div class="field">
  <NullNumberField label="Price" step={1} min={0} bind:value={current.price} />
  <Validator value={current.price} fn={_number({ min: 0, required: true })} />
</div>

<div class="field">
  <NullNumberField label="Stations Limit" step={1} min={0} bind:value={current.stations} />
  <Validator value={current.stations} fn={_number({ min: 0, required: true })} />
</div>

<div class="field">
  <NullNumberField label="Listeners Limit" step={100} min={0} bind:value={current.listeners} />
  <Validator value={current.listeners} fn={_number({ min: 0, required: true })} />
</div>

<div class="field">
  <NullNumberField label="Transfer Limit" step={1_000_000_000_000} min={0} bind:value={current.transfer} />
  <Validator value={current.transfer} fn={_number({ min: 0, required: true })} />
</div>

<div class="field">
  <NullNumberField label="Storage Limit" min={0} step={1_000_000_000} bind:value={current.storage} />
  <Validator value={current.storage} fn={_number({ min: 0, required: true })} />
</div>

<div class="field">
  <BooleanField label="Is user selectable?" bind:value={current.is_user_selectable} />
</div>