<script lang="ts">
  import NoteStack from "../components/NoteStack.svelte";
  import NoteEntry from "../components/NoteEntry.svelte";
  import Inspiration from "../components/Inspiration.svelte";

  let selectStack = $state(["foo"]);
</script>

<main class="container">
  <div class="notestack">
    <NoteStack key={[]} {selectStack}>
      <NoteEntry
        key={["foo"]}
        datetime="yesterday"
        text="foo"
        bind:selectStack
      />
      <NoteEntry key={["bar"]} datetime="today" text="bar" bind:selectStack />
      <NoteEntry
        key={["baz"]}
        datetime="tomorrow"
        text="baz"
        bind:selectStack
      />
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
