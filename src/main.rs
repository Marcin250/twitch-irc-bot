mod utils;
mod listener;
mod sender;
mod configuration;

use dotenv::dotenv;
use listener::listen;
use sender::{sendAsync, queueMessage};

#[tokio::main]
pub async fn main() -> ()
{
    dotenv().ok();

    let cmd = clap::Command::new("cargo")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            clap::command!("app:listen")
        )
        .subcommand(
            clap::command!("app:send")
        )
        .subcommand(
            clap::command!("app:queue-message")
        );
    let matches = cmd.get_matches();

    match matches.subcommand_name() {
        Some("app:listen") => listen().await,
        Some("app:send") => sendAsync().await.unwrap(),
        Some("app:queue-message") => queueMessage(),
        _ => listen().await,
    }
}