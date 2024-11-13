<script module>
  let now = $state(Temporal.Now.zonedDateTimeISO().toString());
  const id = setInterval(
    () => { now = Temporal.Now.zonedDateTimeISO().toString(); },
    1000
  );
</script>

<script lang="ts">
  import { Temporal } from "@js-temporal/polyfill";
  import { formatTime } from "../time";

  interface Props {
    datetime: string;
  }

  let { datetime }: Props = $props();

  let date = $derived.by(() => {
    if (datetime === "") return "";

    let now2 = Temporal.ZonedDateTime.from(now);
    let dt = Temporal.ZonedDateTime.from(datetime);

    return formatTime(dt, now2);
  });
</script>

<span>{date}</span>

<style></style>
