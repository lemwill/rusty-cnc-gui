<script>
  import { position } from "../lib/socketStore.js";
</script>

<div>
  <p>
    <span class="badge variant-filled">X: {$position.x.toFixed(1)}</span>
    <span class="badge variant-filled">Y: {$position.y.toFixed(1)}</span>
    <span class="badge variant-filled">Z: {$position.z.toFixed(1)}</span>
  </p>
</div>
