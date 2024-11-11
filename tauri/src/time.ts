import type { Temporal } from "@js-temporal/polyfill";

export const formatTime = (dt: Temporal.ZonedDateTime, now: Temporal.ZonedDateTime, locales?: string | string[]): string => {
    let duration = dt.until(now).round({ smallestUnit: "seconds", roundingMode: "trunc" });
    let totalSeconds = duration.total("seconds");

    let timeZoneName: "short" | undefined = "short";
    if (dt.timeZoneId == now.timeZoneId) {
        timeZoneName = undefined;
    }

    // edge case.
    if (totalSeconds < -30) {
        return dt.toLocaleString(locales, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit', timeZoneName });
    }

    if (totalSeconds < 30) {
        return "now"
    }

    if (totalSeconds < 90) {
        let seconds = duration.round({ largestUnit: "seconds", smallestUnit: "seconds", roundingMode: "trunc" }).total("seconds");
        return `${seconds}s ago`
    }

    if (totalSeconds < 90 * 60) {
        let minutes = duration.round({ largestUnit: "minutes", smallestUnit: "minutes", roundingMode: "trunc" }).total("minutes");
        return `${minutes}m ago`
    }

    if (dt.toPlainDate().equals(now.toPlainDate())) {
        return dt.toLocaleString(locales, { hour: '2-digit', minute: '2-digit', timeZoneName });
    }

    if (dt.year === now.year) {
        return dt.toLocaleString(locales, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit', timeZoneName });
    }

    return dt.toLocaleString(locales, { year: 'numeric', month: 'numeric', day: 'numeric' });
}
