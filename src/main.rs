use std::{io, path::PathBuf};

use atuin_client::{
    encryption,
    record::{encryption::PASETO_V4, sqlite_store::SqliteStore, store::Store},
};
use atuin_common::record::{DecryptedData, Host, Record};
use clap::Parser;
// use comfy_table::Table;
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(short, long, default_value = "text")]
    output: Output,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Record(RecordArgs),
}

#[derive(clap::Parser, Debug)]
struct RecordArgs {
    note: String,
}

#[derive(clap::ValueEnum, Default, Clone, Copy, Debug)]
enum Output {
    #[default]
    Text,
    Json,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    let settings = atuin_client::settings::Settings::new().unwrap();
    let sqlite_store = SqliteStore::new(&settings.record_store_path, settings.local_timeout)
        .await
        .unwrap();

    let host_id = atuin_client::settings::Settings::host_id().expect("failed to get host_id");
    let key = encryption::load_key(&settings).unwrap().into();

    let output: Box<dyn EncodeOutput> = match args.command {
        Command::Record(record_args) => {
            println!("adding {:?}", record_args.note);

            let idx = sqlite_store
                .last(host_id, "notes_lsm::record")
                .await
                .unwrap()
                .map_or(0, |p| p.idx + 1);

            let record = Record::builder()
                .data(DecryptedData(
                    serde_json::to_vec(&NoteLeaf {
                        note: record_args.note,
                    })
                    .unwrap(),
                ))
                .tag("notes_lsm::record".to_string())
                .idx(idx)
                .host(Host::new(host_id))
                .version("v0-alpha1".to_string())
                .build();
            let record = record.encrypt::<PASETO_V4>(&key);
            sqlite_store.push(&record).await.unwrap();

            Box::new(NoOutput {})
        }
    };

    let mut stdout = io::stdout();
    output.encode(args.output, &mut stdout).unwrap();
}

trait EncodeOutput {
    fn encode(self: Box<Self>, method: Output, w: &mut dyn io::Write) -> io::Result<()>;
}

struct NoOutput {}

impl EncodeOutput for NoOutput {
    fn encode(self: Box<Self>, method: Output, w: &mut dyn io::Write) -> io::Result<()> {
        match method {
            Output::Text => Ok(()),
            Output::Json => json(&serde_json::Value::Null, w),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tag {
    tag: String,
    description: Option<String>,
}

pub fn json<T>(value: &T, w: &mut dyn io::Write) -> io::Result<()>
where
    T: ?Sized + Serialize,
{
    serde_json::to_writer_pretty(&mut *w, value).map_err(json_io_error)?;
    writeln!(w)
}

fn json_io_error(e: serde_json::Error) -> io::Error {
    io::Error::new(e.io_error_kind().unwrap_or(io::ErrorKind::Other), e)
}

#[derive(Serialize, Deserialize)]
struct NoteLeaf {
    note: String,
}
