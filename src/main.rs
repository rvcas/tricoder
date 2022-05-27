use std::env;

use clap::Parser;

mod cli;
mod common_ports;
mod dns;
mod error;

mod modules;
mod ports;

#[derive(Parser)]
enum Args {
    Scan { target: String },
    Modules,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let args = Args::parse();

    match args {
        Args::Scan { target } => {
            cli::scan(&target)?;
        }
        Args::Modules => {
            cli::modules();
        }
    }

    Ok(())
}
