<script lang="ts">
  export let value: string;
  export let label: string;
  export let required: boolean = false;
  export let icon: string = mdiPaletteOutline;

  import TextField from "$share/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { locale } from "$share/locale";
  import { mdiPaletteOutline } from "@mdi/js";

  import Color from "color";

  const _color_validation = (value: string) => {
    
    if(value === "") {
      if(required) {
        return $locale.validate.required;
      }
    } else {
      try {
        new Color(value);
      } catch(e: any) {
        return $locale.validate.css_color;
      }
    }
    
    return null;
  }
</script>

<div class="color-field">
  <TextField {label} {icon} trim bind:value />
  <Validator {value} fn={_color_validation} />
</div>
