<script lang="ts">
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import { getNote } from "../native";

  interface Props {
    key: string;
    openNoteStack: string[];
    open: (stack: string[]) => void;
    selected?: boolean;
    select: () => void;
  }

  let { key, openNoteStack, open, selected, select }: Props = $props();
  let opened = $derived(openNoteStack.length === 1 && openNoteStack[0] === key);
</script>

{#snippet entry(text: string, datetime: string, children: string[])}
  <NoteEntryInner {datetime} {text} {opened} {open} {selected} {select} />
  <NoteEntryChildren parentKey={key} {children} {openNoteStack} {open} />
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
