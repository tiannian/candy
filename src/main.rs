use clap::Parser;

mod api;
mod command;
mod config;
mod ipxe;

mod error;
pub use error::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = command::Args::parse();

    args.execute().await;
}
