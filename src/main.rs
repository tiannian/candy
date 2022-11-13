use clap::Parser;

mod api;
mod command;
mod config;
mod ipxe;

#[tokio::main]
async fn main() {
    let args = command::Args::parse();

    args.execute().await;
}
