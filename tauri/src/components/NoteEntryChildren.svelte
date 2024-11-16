<script lang="ts">
  import type { TransitionConfig } from "svelte/transition";
  import NoteEntryLoader from "./NoteEntryLoader.svelte";
  import Bar from "./Bar.svelte";

  interface Props {
    parentKey: string;
    childrenIds: string[];
    noteDepth: number;
    openNoteStack: string[];
    openNote: (stack: string[]) => void;
    select: (key: string, toggle: boolean) => void;
  }

  let {
    parentKey,
    childrenIds,
    noteDepth,
    openNoteStack,
    openNote,
    select,
  }: Props = $props();

  let isParentSelected = $derived(openNoteStack[0] === parentKey);

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
    <Bar key={parentKey} />
    <div class="list">
      {#each childrenIds as key (key)}
        <NoteEntryLoader
          {key}
          noteDepth={noteDepth + 1}
          openNoteStack={openNoteStack.slice(1)}
          openNote={(stack) => openNote([parentKey, ...stack])}
          {select}
        />
      {/each}
    </div>
  </div>
{/if}

<style>
  .notestack {
    width: calc(100% - 0.375em);
    display: flex;
    height: max-content;
    margin-left: 0.375em;
  }

  .list {
    width: 100%;
    display: flex;
    flex-direction: column;
    margin-left: 0px;
  }
</style>
