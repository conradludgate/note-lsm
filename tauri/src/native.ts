import { invoke } from "@tauri-apps/api/core";

interface Note {
    note: string;
    datetime: string;
    children: string[];
}

export async function getNote(id: string): Promise<Note> {
    return await invoke<Note>("get_note", { id });
}

export async function unprocessed(): Promise<string[]> {
    return await invoke<string[]>("unprocessed", {})
}

export async function addNote(note: string, children: string[]): Promise<string> {
    return await invoke<string>("add_note", { note, children })
}
