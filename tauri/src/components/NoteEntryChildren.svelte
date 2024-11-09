<script lang="ts">
  import NoteStack from "./NoteStack.svelte";
  import type { TransitionConfig } from "svelte/transition";
  import NoteEntryLoader from "./NoteEntryLoader.svelte";

  interface Props {
    key: string[];
    childrenIds: string[];
    selectStack: string[];
  }

  let { key, childrenIds, selectStack = $bindable() }: Props = $props();

  let isSelected = $derived(arrayEqual(selectStack, key));
  let isParentSelected = $derived(arrayHashPrefix(selectStack, key));

  $inspect({ selectStack, key, isSelected });

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }

  function arrayHashPrefix(a: string[], prefix: string[]): boolean {
    return prefix.length <= a.length && prefix.every((el, ix) => el === a[ix]);
  }

  export function scale2(
    node: Element,
    { delay = 0, duration = 400 } = {}
  ): TransitionConfig {
    // const style = getComputedStyle(node);
    // const height = parseFloat(style["height"]);
    // console.log(height);

    return {
      delay,
      duration,
      easing: (t) => t,
      css: (_t: number, u: number) => {

        const style = getComputedStyle(node);
    const height = parseFloat(style["height"]);
      return `
        height: ${height * (1 - u)}px;
        transform: scale(${1 - u * 0.5}, ${1 - u});
        opacity: ${1 - u}
      `},
    };
  }
</script>

{#if isParentSelected && childrenIds.length > 0}
  <div transition:scale2={{ duration: 150 }} class="notestack">
    <NoteStack {key} bind:selectStack>
      {#each childrenIds as childId}
        <NoteEntryLoader key={[...key, childId]} bind:selectStack />
      {/each}
    </NoteStack>
  </div>
{/if}

<style>
  .notestack {
    margin-left: 12px;
  }
</style>
