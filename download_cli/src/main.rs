use clap::Parser;

use crate::manager::DownloadManager;

mod chunk;
mod helper;
mod manager;
mod task;

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
            let manger = DownloadManager {};

            manger.execute(&opts.url).await?;

            println!("Download completed successfully!");
        }
    }
    Ok(())
}
