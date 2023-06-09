<script lang="ts">
  import RepeatButton from "../components/RepeatButton.svelte";
  import { jog, JogMessage } from "../lib/socketStore.js";
  import PositionView from "./PositionView.svelte";

  export const jog_x_plus = () => jog(JogMessage.Axis.X, 1);
  export const jog_x_minus = () => jog(JogMessage.Axis.X, -1);
  export const jog_y_plus = () => jog(JogMessage.Axis.Y, 1);
  export const jog_y_minus = () => jog(JogMessage.Axis.Y, -1);
  export const jog_z_plus = () => jog(JogMessage.Axis.Z, 1);
  export const jog_z_minus = () => jog(JogMessage.Axis.Z, -1);
</script>

<PositionView />
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
