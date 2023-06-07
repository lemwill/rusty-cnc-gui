<script lang="ts">
  import Communication from "../lib/Communication.svelte";
  import RepeatButton from "../components/RepeatButton.svelte";
  import {
    jog_x_plus,
    jog_x_minus,
    jog_y_plus,
    jog_y_minus,
    jog_z_plus,
    jog_z_minus,
  } from "../lib/Communication.svelte";
</script>

<Communication />
<br />
<div class="grid grid-cols-3 gap-0">
  <div />
  <div>
    <RepeatButton label="Y+" onRepeat={jog_y_plus} />
  </div>
  <div />
  <div>
    <RepeatButton label="X-" onRepeat={jog_x_minus} />
  </div>
  <div>
    <!-- You could place a stop button or other central control here -->
  </div>
  <div>
    <RepeatButton label="X+" onRepeat={jog_x_plus} />
  </div>
  <div />
  <div>
    <RepeatButton label="Y-" onRepeat={jog_y_minus} />
  </div>
  <div />
</div>
<br />

<div class="grid grid-cols-2 gap-0">
  <div>
    <RepeatButton label="Z+" onRepeat={jog_z_plus} />
  </div>
  <div>
    <RepeatButton label="Z-" onRepeat={jog_z_minus} />
  </div>
</div>
