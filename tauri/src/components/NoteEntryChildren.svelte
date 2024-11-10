<script lang="ts">
  import type { TransitionConfig } from "svelte/transition";
  import NoteEntryLoader from "./NoteEntryLoader.svelte";
  import Bar from "./Bar.svelte";

  interface Props {
    key: string[];
    childrenIds: string[];
    openNoteStack: string[];
    selectedNotes: string[];
  }

  let {
    key,
    childrenIds,
    openNoteStack = $bindable(),
    selectedNotes = $bindable(),
  }: Props = $props();

  let isParentSelected = $derived(arrayHashPrefix(openNoteStack, key));

  function arrayHashPrefix(a: string[], prefix: string[]): boolean {
    return prefix.length <= a.length && prefix.every((el, ix) => el === a[ix]);
  }

  export function scale2(
    node: Element,
    { delay = 0, duration = 400 } = {}
  ): TransitionConfig {
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
        `;
      },
    };
  }
</script>

{#if isParentSelected && childrenIds.length > 0}
  <div transition:scale2={{ duration: 150 }} class="notestack">
    <Bar {key} />
    <div class="list">
      {#each childrenIds as childId}
        <NoteEntryLoader
          key={[...key, childId]}
          bind:openNoteStack
          bind:selectedNotes
        />
      {/each}
    </div>
  </div>
{/if}

<style>
  .notestack {
    width: 100%;
    display: flex;
    height: max-content;
  }

  .list {
    width: 100%;
    display: flex;
    flex-direction: column;
    margin-left: 0px;
  }
</style>
