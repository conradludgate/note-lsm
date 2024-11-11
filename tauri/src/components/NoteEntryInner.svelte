<script module>
  let now = $state(Temporal.Now.zonedDateTimeISO().toString());
  const id = setInterval(
    () => { now = Temporal.Now.zonedDateTimeISO().toString(); },
    1000
  );
</script>

<script lang="ts">
  import { Temporal } from "@js-temporal/polyfill";
  import type { SvelteSet } from "svelte/reactivity";
  import { formatTime } from "../time";

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

  let date = $derived.by(() => {
    if (datetime === "") {
      return "";
    }
    let now2 = Temporal.ZonedDateTime.from(now);
    let dt = Temporal.ZonedDateTime.from(datetime);

    return formatTime(dt, now2);
  });

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="inner"
  class:opened={opened}
  class:selected={selected}
  onclick={(e) => {
    if (e.shiftKey) {
      if (key.length !== 1) {
        return;
      }
      let k = key[0];

      if (selectedNotes.has(k)) {
        selectedNotes.delete(k)
      } else {
        selectedNotes.add(k)
      }
    } else {
      openNoteStack = key;
    }
  }}
>
  <p class="text">{text}</p>
  <span class="date">{date}</span>
</div>

<style>
  .inner {
    height: 3.5em;
    transition: 150ms;
    padding-top: 15px;
    padding-bottom: 15px;
    padding-left: 15px;
    padding-right: 15px;
    position: relative;
    margin-left: -8px;
    border-bottom-left-radius: 4px;

    -webkit-user-select: none; /* Safari */
    user-select: none; /* Standard syntax */
    cursor: pointer;
  }

  .inner.opened {
    background-color: rgb(227, 227, 227);
  }

  .inner:hover {
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
