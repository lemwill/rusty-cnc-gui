<script lang="ts">
  import Toolpath from "../lib/Toolpath.svelte";

  import {
    point_xy_plane,
    point_xz_plane,
    point_yz_plane,
  } from "../lib/Toolpath.svelte";
</script>

<main>
  <Toolpath />
  <button class="btn variant-filled" on:click={point_xy_plane}>XY Plane</button>

  <button class="btn variant-filled" on:click={point_yz_plane}>YZ Plane</button>
  <button class="btn variant-filled" on:click={point_xz_plane}>XZ Plane</button>

  <!-- the rest of your MainPage code -->
</main>
