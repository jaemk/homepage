#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate rouille;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate env_logger;
extern crate serde;
extern crate toml;

#[macro_use]
mod macros;
mod errors;
mod service;

use std::env;
use std::fs;
use std::io::Read;

use clap::{App, Arg, SubCommand};

use errors::*;

static APPNAME: &'static str = "HomePage";

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Deserialize)]
pub struct Config {
    pub params: toml::Value,
}
impl Config {
    pub fn load() -> Self {
        let mut f = fs::File::open("Cargo.toml").expect("Failed opening Cargo.toml");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Error reading Cargo.toml");
        let params: toml::Value = toml::from_str(&s).expect("Failed parsing Cargo.toml");
        Self { params }
    }
    pub fn check(&self) -> Result<()> {
        Ok(())
    }
}

// ---------------
// Traits for constructing `rouille::Response`s from other types
// ---------------

pub trait ToHtmlResponse {
    fn to_html_resp(&self) -> rouille::Response;
}

pub trait ToTextResponse {
    fn to_text_resp(&self) -> rouille::Response;
}

pub trait ToJsonResponse {
    fn to_json_resp(&self) -> Result<rouille::Response>;
}

impl ToHtmlResponse for String {
    fn to_html_resp(&self) -> rouille::Response {
        rouille::Response::html(self.as_str())
    }
}
impl ToTextResponse for String {
    fn to_text_resp(&self) -> rouille::Response {
        rouille::Response::text(self.as_str())
    }
}

impl ToJsonResponse for serde_json::Value {
    fn to_json_resp(&self) -> Result<rouille::Response> {
        let s = serde_json::to_string(self)?;
        let resp = rouille::Response::from_data("application/json", s.as_bytes());
        Ok(resp)
    }
}

fn run() -> Result<()> {
    let matches = App::new(APPNAME)
        .version(crate_version!())
        .about("Homepage Sever")
        .subcommand(
            SubCommand::with_name("serve")
                .about("Initialize Server")
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .short("p")
                        .takes_value(true)
                        .default_value("3002")
                        .help("Port to listen on."),
                )
                .arg(
                    Arg::with_name("public")
                        .long("public")
                        .help("Serve on '0.0.0.0' instead of 'localhost'"),
                )
                .arg(
                    Arg::with_name("debug")
                        .long("debug")
                        .help("Output debug logging info. Shortcut for setting env-var LOG=debug"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("serve", Some(serve_matches)) => {
            env::set_var("LOG", "info");
            if serve_matches.is_present("debug") {
                env::set_var("LOG", "debug");
            }
            let port = serve_matches
                .value_of("port")
                .expect("default port should be set by clap")
                .parse::<u16>()
                .chain_err(|| "`--port` expects an integer")?;
            let host = if serve_matches.is_present("public") {
                "0.0.0.0"
            } else {
                "localhost"
            };
            service::start(&host, port)?;
        }
        _ => {
            eprintln!("{}: see `--help`", APPNAME);
        }
    }
    Ok(())
}

quick_main!(run);
