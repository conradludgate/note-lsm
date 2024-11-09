<script lang="ts">
  interface Props {
    key: string[];
    datetime: string;
    text: string;
    selectStack: string[];
  }

  let { key, datetime, text, selectStack = $bindable() }: Props = $props();

  let isSelected = $derived(arrayEqual(selectStack, key));

  function arrayEqual(a: string[], b: string[]): boolean {
    return a.length === b.length && a.every((el, ix) => el === b[ix]);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="inner"
  class:selected={isSelected}
  onclick={() => {
    selectStack = key;
  }}
>
  <span>{datetime}</span><br />
  <span>{text}</span>
</div>

<style>
  .inner {
    height: 50px;
    transition: 150ms;
    padding-top: 10px;
    padding-bottom: 10px;
  }

  .inner > span {
    font-size: 12pt;
  }

  .inner.selected {
    background-color: rgb(227, 227, 227);
  }

  .inner:hover {
    background-color: rgb(227, 227, 227);
  }
</style>
