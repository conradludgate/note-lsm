<script module>
  import * as monaco from "monaco-editor";

  import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
  import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
  import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker'
  import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker'
  import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'

  self.MonacoEnvironment = {
    getWorker(_, label) {
      if (label === 'json') {
        return new jsonWorker()
      }
      if (label === 'css' || label === 'scss' || label === 'less') {
        return new cssWorker()
      }
      if (label === 'html' || label === 'handlebars' || label === 'razor') {
        return new htmlWorker()
      }
      if (label === 'typescript' || label === 'javascript') {
        return new tsWorker()
      }
      if (label === 'typescript' || label === 'javascript') {
        return new tsWorker()
      }
      return new editorWorker()
    }
  }
</script>

<script lang="ts">
  import type { Action } from "svelte/action";

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
      cursorBlinking: readOnly ? "solid": undefined,
      cursorStyle: readOnly ? "line-thin": undefined,
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
