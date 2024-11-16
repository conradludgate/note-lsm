import { Temporal } from "@js-temporal/polyfill";
import { invoke } from "@tauri-apps/api/core";

interface NoteInner {
    note: string;
    datetime: string;
    children: string[];
}

export interface Note {
    note: string;
    datetime: Temporal.ZonedDateTime;
    children: string[];
}

export async function getNote(id: string): Promise<Note> {
    let { note, datetime, children } = await invoke<NoteInner>("get_note", { id });
    return {
        note, children, datetime: Temporal.ZonedDateTime.from(datetime),
    };
}

export async function unprocessed(): Promise<string[]> {
    return await invoke<string[]>("unprocessed", {})
}

export async function addNote(note: string, children: string[]): Promise<string> {
    return await invoke<string>("add_note", { note, children })
}
