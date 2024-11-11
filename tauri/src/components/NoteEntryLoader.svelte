<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";
  import type { SvelteSet } from "svelte/reactivity";

  interface Props {
    key: string[];
    openNoteStack: string[];
    selectedNotes: SvelteSet<string>;
  }

  interface Note {
    note: string;
    datetime: string;
    children: string[];
  }

  let {
    key,
    openNoteStack = $bindable(),
    selectedNotes = $bindable(),
  }: Props = $props();
</script>

<div class="entry">
  {#await invoke < Note > ("get_note", { id: key[key.length - 1] })}
    <NoteEntryInner
      {key}
      datetime={""}
      text={""}
      bind:openNoteStack
      bind:selectedNotes
    />
    <NoteEntryChildren
      {key}
      childrenIds={[]}
      bind:openNoteStack
      bind:selectedNotes
    />
  {:then note}
    <NoteEntryInner
      {key}
      datetime={note.datetime}
      text={note.note}
      bind:openNoteStack
      bind:selectedNotes
    />
    <NoteEntryChildren
      {key}
      childrenIds={note.children}
      bind:openNoteStack
      bind:selectedNotes
    />
  {/await}
</div>

<style>
  .entry {
    height: max-content;
  }
</style>
