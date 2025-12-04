use std::env;
use axum::Router;
use axum::routing::{get, post};
use clap::Parser;
use log::info;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;
use crate::database::init::init;
use crate::error::AppError;
use crate::handlers::blocklist::{add_ip, get_ip};

pub mod database;
pub mod error;
pub mod pool;
pub mod handlers;
pub mod templates;
pub mod forms;

#[derive(Debug, Parser, Default)]
struct PreCli {
    /// Optional `.env` file path for loading environment variables.
    #[clap(short, long, value_name = "ENV_FILE")]
    env_file: Option<String>,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional `.env` file path for loading environment variables.
    #[clap(short, long, value_name = "ENV_FILE")]
    env_file: Option<String>,
    /// Optional log level.
    #[clap(
        long,
        value_name = "APP_LOG_LEVEL",
        env = "APP_LOG_LEVEL",
        default_value = "info"
    )]
    app_log_level: String,

    /// Optional log level for all included components, such as pingora.
    #[clap(
        long,
        value_name = "ALL_LOG_LEVEL",
        env = "ALL_LOG_LEVEL",
        default_value = ""
    )]
    all_log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let pre = PreCli::try_parse().unwrap_or_default();
    if let Some(env_file) = pre.env_file {
        dotenvy::from_filename(env_file).expect("failed to load .env file");
    } else {
        dotenvy::dotenv().ok();
    }

    let cli = Cli::parse();
    let env =
        EnvFilter::new(format!("blocklist={},{}", cli.app_log_level, cli.all_log_level).as_str());
    let timer = tracing_subscriber::fmt::time::LocalTime::rfc_3339();
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_target(true)
        .with_env_filter(env)
        .init();

    let pool = init().await?;
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/blocklist", get(get_ip))
        .route("/blocklist", post(add_ip))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(parse_host())
        .await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn parse_host() -> String {
    let hostname = env::var("BLOCKLIST_HOSTNAME").unwrap_or("localhost".to_string());
    let port = env::var("BLOCKLIST_PORT").unwrap_or("6060".to_string());
    format!("{hostname}:{port}")
}
