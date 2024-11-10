<script lang="ts">
  import Inspiration from "../components/Inspiration.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import NoteEntryLoader from "../components/NoteEntryLoader.svelte";
  import Bar from "../components/Bar.svelte";

  let selectStack = $state([]);
</script>

<main class="container">
  <div class="notestack">
    <Bar key={[]} />
    <div class="list">
      {#await invoke<string[]>("unprocessed", {}) then notes}
        {#each notes as note}
          <NoteEntryLoader key={[note]} bind:selectStack />
        {/each}
      {/await}
    </div>
  </div>
  <div class="notetext">
    <Inspiration />
  </div>
</main>

<style>
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
    margin: 0;
    padding-top: 1vh;
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

  .notetext {
    width: 100%;
    padding: 25px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
