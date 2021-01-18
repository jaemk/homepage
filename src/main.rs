mod logger;
mod service;

use std::env;
use std::fs;
use std::io::Read;

use slog::{o, Drain};

fn env_or(k: &str, default: &str) -> String {
    env::var(k).unwrap_or_else(|_| default.to_string())
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::load();

    // The "base" logger that all crates should branch off of
    pub static ref BASE_LOG: slog::Logger = {
        if CONFIG.log_format == "pretty" {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::CompactFormat::new(decorator).build().fuse();
            let drain = slog_async::Async::new(drain).build().fuse();
            let drain = slog::LevelFilter::new(drain, slog::Level::Debug).fuse();
            slog::Logger::root(drain, o!())
        } else {
            let drain = slog_json::Json::default(std::io::stderr()).fuse();
            let drain = slog_async::Async::new(drain).build().fuse();
            let drain = slog::LevelFilter::new(drain, slog::Level::Info).fuse();
            slog::Logger::root(drain, o!())
        }
    };

    // Base logger
    pub static ref LOG: slog::Logger = BASE_LOG.new(slog::o!("app" => "homepage"));
}

pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
    pub log_format: String,
}
impl Config {
    pub fn load() -> Self {
        let version = fs::File::open("commit_hash.txt")
            .map(|mut f| {
                let mut s = String::new();
                f.read_to_string(&mut s).expect("Error reading commit_hash");
                s
            })
            .unwrap_or_else(|_| "unknown".to_string());

        Self {
            version,
            host: env_or("HOST", "0.0.0.0"),
            port: env_or("PORT", "5000").parse().expect("invalid port"),
            log_format: env_or("LOG_FORMAT", "json")
                .to_lowercase()
                .trim()
                .to_string(),
        }
    }

    pub fn ensure_loaded(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

async fn run() -> anyhow::Result<()> {
    slog::info!(
        LOG, "initializing";
        "version" => &CONFIG.version,
        "host" => &CONFIG.host,
        "port" => &CONFIG.port,
        "log_format" => &CONFIG.log_format,
    );
    service::start().await?;
    Ok(())
}

#[actix_web::main]
pub async fn main() {
    if let Err(e) = run().await {
        slog::error!(LOG, "Error: {:?}", e);
    }
}
