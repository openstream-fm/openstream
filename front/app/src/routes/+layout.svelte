<script lang="ts">
  export let data: import("./$types").LayoutData;
  import "$lib/css/app.css";

</script>

<svelte:head>
  <meta http-equiv="content-language" content={data.lang} /> 
</svelte:head>

<slot />