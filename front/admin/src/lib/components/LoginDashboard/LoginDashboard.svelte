<style>
  .dashboard {
    flex: 1;
    display: flex;
    align-self: stretch;
    justify-self: stretch;
    flex-direction: column;
    padding-inline: min(20%, max(12rem, 10%));
    padding-block: 6.5rem;
    position: relative;
    background-image: url($lib/img/login-bg.jpg);
    background-size: cover;
    background-position: center;
    --field-container-bg: rgba(0,0,0,0);
  }

  .logo {
    position: absolute;
    inset-block-start: 2rem;
    inset-inline-start: 2rem;
    font-size: 2rem;
    font-weight: var(--font-bold);;
    color: var(--red);
    text-shadow: #fff 0 0 3px;
  }

  @media screen and (max-width: 600px) {
    .dashboard {
      padding-inline: 0;
    }
  }
</style>

<div class="dashboard">
  <div class="logo">openstream</div>
  <slot />
</div>