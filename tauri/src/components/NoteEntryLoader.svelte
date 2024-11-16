<script lang="ts">
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import { getNote } from "../native";

  interface Props {
    key: string;
    noteDepth: number;
    openNoteStack: string[];
    openNote: (stack: string[]) => void;
    selectedNotes: string[];
  }

  let {
    key,
    noteDepth,
    openNoteStack,
    openNote,
    selectedNotes = $bindable(),
  }: Props = $props();
</script>

{#snippet entry(text: string, datetime: string, children: string[])}
  <NoteEntryInner
    {key}
    {datetime}
    {text}
    {noteDepth}
    {openNoteStack}
    {openNote}
    bind:selectedNotes
  />
  <NoteEntryChildren
    parentKey={key}
    childrenIds={children}
    {noteDepth}
    {openNoteStack}
    {openNote}
    bind:selectedNotes
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
