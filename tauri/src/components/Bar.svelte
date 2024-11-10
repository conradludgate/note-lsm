<script lang="ts">
  import { keyColor } from "../hash";

  interface Props {
    key: string[];
  }

  let { key }: Props = $props();
  let color = $derived(keyColor(key));
</script>

<div class="bar" style:background-color={color}></div>

<style>
  .bar {
    z-index: 1;
    width: 8px;
    margin: 0px 2px 0px 4px;
    border: 0px solid;
    border-bottom-left-radius: 4px;
    border-bottom-right-radius: 4px;
  }
</style>
