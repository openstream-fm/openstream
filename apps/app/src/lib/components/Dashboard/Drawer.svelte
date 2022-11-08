<script lang="ts">
	import DrawerItem from "./DrawerItem.svelte";
  import { 
    mdiViewDashboardOutline,
    mdiRadioTower,
    mdiPoll,
    mdiAccountOutline,
    mdiShieldAccountOutline,
  } from "@mdi/js";
</script>

<style>
  .drawer {
    position: relative;
    width: var(--drawer-width);
    flex: none;
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    z-index: var(--z-drawer-wide);
  }

  .top {
    height: var(--top-height);
    font-weight: 600;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding: 0.5rem 0 0 2rem;
  }

  .inner {
    position: sticky;
    top: 0;
    height: 100vh;
  }

  @media screen and (max-width: 900px) {
    .drawer {
      margin-inline-start: calc(var(--drawer-width) * -1);
      box-shadow: none;
    }
  }

</style>

<div class="drawer">
  <div class="inner">
    <div class="top">
      openstream
    </div>

    <div class="links">
      <DrawerItem href="/" label="Dashboard" icon={mdiViewDashboardOutline} />
      <DrawerItem href="/profile" label="Profile" icon={mdiAccountOutline} />
      <DrawerItem href="/account" label="Account" icon={mdiShieldAccountOutline} />
      <DrawerItem href="/stations" label="Stations" icon={mdiRadioTower} />
      <DrawerItem href="/analytics" label="Analytics" icon={mdiPoll} />
    </div>
  </div>
</div>