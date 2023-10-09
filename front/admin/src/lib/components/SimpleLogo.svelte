<script lang="ts">
	import { locale } from "$lib/locale";

  // @ts-ignore
  import logo from "$share/img/logo-trans-128.png?w=32&format=webp";
</script>

<style>
  .logo {
    color: var(--red);
    font-size: min(6vw, 2rem);
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .logo-icon {
    width: 1em;
    height: 1em;
    flex: none;
    background-size: contain;
    background-repeat: no-repeat;
    background-position: center;
    margin-inline-end: 0.25em;
  }

  .logo-text {
    color: #000;
    font-size: 0.8em;
    font-weight: var(--font-bold);
  }
</style>

<div class="logo">
  <div class="logo-icon" style:background-image="url({ logo })" />
  <div class="logo-text">
    {$locale.logo_text}
  </div>
</div>

