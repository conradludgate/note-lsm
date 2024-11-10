<script lang="ts">
  import { fromMarkdown } from "mdast-util-from-markdown";
  import RenderContent from "./RenderContent.svelte";
  import RenderChildren from "./RenderChildren.svelte";

  const data = `1) Hello, _Jupiter_ and *Neptune* and **Pluto**!
2) the world is full of [cats](https://http.cat/404)

---

hello

# hello 1

## hello 2

### hello 3

#### hello 4

##### hello 5

###### hello 6
`;
  const node = fromMarkdown(data);
</script>

<RenderChildren {data} {node} />

<style></style>
