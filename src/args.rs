use clap::{Args, Subcommand};
use reqwest;
use std::path;

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///  files using HTTP protocol
    #[command(name = "--file-downloader", visible_alias = "-fd")]
    FileDownloader(FileDownloader),
}

#[derive(Args, Debug)]
pub struct FileDownloader {
    /// Url for the file to be downloaded
    #[arg(long, short, num_args = 1.., value_delimiter = ' ')]
    pub url: Vec<reqwest::Url>,
    /// Name of output file
    #[arg(long, short, num_args = 1.., value_delimiter = ' ')]
    pub output: Vec<path::PathBuf>,
    /// Downloads multiple files at same time
    #[arg(long, short, required = false, requires_all(["url","output"]))]
    pub many_files: bool,
}
