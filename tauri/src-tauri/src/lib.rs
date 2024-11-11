use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Mutex, OnceLock},
};

use jiff::{civil::datetime, tz::TimeZone, Timestamp, Zoned};
use note_lsm_lib::RecordId;
use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Clone, Debug)]
pub struct Note {
    pub note: String,
    pub datetime: Zoned,
    pub children: Vec<RecordId>,
}

static STATE: OnceLock<Mutex<BTreeMap<RecordId, Note>>> = OnceLock::new();
static UNPROCESSED: OnceLock<Mutex<BTreeSet<RecordId>>> = OnceLock::new();

fn create_note(note: Note) -> RecordId {
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
    add_note(id, note);
    id
}

fn add_note(id: RecordId, note: Note) {
    let mut state = STATE.get().unwrap().lock().unwrap();
    let mut unprocessed = UNPROCESSED.get().unwrap().lock().unwrap();

    unprocessed.insert(id);
    for child in &note.children {
        unprocessed.remove(child);
    }
    state.insert(id, note);
}

#[tauri::command]
async fn unprocessed() -> Vec<RecordId> {
    let unprocessed = UNPROCESSED.get().unwrap().lock().unwrap();

    unprocessed.iter().rev().copied().collect()
}

#[tauri::command]
async fn get_note(id: RecordId) -> Note {
    let state = STATE.get().unwrap().lock().unwrap();
    state.get(&id).cloned().unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    STATE.set(Mutex::new(BTreeMap::new())).unwrap();
    UNPROCESSED.set(Mutex::new(BTreeSet::new())).unwrap();

    create_note(Note {
        note: "my birthday :3".to_string(),
        datetime: datetime(2023, 12, 19, 11, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![],
    });

    create_note(Note {
        note: "bar".to_string(),
        datetime: datetime(2024, 11, 9, 12, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![],
    });

    let foo2 = create_note(Note {
        note: "foo/foo2".to_string(),
        datetime: datetime(2024, 11, 10, 2, 0, 59, 0)
            .to_zoned(TimeZone::get("Europe/London").unwrap())
            .unwrap(),
        children: vec![],
    });

    let long_note = "this is a long note just to test how the over flow is handled lol ok bye wait on this line is not long enough yet lol ok now try".to_string();
    create_note(Note {
        note: long_note,
        datetime: datetime(2024, 11, 10, 10, 30, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap(),
        children: vec![foo2],
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, unprocessed, get_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
