<script lang="ts">
  import {Hsluv} from "hsluv";
  import { keyHash } from "../hash";

  interface Props {
    children?: any;
		key: string[];
    selectStack: string[];
	}

  let { key, children, selectStack, ...props }: Props = $props();

  let isSelected = $derived(arrayHashPrefix(selectStack, key));

  let color = $derived.by(() => {
    let hash = keyHash(key);

    let conv = new Hsluv();
    conv.hsluv_h = (hash / 1000) % 360.0;
    conv.hsluv_s = 50;
    conv.hsluv_l = 50;
    conv.hsluvToHex()
    return conv.hex;
  });


  function arrayHashPrefix(a: string[], prefix: string[]): boolean {
    return prefix.length <= a.length && prefix.every((el, ix) => el === a[ix]);
  }
</script>

<div class="notestack" class:selected={isSelected}>
  <div class="bar" style:background-color={color}></div>
  <div class="list">
    {@render children?.()}
  </div>
</div>

<style>
  .notestack {
    /* width: 300px; */
    width: 100%;
    display: flex;
    height: max-content;
  }

  .bar {
    z-index: 1;
    width: 8px;
    margin: 4px;
    background-color: black;
    border: 0px solid;
    border-radius: 4px;
  }

  .list {
    z-index: 0;
    /* width: 280px; */
    width: 100%;
    display: flex;
    flex-direction: column;
    margin-left: -16px;
  }
</style>
