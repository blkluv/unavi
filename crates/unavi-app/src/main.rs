#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, ValueEnum};
use surrealdb::Surreal;
use tracing::Level;
use unavi_app::StartOptions;

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let search = location.search().unwrap();
    let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();

    let mut args = Args {
        debug_physics: false,
        log_level: LogLevel::default(),
    };

    if let Some(value) = params.get("debug-physics") {
        if let Ok(value) = value.parse() {
            args.debug_physics = value;
        }
    }

    if let Some(value) = params.get("log-level") {
        match value.as_str() {
            "info" => args.log_level = LogLevel::Info,
            "debug" => args.log_level = LogLevel::Debug,
            "trace" => args.log_level = LogLevel::Trace,
            _ => tracing::warn!("Unknown log-level: {}", value),
        }
    }

    let db = Surreal::new::<surrealdb::engine::local::IndxDb>("unavi")
        .await
        .expect("Failed to create SurrealDB.");

    let opts = args_to_options(args);

    unavi_app::start(db, opts).await;
}

#[cfg(target_family = "wasm")]
fn main() {}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Enables physics debug mode.
    #[arg(long)]
    debug_physics: bool,
    /// Sets the log level.
    #[arg(long, default_value_t, value_enum)]
    log_level: LogLevel,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() {
    use directories::ProjectDirs;
    use surrealdb::engine::local::SurrealKV;

    let args = Args::parse();

    let dirs = ProjectDirs::from("xyz", "unavi", "unavi-app").expect("Failed to get project dirs.");
    let db_path = dirs.data_dir();

    std::fs::create_dir_all(db_path).expect("Failed to create database dir.");

    let db = Surreal::new::<SurrealKV>(db_path)
        .await
        .expect("Failed to create SurrealDB.");

    let opts = args_to_options(args);
    unavi_app::start(db, opts).await;
}

fn args_to_options(args: Args) -> StartOptions {
    let log_level = match args.log_level {
        LogLevel::Info => Level::INFO,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Trace => Level::TRACE,
    };

    StartOptions {
        debug_physics: args.debug_physics,
        log_level,
    }
}
