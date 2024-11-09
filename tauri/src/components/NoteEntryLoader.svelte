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

  let isSelected = $derived(arrayEqual(selectStack, key));
  let isParentSelected = $derived(arrayHashPrefix(selectStack, key));

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }

  function arrayHashPrefix(a: string[], prefix: string[]): boolean {
    return prefix.length <= a.length && prefix.every((el, ix) => el === a[ix]);
  }
</script>

<div class="entry" class:selected={isSelected}>
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
