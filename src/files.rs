use anyhow::Result;
use std::{io, path};
use tokio::io::AsyncWriteExt;

pub async fn write_files(response: reqwest::Response, path: path::PathBuf) -> Result<()> {
    let dir = path::Path::new("/data");
    let file_path =
        path::Path::new(dir.file_name().unwrap()).join(path.as_path().file_name().unwrap());

    tokio::fs::create_dir_all(dir.file_name().unwrap()).await?;

    let mut file = tokio::fs::File::create(&file_path).await?;
    let mut content = io::Cursor::new(response.bytes().await?);

    tokio::io::copy(&mut content, &mut file).await?;
    file.flush().await?;
    println!(
        "{} downloaded...",
        &file_path.file_name().unwrap().to_str().unwrap().to_string()
    );
    Ok(())
}

// pub fn move_files() {
//      //TODO move files to certain destination by arg -m
// }

pub async fn remove_files() {
    let dir = path::Path::new("data");
    if !dir.is_dir() {
        eprintln!("No data found to clean...");
        return;
    }
    tokio::fs::remove_dir_all(dir).await.unwrap();
    tokio::fs::create_dir(dir).await.unwrap();
    println!("Data cleaned...");
}

pub async fn list_files() {
    let mut entries = tokio::fs::read_dir("data").await.unwrap();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        println!("{:?}", entry.file_name())
    }
}
