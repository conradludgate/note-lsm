<script lang="ts">
  import { keyColor } from "../hash";

  interface Props {
    key: string;
  }

  let { key }: Props = $props();
  let color = $derived(keyColor(key));
</script>

<div class="bar" style:background-color={color}></div>

<style>
  .bar {
    z-index: 1;
    width: 0.5em;
    /* margin-right: 0.375em; */
    border: 0px solid;
    border-bottom-left-radius: 0.25em;
    /* border-bottom-right-radius: 0.25em; */
  }
</style>
