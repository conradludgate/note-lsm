use std::{io, path::PathBuf};

use atuin_client::{
    encryption,
    record::{encryption::PASETO_V4, sqlite_store::SqliteStore, store::Store},
};
use atuin_common::record::{DecryptedData, Host, Record};
use clap::Parser;
use comfy_table::Table;
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
    Tags(TagsArgs),
    Record(RecordArgs),
}

#[derive(clap::Parser, Debug)]
struct TagsArgs {
    #[command(subcommand)]
    command: TagsCommand,
}

#[derive(clap::Subcommand, Debug)]
enum TagsCommand {
    List,
    Add {
        tag: String,
        description: Option<String>,
    },
}

#[derive(clap::Parser, Debug)]
struct RecordArgs {
    #[arg(short, long = "tag")]
    tags: Vec<String>,

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
        Command::Tags(tags_args) => match tags_args.command {
            TagsCommand::List => {
                let tags = sqlite_store
                    .all_tagged("notes_lsm::tag")
                    .await
                    .unwrap()
                    .into_iter()
                    .filter(|encrypted| encrypted.version == "v0-alpha0")
                    .map(|encrypted| encrypted.decrypt::<PASETO_V4>(&key).unwrap())
                    .map(|decrypted| serde_json::from_slice(&decrypted.data.0).unwrap())
                    .collect();

                Box::new(TagsListOutput { tags })
            }
            TagsCommand::Add { tag, description } => {
                println!("adding tag {tag:?} with description {description:?}",);

                let idx = sqlite_store
                    .last(host_id, "notes_lsm::tag")
                    .await
                    .unwrap()
                    .map_or(0, |p| p.idx + 1);

                let record = Record::builder()
                    .data(DecryptedData(
                        serde_json::to_vec(&Tag { tag, description }).unwrap(),
                    ))
                    .tag("notes_lsm::tag".to_string())
                    .idx(idx)
                    .host(Host::new(host_id))
                    .version("v0-alpha0".to_string())
                    .build();
                let record = record.encrypt::<PASETO_V4>(&key);
                sqlite_store.push(&record).await.unwrap();

                Box::new(NoOutput {})
            }
        },
        Command::Record(record_args) => {
            println!(
                "adding {:?} with tags {:?}",
                record_args.note, record_args.tags
            );

            let idx = sqlite_store
                .last(host_id, "notes_lsm::record")
                .await
                .unwrap()
                .map_or(0, |p| p.idx + 1);

            let record = Record::builder()
                .data(DecryptedData(
                    serde_json::to_vec(&NoteLeaf {
                        note: record_args.note,
                        tags: record_args.tags,
                    })
                    .unwrap(),
                ))
                .tag("notes_lsm::record".to_string())
                .idx(idx)
                .host(Host::new(host_id))
                .version("v0-alpha0".to_string())
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

struct TagsListOutput {
    tags: Vec<Tag>,
}

impl EncodeOutput for TagsListOutput {
    fn encode(self: Box<Self>, method: Output, w: &mut dyn io::Write) -> io::Result<()> {
        match method {
            Output::Text => {
                let mut table = Table::new();
                table.set_header(["tag", "description"]).add_rows(
                    self.tags
                        .into_iter()
                        .map(|tag| [tag.tag, tag.description.unwrap_or_default()]),
                );
                writeln!(w, "{table}")
            }
            Output::Json => json(&self.tags, w),
        }
    }
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
    tags: Vec<String>,
}
