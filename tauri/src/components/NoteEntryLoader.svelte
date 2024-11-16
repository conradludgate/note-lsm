<script lang="ts">
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import { getNote } from "../native";

  interface Props {
    key: string;
    openNoteStack: string[];
    openNote: (stack: string[]) => void;
    selected?: boolean;
    select: () => void;
  }

  let { key, openNoteStack, openNote, selected, select }: Props = $props();
</script>

{#snippet entry(text: string, datetime: string, children: string[])}
  <NoteEntryInner
    {key}
    {datetime}
    {text}
    {openNoteStack}
    {openNote}
    {selected}
    {select}
  />
  <NoteEntryChildren
    parentKey={key}
    childrenIds={children}
    {openNoteStack}
    {openNote}
  />
{/snippet}

<div class="entry">
  {#await getNote(key)}
    {@render entry("", "", [])}
  {:then note}
    {@render entry(note.note, note.datetime, note.children)}
  {/await}
</div>

<style>
  .entry {
    height: max-content;
  }
</style>
