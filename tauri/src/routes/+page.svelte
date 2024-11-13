<script lang="ts">
  import Inspiration from "../components/Inspiration.svelte";
  import NoteEntryLoader from "../components/NoteEntryLoader.svelte";
  import Bar from "../components/Bar.svelte";
  import Render from "../components/md/Render.svelte";
  import { getNote, unprocessed } from "../native";

  let draft = $state("");
  let openNoteStack = $state<string[]>([]);
  let selectedNotes = $state<string[]>([]);

  $effect(() => {
    Promise.all(selectedNotes.map((id) => getNote(id))).then((notes) => {
      draft = notes.map((note) => note.note).join("\n\n---\n\n");
    });
  });
</script>

<main class="container">
  <div class="notestack">
    <Bar key={[]} />
    <div class="list">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="edit"
        onclick={() => {
          openNoteStack = [":draft:"];
        }}
      >
        <i class="fa-regular fa-edit"></i>
        <span>New Note</span>
      </div>
      {#await unprocessed() then notes}
        {#each notes as note}
          <NoteEntryLoader key={[note]} bind:openNoteStack bind:selectedNotes />
        {/each}
      {/await}
    </div>
  </div>
  <div class="separator"></div>
  <div class="notetext">
    {#if openNoteStack.length === 0}
      <Inspiration />
    {:else if openNoteStack.length === 1 && openNoteStack[0] === ":draft:"}
      <Render bind:text={draft} readOnly={false} />
    {:else}
      {#await getNote(openNoteStack[openNoteStack.length - 1]) then note}
        <Render text={note.note} readOnly />
      {/await}
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    display: flex;
    text-align: center;
  }

  .notestack {
    flex: 0 0 auto;
    width: 350px;
    display: flex;
    height: max-content;
  }

  .notestack > .list {
    width: 100%;
  }

  .notestack > .list > .edit {
    flex: 0 0 auto;
    height: 2em;
    line-height: 2em;
    padding: 0.5em 0;
    margin-left: -8px;
    background-color: #e7e7e7;
  }

  .separator {
    width: 0px;
    box-shadow: 2px 0px 2px 2px lightgray;
    height: 100vh;
  }

  .notetext {
    min-width: 0;
    width: 100%;
    padding-left: 4px;

    text-align: left;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
