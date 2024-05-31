use anyhow::Result;
use reqwest;
use std::path;

use crate::{args::FileDownloader, files::write_files};

async fn get(url: reqwest::Url, path: path::PathBuf) -> Result<()> {
    let res = reqwest::get(url).await?;

    write_files(res, path).await?;
    Ok(())
}

pub async fn get_many(cli: &FileDownloader) {
    if cli.url.is_empty() || cli.output.is_empty() || cli.url.len().ne(&cli.output.len()) {
        eprintln!("ERROR:Not Enough arguements...");
        return;
    }
    let mut result = Ok(());
    for (url, path) in cli.url.iter().by_ref().zip(cli.output.iter().by_ref()) {
        result = get(url.to_owned(), path.to_owned()).await;
    }
    if result.is_err() {
        eprintln!("ERROR:{}", result.err().unwrap().to_string())
    }
}

pub async fn get_single(cli: &FileDownloader) {
    if cli.url.is_empty() || cli.output.is_empty() {
        eprintln!("ERROR:Not Enough arguements...");
        return;
    }
    if cli.url.len() > 1 || cli.output.len() > 1 {
        eprintln!("ERROR:Arguement -m missing for multi file download...");
        return;
    }
    let result = get(
        cli.url.get(0).unwrap().to_owned(),
        cli.output.get(0).unwrap().to_owned(),
    )
    .await;
    if result.is_err() {
        eprintln!("ERROR:{}", result.err().unwrap().to_string());
    }
}
