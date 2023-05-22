<script lang="ts">
  export let data: import("./$types").LayoutData;
  import "$lib/css/app.css";

  $: meta_lang = data.locale.region ? `${data.locale.lang}-${data.locale.region}` : data.locale.lang;
</script>

<svelte:head>
  <meta http-equiv="content-language" content={meta_lang} /> 
</svelte:head>

<slot />