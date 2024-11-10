<script lang="ts">
  interface Props {
    text: string;
  }

  const { text }: Props = $props();
  const lines = $derived(text.split("\n"));
</script>

{#each lines as line, i}
  <span>{line}</span>{#if i + 1 < lines.length}<br />{/if}
{/each}

<style>
</style>
