#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
#[macro_use] extern crate rouille;
#[macro_use] extern crate tera;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate chrono;

#[macro_use] mod macros;
mod errors;
mod service;

use std::env;

use clap::{App, Arg, SubCommand};

use errors::*;


static APPNAME: &'static str = "HomePage";


/// Trait for constructing `rouille::Response`s from other types
pub trait ToResponse {
    fn to_html_resp(&self) -> rouille::Response;
    fn to_text_resp(&self) -> rouille::Response;
}
impl ToResponse for String {
    fn to_html_resp(&self) -> rouille::Response {
        rouille::Response::html(self.as_str())
    }
    fn to_text_resp(&self) -> rouille::Response {
        rouille::Response::text(self.as_str())
    }
}


fn run() -> Result<()> {
    let matches = App::new(APPNAME)
        .version(crate_version!())
        .about("Homepage Sever")
        .subcommand(SubCommand::with_name("serve")
            .about("Initialize Server")
            .arg(Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("3002")
                .help("Port to listen on."))
            .arg(Arg::with_name("public")
                .long("public")
                .help("Serve on '0.0.0.0' instead of 'localhost'"))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Output debug logging info. Shortcut for setting env-var LOG=debug")))
        .get_matches();

    match matches.subcommand() {
        ("serve", Some(serve_matches)) => {
            env::set_var("LOG", "info");
            if serve_matches.is_present("debug") { env::set_var("LOG", "debug"); }
            let port = serve_matches.value_of("port")
                .expect("default port should be set by clap")
                .parse::<u16>()
                .chain_err(|| "`--port` expects an integer")?;
            let host = if serve_matches.is_present("public") { "0.0.0.0" } else { "localhost" };
            service::start(&host, port)?;
        }
        _ => {
            eprintln!("{}: see `--help`", APPNAME);
        }
    }
    Ok(())
}


quick_main!(run);

