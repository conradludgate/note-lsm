<script lang="ts">
  import type { TransitionConfig } from "svelte/transition";
  import NoteEntryLoader from "./NoteEntryLoader.svelte";
  import Bar from "./Bar.svelte";
  import type { Temporal } from "@js-temporal/polyfill";

  interface Props {
    currentTime: Temporal.ZonedDateTime;
    parentKey: string;
    children: string[];
    openNoteStack: string[];
    open: (stack: string[]) => void;
  }

  let { currentTime, parentKey, children, openNoteStack, open }: Props =
    $props();

  let sortedIds = $derived([...children].sort().reverse());
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

{#if isParentSelected && sortedIds.length > 0}
  <div transition:scale2={{ duration: 150 }} class="notestack">
    <Bar key={parentKey} />
    <div class="list">
      {#each sortedIds as key (key)}
        <NoteEntryLoader
          {currentTime}
          {key}
          openNoteStack={openNoteStack.slice(1)}
          open={(stack) => open([key, ...stack])}
          select={() => {}}
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
