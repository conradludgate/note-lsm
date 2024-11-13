<script lang="ts">
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import { getNote } from "../native";

  interface Props {
    key: string[];
    openNoteStack: string[];
    selectedNotes: string[];
  }

  let {
    key,
    openNoteStack = $bindable(),
    selectedNotes = $bindable(),
  }: Props = $props();
</script>

{#snippet entry(text: string, datetime: string, children: string[])}
  <NoteEntryInner
    {key}
    datetime={datetime}
    text={text}
    bind:openNoteStack
    bind:selectedNotes
  />
  <NoteEntryChildren
    {key}
    childrenIds={children}
    bind:openNoteStack
    bind:selectedNotes
  />
{/snippet}

<div class="entry">
  {#await getNote(key[key.length - 1])}
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
