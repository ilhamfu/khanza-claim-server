use std::{process::Stdio, sync::Arc};

use anyhow::{Context, Ok};
use bg::{export_one, export_range, export_raw};
use clap::Parser;
use cli::{
    KhanzaKlaimExportSubcommand, KhanzaKlaimParser, KhanzaKlaimRunSubcommand, KhanzaKlaimSubcommand,
};
use config::Config;
use logger::init_logger;
use sqlx::MySqlPool;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

mod bg;
mod config;
mod dto;
mod report;
mod util;

mod cli;
mod logger;

pub struct AppContext {
    pub pool: MySqlPool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if dotenvy::dotenv().is_err() {
        tracing::warn!("`.env` file is not found, skip loading environment variable")
    }

    let KhanzaKlaimParser {
        command,
        verbose,
        config,
    } = KhanzaKlaimParser::parse();

    if verbose {
        init_logger()?;
    }

    let pool = sqlx::MySqlPool::connect(&config.database_url).await?;

    let app_context = Arc::new(AppContext { config, pool });
    let cancel_token = CancellationToken::new();

    tokio::try_join! {
        run_command(&cancel_token, app_context.clone(), command),
        shutdown_signal(&cancel_token)
    }?;

    Ok(())
}

async fn run_command(
    cancel_token: &CancellationToken,
    app_context: Arc<AppContext>,
    command: KhanzaKlaimSubcommand,
) -> anyhow::Result<()> {
    let app_context = app_context.clone();
    let token = cancel_token.clone();
    match command {
        KhanzaKlaimSubcommand::Run(KhanzaKlaimRunSubcommand { to, from, .. }) => {
            start_with_driver(
                token.clone(),
                tokio::task::spawn(async move {
                    export_range(token.clone(), app_context, from, to).await?;
                    Ok(())
                }),
            )
            .await?;
        }
        KhanzaKlaimSubcommand::Export(KhanzaKlaimExportSubcommand { no_rawat, force }) => {
            start_with_driver(
                token.clone(),
                tokio::task::spawn(async move {
                    export_one(app_context, &no_rawat, force).await?;
                    Ok(())
                }),
            )
            .await?;
        }
        KhanzaKlaimSubcommand::Raw(cli::KhanzaKlaimRawCommand { no_rawat }) => {
            export_raw(&app_context, &no_rawat).await?;
            token.cancel();
        }
    };

    Ok(())
}

async fn start_with_driver<A>(
    cancel_token: CancellationToken,
    join_handle: JoinHandle<A>,
) -> anyhow::Result<()> {
    let mut cmd = tokio::process::Command::new("chromedriver")
        .arg("--port=4444")
        .process_group(0)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    tokio::select! {
        _= join_handle => {
            tracing::info!("killing webdriver");
            cmd.kill().await.context("error when killing child process")?;
            cancel_token.cancel();
            tracing::info!("webdriver killed");
            Ok(())
        }
        res= cmd.wait() => {
            let val = res.context("child process exited")?;
            tracing::info!("webdriver stopped running, {}",val.code().unwrap_or_default());
            cancel_token.cancel();
            Ok(())
        }
    }?;

    Ok(())
}

async fn shutdown_signal(cancel_token: &CancellationToken) -> anyhow::Result<()> {
    let token = cancel_token.clone();
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::select! {
        _ = token.cancelled() => { },
        _ = ctrl_c => { token.cancel(); }
    }
    Ok(())
}
