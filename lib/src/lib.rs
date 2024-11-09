use atuin_client::{
    encryption,
    record::{encryption::PASETO_V4, sqlite_store::SqliteStore, store::Store},
};
use atuin_common::record::{DecryptedData, Host, HostId, Record};
use serde::{Deserialize, Serialize};

pub use atuin_common::record::RecordId;

pub struct Client {
    settings: atuin_client::settings::Settings,
    store: SqliteStore,
    host_id: HostId,
    key: [u8; 32],
}

impl Client {
    pub async fn new() -> Self {
        let settings = atuin_client::settings::Settings::new().unwrap();
        let store = SqliteStore::new(&settings.record_store_path, settings.local_timeout)
            .await
            .unwrap();

        let host_id = atuin_client::settings::Settings::host_id().expect("failed to get host_id");
        let key = encryption::load_key(&settings).unwrap().into();

        Self {
            settings,
            store,
            host_id,
            key,
        }
    }

    pub async fn add_record(&self, note: String, children: Vec<RecordId>) {
        let idx = self
            .store
            .last(self.host_id, "notes_lsm::record")
            .await
            .unwrap()
            .map_or(0, |p| p.idx + 1);

        let record = Record::builder()
            .data(DecryptedData(
                serde_json::to_vec(&Note { note, children }).unwrap(),
            ))
            .tag("notes_lsm::record".to_string())
            .idx(idx)
            .host(Host::new(self.host_id))
            .version("v0-alpha2".to_string())
            .build();
        let record = record.encrypt::<PASETO_V4>(&self.key);
        self.store.push(&record).await.unwrap();
    }
}

#[derive(Serialize, Deserialize)]
struct Note {
    note: String,
    children: Vec<RecordId>,
}
