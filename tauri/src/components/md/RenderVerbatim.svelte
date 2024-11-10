<script lang="ts">
  import type {  RootContent } from "mdast";
  import RenderText from "./RenderText.svelte";

  interface Props {
    data: string;
    node: RootContent;
  }

  const { data, node: n }: Props = $props();
</script>

<RenderText text={data.slice(n.position!.start.offset!, n.position!.end.offset!)}/>

<style>
</style>
