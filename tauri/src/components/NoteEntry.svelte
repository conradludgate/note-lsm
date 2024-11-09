<script lang="ts">
  import NoteStack from "./NoteStack.svelte";
  import NoteEntry from "./NoteEntry.svelte";
  import type { TransitionConfig } from "svelte/transition";

  interface Props {
    key: string[];
    datetime: string;
    text: string;
    selectStack: string[];
  }

  let { key, datetime, text, selectStack = $bindable() }: Props = $props();

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
    const style = getComputedStyle(node);
    const height = parseFloat(style["height"]);

    return {
      delay,
      duration,
      easing: (t) => t,
      css: (_t: number, u: number) => `
        height: ${height * (1 - u)}px;
        transform: scale(${1 - u * 0.5}, ${1 - u});
        opacity: ${1 - u}
      `,
    };
  }
</script>

<div class="entry" class:selected={isSelected}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="inner"
    onclick={() => {
      selectStack = key;
    }}
  >
    <span>{datetime}</span><br />
    <span>{text}</span>
  </div>
  {#if isParentSelected && arrayEqual(key, ["foo"])}
    <div transition:scale2={{ duration: 150 }} class="notestack">
      <NoteStack {key} {selectStack}>
        <NoteEntry
          key={["foo", "foo2"]}
          datetime="yesterday"
          text="foo/foo2"
          bind:selectStack
        />
      </NoteStack>
    </div>
  {/if}
</div>

<style>
  .entry {
    height: max-content;
  }

  .inner {
    height: 50px;
    transition: 150ms;
    padding-top: 10px;
    padding-bottom: 10px;
  }

  .inner > span {
    font-size: 12pt;
  }

  .entry.selected > .inner {
    background-color: rgb(227, 227, 227);
  }

  .inner:hover {
    background-color: rgb(227, 227, 227);
  }

  .notestack {
    margin-left: 12px;
  }
</style>
