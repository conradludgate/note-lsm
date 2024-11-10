<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import NoteEntryInner from "./NoteEntryInner.svelte";
  import NoteEntryChildren from "./NoteEntryChildren.svelte";

  interface Props {
    key: string[];
    selectStack: string[];
  }

  interface Note {
    note: string;
    datetime: string;
    children: string[];
  }

  let { key, selectStack = $bindable() }: Props = $props();
</script>

<div class="entry">
  {#await invoke < Note > ("get_note", { id: key[key.length - 1] })}
    <NoteEntryInner {key} datetime={""} text={""} bind:selectStack />
    <NoteEntryChildren {key} childrenIds={[]} bind:selectStack />
  {:then note}
    <NoteEntryInner
      {key}
      datetime={note.datetime}
      text={note.note}
      bind:selectStack
    />
    <NoteEntryChildren {key} childrenIds={note.children} bind:selectStack />
  {/await}
</div>

<style>
  .entry {
    height: max-content;
  }
</style>
