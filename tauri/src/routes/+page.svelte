<script lang="ts">
  import Inspiration from "../components/Inspiration.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import NoteEntryLoader from "../components/NoteEntryLoader.svelte";
  import Bar from "../components/Bar.svelte";
  import Render from "../components/md/Render.svelte";

  interface Note {
    note: string;
    datetime: string;
    children: string[];
  }

  let draft = $state(`1) Hello, _Jupiter_ and *Neptune* and **Pluto**[0]!
2) the world is full of [cats](https://http.cat/404)

[0]: https://http.cat/418

---

hello

# hello 1

## hello 2

### hello 3

#### hello 4

##### hello 5

###### hello 6

\`\`\`rust
lol
\`\`\`
`);

  let openNoteStack = $state<string[]>([]);
  let selectedNotes = $state<string[]>([]);
</script>

<main class="container">
  <div class="notestack">
    <Bar key={[]} />
    <div class="list">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="edit" onclick={() => { openNoteStack = [":draft:"]; }}>
        <svg xmlns="http://www.w3.org/2000/svg" width="1.13em" height="1em" viewBox="0 0 576 512"><path fill="currentColor" d="m402.3 344.9l32-32c5-5 13.7-1.5 13.7 5.7V464c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V112c0-26.5 21.5-48 48-48h273.5c7.1 0 10.7 8.6 5.7 13.7l-32 32c-1.5 1.5-3.5 2.3-5.7 2.3H48v352h352V350.5c0-2.1.8-4.1 2.3-5.6m156.6-201.8L296.3 405.7l-90.4 10c-26.2 2.9-48.5-19.2-45.6-45.6l10-90.4L432.9 17.1c22.9-22.9 59.9-22.9 82.7 0l43.2 43.2c22.9 22.9 22.9 60 .1 82.8M460.1 174L402 115.9L216.2 301.8l-7.3 65.3l65.3-7.3zm64.8-79.7l-43.2-43.2c-4.1-4.1-10.8-4.1-14.8 0L436 82l58.1 58.1l30.9-30.9c4-4.2 4-10.8-.1-14.9"/></svg>
      </div>
      {#await invoke<string[]>("unprocessed", {}) then notes}
        {#each notes as note}
          <NoteEntryLoader key={[note]} bind:openNoteStack bind:selectedNotes/>
        {/each}
      {/await}
    </div>
  </div>
  <div class="separator"></div>
  <div class="notetext">
    {#if openNoteStack.length === 0}
      <Inspiration />
    {:else if openNoteStack.length === 1 && openNoteStack[0] === ":draft:"}
      <Render bind:text={draft} readOnly={false}/>
    {:else}
      {#await invoke<Note>("get_note", {id: openNoteStack[openNoteStack.length - 1]}) then note}
        <Render text={note.note} readOnly/>
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
    height: 1em;
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
    width: 100%;
    padding: 25px;

    text-align: left;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
