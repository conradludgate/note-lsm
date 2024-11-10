<script lang="ts">
  import type { Parent } from "mdast";
  import RenderContent from "./RenderContent.svelte";
  import RenderText from "./RenderText.svelte";

  interface Props {
    data: string,
    node: Parent;
  }

  const { data, node: n }: Props = $props();
</script>

{#if n.children.length > 0}
  <RenderText
    text={data.slice(n.position!.start.offset!, n.children[0].position!.start.offset!)}
  />{#each n.children as node, i}
    <RenderContent {data} {node} /><RenderText
      text={
        data.slice(
            n.children[i].position!.end.offset!,
            i+1 < n.children.length
              ? n.children[i+1].position!.start.offset!
              : n.position!.end.offset!)
      }
    />
  {/each}
{:else}
  <RenderText
    text={data.slice(n.position!.start.offset!, n.position!.end.offset!)}
  />
{/if}


<style>
</style>
