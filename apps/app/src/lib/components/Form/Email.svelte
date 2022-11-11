<script lang="ts">
	export let value: string;
  export let label: string;

  import FieldContainer from "./FieldContainer.svelte";
  import Label from "./Label.svelte";
  import styles from "./forms.module.css"; 
</script>


<FieldContainer>
  <input type="email" class={styles["forms-input"]} bind:value={value} />
  <Label {label} full={value !== ""} />
</FieldContainer>