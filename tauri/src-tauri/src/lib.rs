use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};

use jiff::{civil::datetime, tz::TimeZone, Timestamp, Zoned};
use note_lsm_lib::RecordId;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Note {
    pub note: String,
    pub datetime: Zoned,
    pub children: Vec<RecordId>,
}

#[derive(Default)]
struct AppState {
    map: BTreeMap<RecordId, Note>,
    unprocessed: BTreeSet<RecordId>,
}

impl AppState {
    fn create_note(&mut self, note: Note) -> RecordId {
        let timestamp = note
            .datetime
            .timestamp()
            .duration_since(Timestamp::UNIX_EPOCH);
        let timestamp = uuid::Timestamp::from_unix(
            uuid::NoContext,
            timestamp.as_secs() as u64,
            timestamp.subsec_nanos() as u32,
        );
        let id = RecordId(uuid::Uuid::new_v7(timestamp));
        self.add_note(id, note);
        id
    }

    fn add_note(&mut self, id: RecordId, note: Note) {
        self.unprocessed.insert(id);
        for child in &note.children {
            self.unprocessed.remove(child);
        }
        self.map.insert(id, note);
    }
}

#[tauri::command]
async fn unprocessed(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<RecordId>, ()> {
    let state = state.lock().unwrap();

    Ok(state.unprocessed.iter().rev().copied().collect())
}

#[tauri::command]
async fn get_note(id: RecordId, state: tauri::State<'_, Mutex<AppState>>) -> Result<Note, ()> {
    let state = state.lock().unwrap();
    state.map.get(&id).cloned().ok_or(())
}

#[tauri::command]
async fn add_note(note: String, children: Vec<RecordId>, state: tauri::State<'_, Mutex<AppState>>) -> Result<RecordId, ()> {
    let mut state = state.lock().unwrap();
    Ok(state.create_note(Note{note,datetime:Zoned::now(),children}))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut state = AppState::default();

    state.create_note(Note {
        note: "my birthday :3".to_string(),
        datetime: datetime(2023, 12, 19, 11, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![],
    });

    state.create_note(Note {
        note: "bar".to_string(),
        datetime: datetime(2024, 11, 9, 12, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![],
    });

    let foo2 = state.create_note(Note {
        note: "foo/foo2".to_string(),
        datetime: datetime(2024, 11, 10, 2, 0, 59, 0)
            .to_zoned(TimeZone::get("Europe/London").unwrap())
            .unwrap(),
        children: vec![],
    });

    let long_note = "this is a long note just to test how the over flow is handled lol ok bye wait on this line is not long enough yet lol ok now try".to_string();
    state.create_note(Note {
        note: long_note,
        datetime: datetime(2024, 11, 10, 10, 30, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![foo2],
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![unprocessed, get_note, add_note])
        .manage(Mutex::new(state))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
