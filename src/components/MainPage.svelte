<script lang="ts">
  import Toolpath from "./Toolpath.svelte";
  import UploadGCodeButton from "./UploadGCodeButton.svelte";

  import {
    point_xy_plane,
    point_xz_plane,
    point_yz_plane,
  } from "./Toolpath.svelte";
</script>

<main>
  <Toolpath />

  <button class="btn variant-filled" on:click={point_xy_plane}>XY Plane</button>

  <button class="btn variant-filled" on:click={point_yz_plane}>YZ Plane</button>
  <button class="btn variant-filled" on:click={point_xz_plane}>XZ Plane</button>
  <UploadGCodeButton />
  <!-- the rest of your MainPage code -->
</main>
