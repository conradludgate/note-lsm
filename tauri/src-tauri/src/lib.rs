use jiff::{civil::datetime, tz::TimeZone, Zoned};
use note_lsm_lib::RecordId;
use serde::Serialize;
use uuid::uuid;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
pub struct Note {
    pub note: String,
    pub datetime: Zoned,
    pub children: Vec<RecordId>,
}

const FOO: RecordId = RecordId(uuid!("b7d8ac79-4e91-4af8-b164-6e14212531a8"));
const FOO2: RecordId = RecordId(uuid!("68563c3e-5e54-4388-8fba-c1699a25817e"));
const BAR: RecordId = RecordId(uuid!("46924371-80e2-41ab-85ac-c44d9cb90d81"));
const BAZ: RecordId = RecordId(uuid!("3965e843-d386-424d-9c32-f5d0d4234641"));

#[tauri::command]
async fn unprocessed() -> Vec<RecordId> {
    vec![FOO, BAR, BAZ]
}

#[tauri::command]
async fn get_note(id: RecordId) -> Note {
    match id {
        FOO => Note {
            note: "this is a long note just to test how the over flow is handled lol ok bye wait on this line is not long enough yet lol ok now try".to_string(),
            datetime: datetime(2024, 11, 10, 10, 30, 22, 0)
                .to_zoned(TimeZone::get("Europe/Paris").unwrap())
                .unwrap(),
            children: vec![FOO2],
        },
        FOO2 => Note {
            note: "foo/foo2".to_string(),
            datetime: datetime(2024, 11, 10, 2, 0, 0, 0)
                .to_zoned(TimeZone::get("Europe/London").unwrap())
                .unwrap(),
            children: vec![],
        },
        BAR => Note {
            note: "bar".to_string(),
            datetime: datetime(2024, 11, 9, 12, 19, 22, 0)
                .to_zoned(TimeZone::get("Europe/Paris").unwrap())
                .unwrap(),
            children: vec![],
        },
        BAZ => Note {
            note: "my birthday :3".to_string(),
                datetime: datetime(2023, 12, 19, 11, 19, 22, 0)
                    .to_zoned(TimeZone::get("Europe/Paris").unwrap())
                    .unwrap(),
            children: vec![],
        },
        _ => panic!("unknown note"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, unprocessed, get_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
