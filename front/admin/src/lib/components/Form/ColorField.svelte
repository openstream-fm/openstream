<script lang="ts">
  export let value: string;
  export let label: string;
  export let required: boolean;

  import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	
  import Color from "color";

  const _color_validation = (value: string) => {
    
    if(value === "") {
      if(required) {
        // TODO: locale
        return "This field is required";
      }
    } else {
      try {
        new Color(value);
      } catch(e: any) {
        // TODO: locale
        return "This field must be a valid CSS color, e.g. #ffffff or rgba(0,0,0,0)"
      }
    }
    
    return null;
  }
</script>

<div class="color-field">
  <TextField {label} trim bind:value />
  <Validator {value} fn={_color_validation} />
</div>
