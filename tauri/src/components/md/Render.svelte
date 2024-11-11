<script lang="ts">
  import type { Action } from "svelte/action";
  import * as monaco from "monaco-editor";

  interface Props {
    text: string;
    readOnly: boolean;
  }

  let { text = $bindable(), readOnly = false }: Props = $props();

  const check: Action = $derived((renderNode) => {
    let editor = monaco.editor.create(renderNode, {
      value: text,
      language: "markdown",
      minimap: {
        enabled: false,
      },
      fontSize: 16,
      readOnly,
      lineNumbers: "off",
      scrollbar: {
        vertical: "hidden",
        horizontal: "hidden",
      },
    });

    editor.onDidChangeModelContent((_e) => {
      text = editor.getValue();
    });
  });
</script>

<div use:check></div>

<style>
  div {
    height: 100%;
  }
</style>
