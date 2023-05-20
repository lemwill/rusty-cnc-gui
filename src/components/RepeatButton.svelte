<script>
  export let label = "";
  export let onRepeat;

  let intervalId;

  function handleMousedown() {
    if (onRepeat) {
      onRepeat();
      intervalId = setInterval(onRepeat, 100);
    }
  }

  function handleMouseup() {
    clearInterval(intervalId);
  }
</script>

<button
  class="btn variant-filled"
  on:mousedown={handleMousedown}
  on:mouseup={handleMouseup}
  on:mouseleave={handleMouseup}
>
  {label}
</button>
