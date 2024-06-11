use std::{
    collections::BinaryHeap,
    sync::{Arc, Mutex},
};

use crate::{helper::HeapSortWrapper, manager::DownloadManager};

use clap::Parser;

mod chunk;
mod helper;
mod manager;

#[derive(Debug, Parser)]
#[command(name = "download_cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "down", about = "根据url进行多线程下载")]
    Down(Args),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Down(opts) => {
            let sort_queue: HeapSortWrapper = Arc::new(Mutex::new(BinaryHeap::new()));

            let manger = DownloadManager::new(&opts.url, sort_queue);

            let content_length = manger.get_content_length().await?;
            let tasks = manger.create_chunk_tasks(content_length);

            manger.execute_tasks(tasks).await?;

            println!("Download completed successfully!");
        }
    }
    Ok(())
}
