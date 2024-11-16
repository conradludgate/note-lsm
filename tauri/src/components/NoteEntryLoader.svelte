<script lang="ts">
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import { getNote } from "../native";
  import type { Temporal } from "@js-temporal/polyfill";

  interface Props {
    currentTime: Temporal.ZonedDateTime;
    key: string;
    openNoteStack: string[];
    open: (stack: string[]) => void;
    selected?: boolean;
    select: () => void;
  }

  let { currentTime, key, openNoteStack, open, selected, select }: Props =
    $props();
  let opened = $derived(openNoteStack.length === 1 && openNoteStack[0] === key);
</script>

{#snippet entry(
  text: string,
  datetime: Temporal.ZonedDateTime | undefined,
  children: string[]
)}
  <NoteEntryInner
    {currentTime}
    {datetime}
    {text}
    {opened}
    open={() => open([])}
    {selected}
    {select}
  />
  <NoteEntryChildren
    {currentTime}
    parentKey={key}
    {children}
    {openNoteStack}
    {open}
  />
{/snippet}

<div class="entry">
  {#await getNote(key)}
    {@render entry("", undefined, [])}
  {:then note}
    {@render entry(note.note, note.datetime, note.children)}
  {/await}
</div>

<style>
  .entry {
    height: max-content;
  }
</style>
