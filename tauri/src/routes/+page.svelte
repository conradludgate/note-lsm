<script lang="ts">
  import NoteStack from "../components/NoteStack.svelte";
  import Inspiration from "../components/Inspiration.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import NoteEntryLoader from "../components/NoteEntryLoader.svelte";

  let load = $state(invoke<string[]>("unprocessed", {}));

  let selectStack = $state(["foo"]);
</script>

<main class="container">
  <div class="notestack">
    <NoteStack key={[]} bind:selectStack>
      {#await invoke<string[]>("unprocessed", {}) then notes}
        {#each notes as note}
          <NoteEntryLoader key={[note]} bind:selectStack />
        {/each}
      {/await}
    </NoteStack>
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
    width: 300px;
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
