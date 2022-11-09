<script lang="ts">
	export let value: string;
  export let label: string;
  export let visible: boolean = false;

	import Icon from "../Icon.svelte";
  import FieldContainer from "./FieldContainer.svelte";
  import Label from "./Label.svelte";

  import { mdiEye, mdiEyeOff } from "@mdi/js";
</script>

<style>
  .wrap {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    flex: 1;
  }

  input {
    display: block;
    flex: 1;
    border: 0;
    outline: 0;
    padding: var(--spacing);
    min-width: 0;
    background: transparent;
  }

  button {
    user-select: none;
    cursor: pointer;
    flex: none;
    appearance: none;
    background: transparent;
    padding: 0;
    border: 0;
    margin: 0;
    outline: 0;
    width: 3rem;
    font-size: 1.5em;
    border-radius: 0.25em;
    align-self: stretch;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #ccc;
    transition: background-color 200ms ease, color 200ms ease;
  }

  button:hover {
    color: #999;
  }
</style>

<FieldContainer>
  <div class="wrap">
    <input type={visible ? "text" : "password"} value={value} on:input={event => value = event.currentTarget.value} />
    <button on:pointerdown|capture|preventDefault={() => visible = !visible}>
      <Icon d={visible ? mdiEyeOff : mdiEye} />
    </button>
  </div>
  <Label {label} full={value !== ""} />
</FieldContainer>