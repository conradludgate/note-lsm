<script module>
  let now = $state(Temporal.Now.zonedDateTimeISO().toString());
  const id = setInterval(
    () => { now = Temporal.Now.zonedDateTimeISO().toString(); },
    1000
  );

  let keyHandlerSetup = $state(false);
  let shift = $state(false);

  const shiftHandler = (e: KeyboardEvent) => { shift = e.shiftKey };

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }
</script>

<script lang="ts">
  import { Temporal } from "@js-temporal/polyfill";
  import type { SvelteSet } from "svelte/reactivity";
  import { formatTime } from "../time";
  import type { MouseEventHandler } from "svelte/elements";
  import { onMount } from "svelte";

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

  let date = $derived.by(() => {
    if (datetime === "") return "";

    let now2 = Temporal.ZonedDateTime.from(now);
    let dt = Temporal.ZonedDateTime.from(datetime);

    return formatTime(dt, now2);
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
  <span class="date">{date}</span>
</div>

<style>
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
  }

  .inner:hover.shift {
    background-color: #f6f6f6;
    border-block: solid 0.2em #f6f6f6;
    cursor:not-allowed;
  }

  .inner.opened {
    background-color: rgb(227, 227, 227);
    border-block: solid 0.2em rgb(227, 227, 227);
  }

  .inner:hover.opened.shift {
    background-color: rgb(227, 227, 227);
    border-block: solid 0.2em rgb(227, 227, 227);
  }

  .inner:hover {
    background-color: rgb(227, 227, 227);
    border-block: solid 0.2em rgb(194, 194, 194);
  }

  .inner:hover.shift[data-depth="1"] {
    background-color: rgb(95, 199, 140);
    /* box-shadow: inset 0.2em 0em 0.2em 0.2em rgb(194, 194, 194); */
    border-block: solid 0.2em rgb(194, 194, 194);
    cursor: pointer;
  }

.inner:hover.selected.shift[data-depth="1"] {
  background-color: rgb(150, 189, 167);
  /* box-shadow: inset 0.2em 0em 0.2em 0.2em rgb(194, 194, 194); */
  border-block: solid 0.2em rgb(194, 194, 194);
  cursor: pointer;
}

  .inner.selected {
    background-color: rgb(95, 199, 140);
    border-block: solid 0.2em rgb(227, 227, 227);
  }

  .inner.opened:hover {
    background-color: rgb(227, 227, 227);
  }

  .inner > .text {
    height: 2.4em;
    font-size: 1em;
    line-height: 1.2em;
    margin-top: 0;
    text-align: left;
    overflow: clip;
    text-overflow: ellipsis;
    text-wrap-style: pretty;
  }

  .inner > .date {
    font-size: 1em;
    position: absolute;
    right: 0.4em;
    bottom: 0.4em;
  }
</style>
