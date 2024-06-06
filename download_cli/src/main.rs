mod download;

use crate::download::download;
use clap::Parser;

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
async fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Down(opts) => download(&opts.url).await.unwrap(),
    }
}
