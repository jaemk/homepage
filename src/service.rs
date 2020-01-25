use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time;

use chrono::Local;
use tera::{Context, Tera};

use crate::errors::*;
use crate::{ToHtmlResponse, ToJsonResponse, ToTextResponse, CONFIG};

/// Initialize things
/// - env logger
/// - template engine
/// - server
/// - handle errors
pub fn start(host: &str, port: u16) -> Result<()> {
    CONFIG.check()?;

    // Set a custom logging format & change the env-var to "LOG"
    // e.g. LOG=info chatbot serve
    use std::io::Write;
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - [{}] -> {}",
                Local::now().format("%Y-%m-%d_%H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unknown>"),
                record.args()
            )
        })
        .parse(&env::var("LOG").unwrap_or_default())
        .init();

    let mut tera = Tera::new("templates/**/*.html").expect("unable to compile tera termplates");
    tera.autoescape_on(vec!["html"]);
    let tera = Arc::new(tera);

    let addr = format!("{}:{}", host, port);
    info!("** Listening on {} **", addr);

    rouille::start_server(&addr, move |request| {
        let template = tera.clone();
        let now = Local::now().format("%Y-%m-%d %H:%M%S");
        let log_ok = |req: &rouille::Request, resp: &rouille::Response, elap: time::Duration| {
            let ms = (elap.as_secs() * 1_000) as f32 + (elap.subsec_nanos() as f32 / 1_000_000.);
            info!(
                "[{}] {} {} -> {} ({}ms)",
                now,
                req.method(),
                req.raw_url(),
                resp.status_code,
                ms
            )
        };
        let log_err = |req: &rouille::Request, elap: time::Duration| {
            let ms = (elap.as_secs() * 1_000) as f32 + (elap.subsec_nanos() as f32 / 1_000_000.);
            info!(
                "[{}] Handler Panicked: {} {} ({}ms)",
                now,
                req.method(),
                req.raw_url(),
                ms
            )
        };

        // dispatch and handle errors
        rouille::log_custom(request, log_ok, log_err, move || {
            match route_request(request, template) {
                Ok(resp) => rouille::content_encoding::apply(request, resp),
                Err(e) => {
                    use self::ErrorKind::*;
                    error!("Handler Error: {}", e);
                    match e.kind() {
                        BadRequest(ref s) => {
                            // bad request
                            s.to_string().to_text_resp().with_status_code(400)
                        }
                        DoesNotExist(ref s) => {
                            // bad request
                            s.to_string().to_text_resp().with_status_code(404)
                        }
                        _ => rouille::Response::text("Something went wrong").with_status_code(500),
                    }
                }
            }
        })
    });
}

fn serve_file<T: AsRef<Path>>(path: T) -> Result<rouille::Response> {
    let path = path.as_ref();
    let ext = path
        .extension()
        .and_then(::std::ffi::OsStr::to_str)
        .unwrap_or("");
    let f = fs::File::open(&path).map_err(ErrorKind::FileOpen)?;
    Ok(rouille::Response::from_file(
        rouille::extension_to_mime(ext),
        f,
    ))
}

/// Route the request to appropriate handler
fn route_request(request: &rouille::Request, template: Arc<Tera>) -> Result<rouille::Response> {
    Ok(router!(request,
        (HEAD) ["/"] => { rouille::Response::text("").with_status_code(200) },
        (GET) ["/"] => {
            template.render("home.html", &Context::new())?.to_html_resp()
        },
        (GET) ["/appinfo"] => {
            json!({"version": CONFIG.params["package"]["version"].as_str().unwrap_or("")}).to_json_resp()?
        },
        (GET) ["/status"] => {
            json!({
                "status": "ok",
                "version": CONFIG.params["package"]["version"].as_str().unwrap_or(""),
            }).to_json_resp()?
        },
        (GET) ["/favicon.ico"]  => { serve_file("static/assets/favicon.ico")? },
        (GET) ["/robots.txt"]   => { serve_file("static/robots.txt")? },
        (GET) ["/keybase.txt"]  => { serve_file("static/keybase.txt")? },

        _ => {
            // static files
            if let Some(req) = request.remove_prefix("/static") {
                let static_resp = rouille::match_assets(&req, "static");
                if static_resp.is_success() {
                    return Ok(static_resp)
                }
            }
            bail_fmt!(ErrorKind::DoesNotExist, "nothing here")
        }
    ))
}
