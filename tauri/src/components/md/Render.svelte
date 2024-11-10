<script lang="ts">
  import { fromMarkdown } from "mdast-util-from-markdown";
  import RenderChildren from "./RenderChildren.svelte";
  import type { Action } from "svelte/action";
  import type { FormEventHandler } from "svelte/elements";

  let data = $state(`1) Hello, _Jupiter_ and *Neptune* and **Pluto**[0]!
2) the world is full of [cats](https://http.cat/404)

[0]: https://http.cat/418

---

hello

# hello 1 

## hello 2

### hello 3

#### hello 4

##### hello 5

###### hello 6

\`\`\`rust
lol
\`\`\`
`);
  const node = $derived(fromMarkdown(data));

  const check: Action = $derived((renderNode) => {
    console.log(renderNode.innerText === data);
  });
  const update: FormEventHandler<HTMLDivElement> = $derived((event) => {
    console.log(event.currentTarget.innerText);
    data = event.currentTarget.innerText;
  });
</script>

<div use:check contenteditable="plaintext-only" oninput={update}>
  <RenderChildren {data} {node} />
</div>

<style></style>
