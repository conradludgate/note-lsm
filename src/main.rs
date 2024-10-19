use std::{io, path::PathBuf};

use clap::Parser;
use comfy_table::Table;
use serde::Serialize;

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

fn main() {
    let args = Args::parse();

    let output: Box<dyn EncodeOutput> = match args.command {
        Command::Tags(tags_args) => match tags_args.command {
            TagsCommand::List => Box::new(TagsListOutput { tags: get_tags() }),
            TagsCommand::Add { tag, description } => {
                println!("adding tag {tag:?} with description {description:?}",);
                Box::new(NoOutput {})
            }
        },
        Command::Record(record_args) => {
            println!(
                "adding {:?} with tags {:?}",
                record_args.note, record_args.tags
            );
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
                table
                    .set_header(["tag", "description"])
                    .add_rows(self.tags.into_iter().map(|tag| [tag.tag, tag.description]));
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

#[derive(Serialize)]
struct Tag {
    tag: String,
    description: String,
}

fn get_tags() -> Vec<Tag> {
    vec![
        Tag {
            tag: "work".to_string(),
            description: "Work related things".to_string(),
        },
        Tag {
            tag: "personal".to_string(),
            description: "Personal related things".to_string(),
        },
    ]
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
