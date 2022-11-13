use clap::{Parser, Subcommand};

use crate::api;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "/etc/candy.conf")]
    config: String,

    #[command(subcommand)]
    subcommand: Sub,
}

impl Args {
    pub async fn execute(&self) {
        match self.subcommand {
            Sub::Init => {}
            Sub::Start => {
                let api = api::Api {
                    config: self.config.clone(),
                };
                api::start(false, api).await.unwrap();
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Sub {
    Init,
    Start,
}
