use args::Commands;
use clap::Parser;
use files::{list_files, remove_files};
use get::{get_many, get_single};

mod archive;
mod args;
mod files;
mod get;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
    /// Lists downloaded files
    #[arg(long)]
    list: bool,
    /// Cleans downloaded files
    #[arg(long)]
    clean: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Some(Commands::FileDownloader(ref args)) => {
            if args.many_files {
                get_many(args).await;
            } else {
                get_single(args).await;
            }
        }
        None => (),
    }
    if cli.list {
        list_files().await;
    } else if cli.clean {
        remove_files().await;
    } else {
        ();
    }
}
