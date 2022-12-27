<script lang="ts">
	export let value: string;
  export let label: string;
  export let visible: boolean = false;

	import Icon from "$share/Icon.svelte";
  import FieldContainer from "./FieldContainer.svelte";
  import Label from "./Label.svelte";

  import { mdiEye, mdiEyeOff } from "@mdi/js";
  import css from "./forms.module.css"; 
  
  let clickToken = false;

  const pointerdown = () => {
    clickToken = true;
    visible = !visible;
  }

  // handle enter key
  const click = () => {
    let t = clickToken;
    clickToken = false;
    if (!t) {
      visible = !visible;
    }
  }
</script>

<style>
  .wrap {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    flex: 1;
  }

  .btn {
    user-select: none;
    cursor: pointer;
    flex: none;
    appearance: none;
    background: transparent;
    padding: 0;
    border: 0;
    margin: 0;
    width: 2.5rem;
    font-size: 1.5em;
    border-radius: 0.25em;
    align-self: stretch;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #bbb;
    transition: background-color 200ms ease, color 200ms ease;
  }

  .btn:hover {
    color: #888;
  }
</style>

<FieldContainer>
  <div class="wrap">
    <input type={visible ? "text" : "password"} class={css["forms-input"]} value={value} on:input={event => value = event.currentTarget.value} />
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="btn" on:pointerdown|capture|preventDefault={pointerdown} on:click={click}>
      <Icon d={visible ? mdiEyeOff : mdiEye} />
    </div>
  </div>
  <Label {label} full={value !== ""} />
</FieldContainer>