<script module>
  let keyHandlerSetup = $state(false);
  let shift = $state(false);

  const shiftHandler = (e: KeyboardEvent) => { shift = e.shiftKey };

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }
</script>

<script lang="ts">
  import type { SvelteSet } from "svelte/reactivity";
  import type { MouseEventHandler } from "svelte/elements";
  import { onMount } from "svelte";
  import DateTime from "./DateTime.svelte";

  onMount(() => {
    $effect(() => {
      if (!keyHandlerSetup) {
        keyHandlerSetup = true;
        window.addEventListener("keydown", shiftHandler,false);
        window.addEventListener("keyup",  shiftHandler ,false);
      }
    })
  })

  interface Props {
    key: string[];
    datetime: string;
    text: string;
    openNoteStack: string[];
    selectedNotes: SvelteSet<string>;
  }

  let { key, datetime, text, openNoteStack = $bindable(), selectedNotes = $bindable() }: Props = $props();

  let opened = $derived(arrayEqual(openNoteStack, key));
  let selected = $derived(key.length === 1 && selectedNotes.has(key[0]));

  let select: MouseEventHandler<HTMLDivElement> = $derived(e => {
    if (e.shiftKey) {
      if (key.length !== 1) return;
      let k = key[0];

      if (selectedNotes.has(k)) {
        selectedNotes.delete(k)
      } else {
        selectedNotes.add(k)
      }
    } else {
      openNoteStack = key;
    }
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="inner"
  class:opened={opened}
  class:selected={selected}
  class:shift={shift}
  data-depth={key.length}
  onclick={select}
>
  <p class="text">{text}</p>
  <div class="date"><DateTime {datetime}/></div>
</div>

<style lang="scss">
  .inner {
    height: 3.5em;
    transition: 150ms;
    padding: 0.8em 1.2em 0.8em 0.7em;
    position: relative;
    /* margin-left: -0.825em; */
    /* border-bottom-left-radius: 0.25em; */

    -webkit-user-select: none; /* Safari */
    user-select: none; /* Standard syntax */
    cursor: pointer;

    background-color: #f6f6f6;
    border-block: solid 0.2em #f6f6f6;

    > .text {
      height: 2.4em;
      font-size: 1em;
      line-height: 1.2em;
      margin-top: 0;
      text-align: left;
      overflow: clip;
      text-overflow: ellipsis;
      text-wrap-style: pretty;
    }

    > .date {
      font-size: 1em;
      position: absolute;
      right: 0.4em;
      bottom: 0.4em;
    }

    // :hover is for regular click, which toggles .opened
    // :hover.shift is for shift-click, which toggles .selected

    &:hover {
      background-color: rgb(227, 227, 227);
      border-block: solid 0.2em rgb(194, 194, 194);
    }

    // for depth != 1, disable the cursor for shift-click.
    &:hover.shift {
      background-color: #f6f6f6;
      border-block: solid 0.2em #f6f6f6;
      cursor:not-allowed;
    }

    // for depth = 1, we re-enable the cursor for shift-click.
    &:hover.shift[data-depth="1"] {
      background-color: rgb(95, 199, 140);
      /* box-shadow: inset 0.2em 0em 0.2em 0.2em rgb(194, 194, 194); */
      border-block: solid 0.2em rgb(194, 194, 194);
      cursor: pointer;
    }

    &.opened {
      background-color: rgb(227, 227, 227);
      border-block: solid 0.2em rgb(227, 227, 227);

      &:hover {
        background-color: rgb(238, 238, 238);
      }

      &:hover.shift[data-depth="1"] {
        background-color: rgb(227, 227, 227);
        border-block: solid 0.2em rgb(227, 227, 227);
      }
    }

    &.selected {
      background-color: rgb(95, 199, 140);
      border-block: solid 0.2em rgb(227, 227, 227);

      &:hover {
        background-color: rgb(227, 227, 227);
        border-block: solid 0.2em rgb(194, 194, 194);
      }

      &:hover.shift[data-depth="1"] {
        background-color: rgb(150, 189, 167);
        /* box-shadow: inset 0.2em 0em 0.2em 0.2em rgb(194, 194, 194); */
        border-block: solid 0.2em rgb(194, 194, 194);
        cursor: pointer;
      }
    }
  }
</style>
