mod cli;
mod graph_utils;
mod i18n;
mod init;
mod ui;
mod update_check;
mod verbosity;
mod wiki;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
