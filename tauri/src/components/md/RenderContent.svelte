<script lang="ts">
  import type { RootContent } from "mdast";
  import { fromMarkdown } from "mdast-util-from-markdown";
  import RenderContent from "./RenderContent.svelte";
  import RenderChildren from "./RenderChildren.svelte";
  import RenderVerbatim from "./RenderVerbatim.svelte";

  interface Props {
    data: string;
    node: RootContent;
  }

  const { data, node: n }: Props = $props();

  // blockquote: Blockquote;
  // break: Break;
  // code: Code;
  // definition: Definition;
  // delete: Delete;
  // emphasis: Emphasis;
  // footnoteDefinition: FootnoteDefinition;
  // footnoteReference: FootnoteReference;
  // heading: Heading;
  // html: Html;
  // image: Image;
  // imageReference: ImageReference;
  // inlineCode: InlineCode;
  // link: Link;
  // linkReference: LinkReference;
  // list: List;
  // listItem: ListItem;
  // paragraph: Paragraph;
  // strong: Strong;
  // table: Table;
  // tableCell: TableCell;
  // tableRow: TableRow;
  // text: Text;
  // thematicBreak: ThematicBreak;
  // yaml: Yaml;
</script>

{#if n.type === "list" && n.ordered}
  <ol><RenderChildren {data} node={n} /></ol>
{:else if n.type === "list" && !n.ordered}
  <ul><RenderChildren {data} node={n} /></ul>
{:else if n.type === "listItem"}
  <li><RenderChildren {data} node={n} /></li>
{:else if n.type === "heading" && n.depth === 1}
  <h1><RenderChildren {data} node={n} /></h1>
{:else if n.type === "heading" && n.depth === 2}
  <h2><RenderChildren {data} node={n} /></h2>
{:else if n.type === "heading" && n.depth === 3}
  <h3><RenderChildren {data} node={n} /></h3>
{:else if n.type === "heading" && n.depth === 4}
  <h4><RenderChildren {data} node={n} /></h4>
{:else if n.type === "heading" && n.depth === 5}
  <h5><RenderChildren {data} node={n} /></h5>
{:else if n.type === "heading" && n.depth === 6}
  <h6><RenderChildren {data} node={n} /></h6>
{:else if n.type === "paragraph"}
  <p><RenderChildren {data} node={n} /></p>
{:else if n.type === "text"}
  <span>{n.value}</span>
{:else if n.type === "emphasis"}
  <em><RenderChildren {data} node={n} /></em>
{:else if n.type === "strong"}
  <strong><RenderChildren {data} node={n} /></strong>
{:else if n.type === "thematicBreak"}
  <RenderVerbatim {data} node={n} />
{:else if n.type === "link"}
  <RenderChildren {data} node={n} />
{:else}
  <div style:display="inline" class={n.type}>
    <RenderVerbatim {data} node={n} />
  </div>
{/if}

<style>
  p, h1, h2, h3, h4, h5, h6 {
    display: inline;
  }
  ul,
  ol {
    list-style-type: none;
    padding: 0;
    display: inline;
  }
  li {
    display: inline;
  }
</style>
