<script module>
  let now = $state(Temporal.Now.zonedDateTimeISO().toString());
  const id = setInterval(
    () => { now = Temporal.Now.zonedDateTimeISO().toString(); },
    1000
  );
</script>

<script lang="ts">
  import { Temporal, Intl, toTemporalInstant } from "@js-temporal/polyfill";

  interface Props {
    key: string[];
    datetime: string;
    text: string;
    selectStack: string[];
  }

  let { key, datetime, text, selectStack = $bindable() }: Props = $props();

  let isSelected = $derived(arrayEqual(selectStack, key));

  let date = $derived.by(() => {
    if (datetime === "") {
      return "";
    }
    let now2 = Temporal.ZonedDateTime.from(now);
    let dt = Temporal.ZonedDateTime.from(datetime);

    let duration = dt.until(now2).round({ smallestUnit: "seconds", roundingMode: "trunc"});

    let totalSeconds = duration.total("seconds");
    // edge case.
    if (totalSeconds < -30) {
      return dt.toLocaleString();
    }

    if (totalSeconds < 30) {
      return "now"
    }

    if (totalSeconds < 90) {
      let seconds = duration.round({ largestUnit: "seconds", smallestUnit: "seconds", roundingMode: "trunc"}).total("seconds");
      return `${seconds}s ago`
    }

    if (totalSeconds < 90*60) {
      let minutes = duration.round({ largestUnit: "minutes", smallestUnit: "minutes", roundingMode: "trunc"}).total("minutes");
      return `${minutes}m ago`
    }

    let dtMinutes = dt.round("minute");

    let timeZoneName: "short" | undefined = "short";
    if (dtMinutes.timeZoneId == now2.timeZoneId) {
      timeZoneName = undefined;
    }

    if (dtMinutes.toPlainDate().equals(now2.toPlainDate())) {
      return dtMinutes.toLocaleString([], {hour: '2-digit', minute:'2-digit', timeZoneName});
    }

    if (dtMinutes.year === now2.year) {
      return dtMinutes.toLocaleString([], {month: 'short', day: 'numeric', hour: '2-digit', minute:'2-digit', timeZoneName});
    }

    return dtMinutes.toLocaleString([], {year: 'numeric', month: 'numeric', day: 'numeric'});
  });

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
  <p class="text">{text}</p>
  <span class="date">{date}</span>
</div>

<style>
  .inner {
    height: 3.5em;
    transition: 150ms;
    padding-top: 15px;
    padding-bottom: 15px;
    padding-left: 15px;
    padding-right: 15px;
    position: relative;
    margin-left: -8px;
    border-bottom-left-radius: 4px;
  }

  .inner.selected {
    background-color: rgb(227, 227, 227);
  }

  .inner:hover {
    background-color: rgb(227, 227, 227);
  }

  .inner > .text {
    height: 2.4em;
    font-size: 1em;
    line-height: 1.2em;
    margin-top: 0;
    text-align: left;
    overflow: clip;
    text-overflow: ellipsis;
    text-wrap-style: pretty;
  }

  .inner > .date {
    font-size: 1em;
    position: absolute;
    right: 0.4em;
    bottom: 0.4em;
  }
</style>
