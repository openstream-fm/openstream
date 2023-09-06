<script lang="ts">
  // @ts-ignore
  import bg from "$share/img/login-bg.jpg?w=1920&format=webp&withoutEnlargement";
  // @ts-ignore
  import logo from "$share/img/logo-trans-128.png?w=36&format=webp";
  import { locale } from "$share/locale";
</script>



<style>
  .login-dashboard {
    flex: 1;
    display: flex;
    align-self: stretch;
    justify-self: stretch;
    flex-direction: column;
    /* padding-inline: min(5%, max(12rem, 10%)); */
    padding-block: 8rem;
    position: relative;
    --field-container-bg: rgba(0,0,0,0);
    overflow: hidden;
    background: #fff;
  }

  .bg {
    position: absolute;
    z-index: var(--z-login-bg);
    pointer-events: none;
    inset-block-start: 0;
    inset-inline-start: 0;
    width: 100vw;
    height: 100vh;
    background-size: cover;
    background-position: center;
  }

  .logo {
    position: absolute;
    z-index: var(--z-login-front);
    display: flex;
    flex-direction: row;
    align-items: center;
    inset-block-start: 2rem;
    inset-inline-start: 2rem;
  }

  .logo-icon {
    flex: none;
    width: 2.25rem;
    height: 2.25rem;
    margin-inline-end: 0.5rem;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
  }

  .logo-text {
    font-size: 1.75rem;
    font-weight: 600;
    color: #000;
  }

  .content {
    position: relative;
    z-index: var(--z-login-front);
    display: flex;
    flex-direction: column;
    flex-grow: 1;
  }

  @media screen and (max-width: 600px) {
    .login-dashboard {
      padding-inline: 0;
    }
  }
</style>

<div class="login-dashboard">
  <div class="bg" style:background-image="url({ bg })" />
  <div class="logo">
    <div class="logo-icon" style:background-image="url({ logo })" />
    <div class="logo-text">{$locale.logo_text}</div>
  </div>
  <div class="content">
    <slot />
  </div>
</div>