use std::{
    error::Error,
    net::TcpListener,
    path::{Path, PathBuf},
};

use actix_web::{rt, web, App, HttpServer};
use clap::{CommandFactory, Parser, Subcommand};

use crate::{api::configurator, cfg::Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run() -> Result<(), Box<dyn Error>> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Run { config }) => start_server(config),
            None => {
                Cli::command().print_help().ok();
                Ok(())
            }
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the application
    Run {
        /// Path to configuration file in .toml format
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,
    },
}

pub fn start_server(config_file: &Path) -> Result<(), Box<dyn Error>> {
    let config = Config::read_from_file(config_file)?;
    let server_config = config.server;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.server.listen_address, config.server.port
    ))?;

    rt::System::new().block_on(async move {
        let task_processor = web::Data::new(configurator::task_processor(config));
        let mut server = HttpServer::new(move || {
            App::new()
                .app_data(task_processor.clone())
                .configure(configurator::app_routes_configurator)
        })
        .listen(listener)?;

        if let Some(workers) = server_config.workers {
            server = server.workers(workers);
        }

        server.run().await
    })?;

    Ok(())
}
