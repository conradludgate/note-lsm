use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::Mutex,
    time::Duration,
};

use jiff::Zoned;
use note_lsm_lib::{Client, RecordId, Settings};
use serde::Serialize;
use tauri::{async_runtime::spawn, Emitter, Manager};
use tokio::time::sleep;

#[derive(Serialize, Clone, Debug)]
pub struct Note {
    pub note: String,
    pub datetime: Zoned,
    pub children: Vec<RecordId>,
}

struct AppState {
    settings: Settings,
    client: tokio::sync::OnceCell<tokio::sync::Mutex<Client>>,
    cache: Mutex<Cache>,
}

impl AppState {
    async fn client(&self) -> &tokio::sync::Mutex<Client> {
        self.client
            .get_or_init(|| async {
                let client = Client::new(&self.settings).await;
                tokio::sync::Mutex::new(client)
            })
            .await
    }

    async fn create_note(&self, note: Note) -> RecordId {
        let id = self
            .client()
            .await
            .lock()
            .await
            .add_record(
                note.note.clone(),
                note.children.clone(),
                note.datetime.clone(),
            )
            .await;

        self.cache.lock().unwrap().add_note(id, note);
        id
    }
}

#[derive(Default)]
struct Cache {
    map: BTreeMap<RecordId, Note>,
    unprocessed: BTreeSet<RecordId>,
}

impl Cache {
    fn add_note(&mut self, id: RecordId, note: Note) {
        self.unprocessed.insert(id);
        for child in &note.children {
            self.unprocessed.remove(child);
        }
        self.map.insert(id, note);
    }
}

#[tauri::command]
async fn unprocessed(state: tauri::State<'_, AppState>) -> Result<Vec<RecordId>, ()> {
    let client = state.client().await;
    client.lock().await.sync(&state.settings).await;
    client
        .lock()
        .await
        .load_notes(|_host, id, note| {
            let mut cache = state.cache.lock().unwrap();
            cache.add_note(
                id,
                Note {
                    note: note.note,
                    datetime: note.datetime,
                    children: note.children,
                },
            );
        })
        .await
        .unwrap();

    let state = state.cache.lock().unwrap();

    Ok(state.unprocessed.iter().rev().copied().collect())
}

#[tauri::command]
async fn get_note(id: RecordId, state: tauri::State<'_, AppState>) -> Result<Note, ()> {
    let state = state.cache.lock().unwrap();
    state.map.get(&id).cloned().ok_or(())
}

#[tauri::command]
async fn add_note(
    note: String,
    children: Vec<RecordId>,
    state: tauri::State<'_, AppState>,
) -> Result<RecordId, ()> {
    Ok(state
        .create_note(Note {
            note,
            datetime: Zoned::now(),
            children,
        })
        .await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut settings = note_lsm_lib::Settings::new().unwrap();

    settings.record_store_path = PathBuf::from(std::env::var_os("HOME").expect("$HOME not found"))
        .join(".local")
        .join("share")
        .join("notelsm")
        .join("store.db")
        .to_str()
        .unwrap()
        .to_owned();

    let state = AppState {
        settings,
        client: tokio::sync::OnceCell::new(),
        cache: Mutex::new(Cache::default()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![unprocessed, get_note, add_note])
        .manage(state)
        .setup(|app| {
            let handle = app.handle().clone();
            spawn(async move {
                let state = handle.state::<AppState>();
                let client = state.client().await;
                loop {
                    let mut updated = false;
                    client
                        .lock()
                        .await
                        .load_notes(|_host, id, note| {
                            let mut cache = state.cache.lock().unwrap();
                            cache.add_note(
                                id,
                                Note {
                                    note: note.note,
                                    datetime: note.datetime,
                                    children: note.children,
                                },
                            );
                            updated = true;
                        })
                        .await
                        .unwrap();

                    if updated {
                        let cache = state.cache.lock().unwrap();
                        let unprocessed: Vec<RecordId> =
                            cache.unprocessed.iter().rev().copied().collect();
                        handle.emit("new-notes", unprocessed).unwrap();
                    }

                    sleep(Duration::from_secs(10)).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
