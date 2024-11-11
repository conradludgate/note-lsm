import { describe, expect, it, test } from 'vitest'
import { formatTime } from './time';
import { Temporal } from '@js-temporal/polyfill';

let now = Temporal.ZonedDateTime.from("2024-06-19T15:22:45-04:00[America/New_York]")

describe("formatTime", () => {
    describe("locale[en-GB]", () => {
        let locales = "en-GB";
        describe("same timezone", () => {
            it('just now', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:22:24-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("now")
            });

            it('recently', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:22:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("34s ago")
            });

            it('earlier', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:15:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7m ago")
            });

            it('today', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("10:21")
            });

            it('last month', () => {
                let dt = Temporal.ZonedDateTime.from("2024-05-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("19 May, 10:21")
            });

            it('last year', () => {
                let dt = Temporal.ZonedDateTime.from("2023-07-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("19/07/2023")
            });
        })

        describe("different timezone", () => {
            it('just now', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:22:24+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("now")
            });

            it('recently', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:22:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("34s ago")
            });

            it('earlier', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:15:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7m ago")
            });

            it('today', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("16:21 CEST")
            });

            it('last month', () => {
                let dt = Temporal.ZonedDateTime.from("2024-05-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("19 May, 16:21 CEST")
            });

            it('last year', () => {
                let dt = Temporal.ZonedDateTime.from("2023-07-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("19/07/2023")
            });
        })
    })

    describe("locale[en-US]", () => {
        let locales = "en-US";
        describe("same timezone", () => {
            it('just now', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:22:24-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("now")
            });

            it('recently', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:22:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("34s ago")
            });

            it('earlier', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T15:15:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7m ago")
            });

            it('today', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("10:21 AM")
            });

            it('last month', () => {
                let dt = Temporal.ZonedDateTime.from("2024-05-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("May 19, 10:21 AM")
            });

            it('last year', () => {
                let dt = Temporal.ZonedDateTime.from("2023-07-19T10:21:11-04:00[America/New_York]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7/19/2023")
            });
        })

        describe("different timezone", () => {
            it('just now', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:22:24+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("now")
            });

            it('recently', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:22:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("34s ago")
            });

            it('earlier', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T21:15:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7m ago")
            });

            it('today', () => {
                let dt = Temporal.ZonedDateTime.from("2024-06-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("04:21 PM GMT+2")
            });

            it('last month', () => {
                let dt = Temporal.ZonedDateTime.from("2024-05-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("May 19, 04:21 PM GMT+2")
            });

            it('last year', () => {
                let dt = Temporal.ZonedDateTime.from("2023-07-19T16:21:11+02:00[Europe/Paris]")

                let time = formatTime(dt, now, locales);
                expect(time).toBe("7/19/2023")
            });
        })
    })
})
