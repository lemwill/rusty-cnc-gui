<script lang="ts">
  import ConfigurationView from "../components/ConfigurationView.svelte";
  import SidebarRight from "../components/SidebarRight.svelte";
  import MainPage from "../components/MainPage.svelte";

  import "@skeletonlabs/skeleton/styles/all.css";

  import {
    AppBar,
    AppRail,
    AppRailTile,
    AppShell,
    LightSwitch,
  } from "@skeletonlabs/skeleton";

  import { writable, type Writable } from "svelte/store";

  const storeValue: Writable<number> = writable(0);
  let selectedView: number;
  storeValue.subscribe((value) => {
    selectedView = value;
  });
</script>

<main>
  <AppShell>
    <svelte:fragment slot="header"><AppBar>Rusty CNC</AppBar></svelte:fragment>
    <svelte:fragment slot="sidebarLeft">
      <AppRail selected={storeValue}>
        <AppRailTile label="Cutting" value={0} />
        <AppRailTile label="Config" value={1} />
      </AppRail>
    </svelte:fragment>

    <svelte:fragment slot="pageHeader" />
    <svelte:fragment slot="sidebarRight">
      <SidebarRight />
    </svelte:fragment>

    {#if selectedView === 0}
      <MainPage />
    {:else if selectedView === 1}
      <ConfigurationView />
    {/if}

    <slot />

    <svelte:fragment slot="pageFooter" />
    <svelte:fragment slot="footer" />
  </AppShell>
</main>

<style>
  main {
    height: 100vh; /* Set main height to fill the viewport */
    display: flex;
  }

  /* Add custom styles to fill the sidebarLeft */
  .appshell-sidebar-left {
    flex: 0 0 auto;
    height: 100%;
    overflow-y: auto;
  }

  /* Adjust the content area to fill the remaining space */
  .appshell-content {
    flex: 1 1 auto;
    height: 100%;
    overflow-y: auto;
  }
</style>
