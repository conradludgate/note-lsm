<script module>
  let keyHandlerSetup = $state(false);
  let shift = $state(false);

  const shiftHandler = (e: KeyboardEvent) => {
    shift = e.shiftKey;
  };
</script>

<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";
  import { onMount } from "svelte";
  import DateTime from "./DateTime.svelte";

  onMount(() => {
    $effect(() => {
      if (!keyHandlerSetup) {
        keyHandlerSetup = true;
        window.addEventListener("keydown", shiftHandler, false);
        window.addEventListener("keyup", shiftHandler, false);
      }
    });
  });

  interface Props {
    datetime: string;
    text: string;
    opened: boolean;
    selected?: boolean;

    open: (stack: string[]) => void;
    select: () => void;
  }

  let { datetime, text, opened, open, selected, select }: Props = $props();

  let onclick: MouseEventHandler<HTMLDivElement> = $derived((e) => {
    if (e.shiftKey) {
      select();
    } else {
      open([]);
    }
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="inner"
  class:opened
  class:selected
  class:shift
  data-selectable={selected !== undefined}
  {onclick}
>
  <p class="text">{text}</p>
  <div class="date"><DateTime {datetime} /></div>
  <!-- <div class="selected-circle"></div> -->
  {#if selected}
    <i class="selected-circle-icon fa-regular fa-circle-check"></i>
  {:else}
    <i class="selected-circle-icon fa-regular fa-circle"></i>
  {/if}
</div>

<style lang="scss">
  .inner {
    height: 3.5em;
    transition: 150ms;
    padding: 0.8em 2.4em 0.8em 0.7em;
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

    > .selected-circle-icon {
      display: none;
      position: absolute;
      right: 0.4em;
      top: 0.4em;
      font-size: 1.5em;
    }

    // :hover is for regular click, which toggles .opened
    // :hover.shift is for shift-click, which toggles .selected

    &.shift {
      &[data-selectable="true"] > .selected-circle-icon {
        display: inline;
      }
    }

    &:hover {
      background-color: rgb(227, 227, 227);
      border-block: solid 0.2em rgb(194, 194, 194);
    }

    // for non selectable, disable the cursor for shift-click.
    &:hover.shift {
      background-color: #f6f6f6;
      border-block: solid 0.2em #f6f6f6;
      cursor: not-allowed;
    }

    // for selectable, we re-enable the cursor for shift-click.
    &:hover.shift[data-selectable="true"] {
      cursor: pointer;
    }

    &.opened {
      background-color: rgb(227, 227, 227);
      border-block: solid 0.2em rgb(227, 227, 227);

      &:hover {
        background-color: rgb(238, 238, 238);
      }

      &:hover.shift[data-selectable="true"] {
        background-color: rgb(227, 227, 227);
        border-block: solid 0.2em rgb(227, 227, 227);
      }
    }

    &.selected {
      .selected-circle-icon {
        display: inline;
      }
    }
  }
</style>
