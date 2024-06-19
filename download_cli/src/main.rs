use clap::Parser;

use crate::manager::DownloadManager;
use crate::manager_binary_heap::BinaryHeapManager;
use crate::manager_channel::ChannelManager;

mod chunk;
mod helper;
mod manager;
mod manager_binary_heap;
mod manager_channel;
mod task;

#[derive(Debug, Parser)]
#[command(name = "download_cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "down", about = "将文件块写入到同一文件中")]
    Down(Args),
    #[command(name = "channel-down", about = "使用channel同步处理进行排序")]
    ChannelDown(Args),
    #[command(name = "binary-heap-down", about = "使用最大堆对文件进行排序")]
    BinaryHeapDown(Args),
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
        SubCommand::Down(args) => {
            let manger = DownloadManager {};
            manger.execute(&args.url).await?;
        }
        SubCommand::ChannelDown(args) => {
            let manager = ChannelManager {};
            manager.execute(&args.url).await?;
        }
        SubCommand::BinaryHeapDown(args) => {
            let manager = BinaryHeapManager {};
            manager.execute(&args.url).await?;
        }
    }
    println!("Download completed successfully!");
    Ok(())
}
