<script lang="ts">
  import type { Blockquote, Break, RootContent, Code, Definition, Delete, Emphasis, FootnoteDefinition, FootnoteReference, Heading, Html, Image, ImageReference, InlineCode, Link, LinkReference, List, ListItem, Paragraph, Strong, Table, TableCell, TableRow, ThematicBreak, Yaml } from "mdast";
  import { fromMarkdown } from "mdast-util-from-markdown";
  import RenderContent from "./RenderContent.svelte";

  interface Props {
    text: string
  }

  const { text }: Props = $props();
  const lines = $derived(text.split("\n"));
  $inspect({text, lines});
</script>

{#each lines as line, i}
  <span>{line}</span>{#if i + 1 < lines.length}<br/>{/if}
{/each}

<style>
</style>
