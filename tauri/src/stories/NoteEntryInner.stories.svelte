<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { fn } from "@storybook/test";
  import NoteEntryInner from "../components/NoteEntryInner.svelte";
  import { Temporal } from "@js-temporal/polyfill";

  // More on how to set up stories at: https://storybook.js.org/docs/writing-stories
  const { Story } = defineMeta({
    title: "NoteEntryInner",
    component: NoteEntryInner,
    tags: ["autodocs"],
    argTypes: {
      text: {
        control: "text",
        description: "Overwritten description",
      },
    },
    args: {
      currentTime: Temporal.ZonedDateTime.from(
        "2024-06-19T15:22:45-04:00[America/New_York]"
      ),
      open: fn(),
      select: fn(),
    },
  });
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story
  name="Selected note"
  args={{
    datetime: Temporal.ZonedDateTime.from(
      "2024-06-19T21:21:24+02:00[Europe/Berlin]"
    ),
    text: "this is a test note",
    opened: false,
    selected: true,
  }}
/>
