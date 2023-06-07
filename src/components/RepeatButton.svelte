<script>
  export let label = "";
  export let onRepeat;

  let intervalId;

  function handleMousedown() {
    if (onRepeat) {
      onRepeat();
      intervalId = setInterval(onRepeat, 30);
    }
  }

  function handleMouseup() {
    clearInterval(intervalId);
  }
</script>

<button
  class="btn variant-filled tight-button"
  on:mousedown={handleMousedown}
  on:mouseup={handleMouseup}
  on:mouseleave={handleMouseup}
>
  {label}
</button>

<style>
  .tight-button {
    /* These are just example values; adjust as needed for your design */
    padding: 10px 10px;
    font-size: 1.6rem;
    font-weight: 150;
  }
</style>
