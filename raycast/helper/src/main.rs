use std::{io, path::PathBuf};

use clap::Parser;
use jiff::Zoned;
use note_lsm_lib::Client;
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

    let mut settings = note_lsm_lib::Settings::new().unwrap();

    settings.record_store_path = PathBuf::from(std::env::var_os("HOME").expect("$HOME not found"))
        .join(".local")
        .join("share")
        .join("notelsm")
        .join("store.db")
        .to_str()
        .unwrap()
        .to_owned();

    let mut client = Client::new(&settings).await;

    let output: Box<dyn EncodeOutput> = match args.command {
        Command::Record(record_args) => {
            println!("adding {:?}", record_args.note);

            client
                .add_record(record_args.note, vec![], Zoned::now())
                .await;
            if settings.should_sync().unwrap() {
                client.sync(&settings).await;
            }

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
