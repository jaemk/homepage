use actix_files::{Files, NamedFile};
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::{web, App};

use tera::{Context, Tera};

use crate::{CONFIG, LOG};

async fn index(
    template: web::Data<tera::Tera>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let s = template
        .render("home.html", &Context::new())
        .map_err(|_| actix_web::error::ErrorInternalServerError("content error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

macro_rules! make_file_serve_fns {
    ($([$name:ident, $path:expr]),* $(,),*) => {
        $(
            async fn $name() -> actix_web::Result<NamedFile> {
                Ok(NamedFile::open($path).map_err(|_| actix_web::error::ErrorInternalServerError("asset not found"))?)
            }
        )*
    };
}

make_file_serve_fns!(
    [favicon, "static/assets/favicon.ico"],
    [robots, "static/robots.txt"],
    [keybase, "static/keybase.txt"],
);

async fn status() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": CONFIG.version,
    })))
}

async fn p404() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::NotFound().body("nothing here"))
}

pub async fn start() -> anyhow::Result<()> {
    CONFIG.ensure_loaded()?;

    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    slog::info!(LOG, "** Listening on {} **", addr);

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").expect("unable to compile tera termplates");

        App::new()
            .data(tera)
            .wrap(crate::logger::Logger::new())
            // root and static files
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::head().to(|| HttpResponse::Ok().header("x-head", "less").finish())),
            )
            .service(Files::new("/static", "static"))
            // status
            .service(web::resource("/status").route(web::get().to(status)))
            // special resources
            .service(web::resource("/favicon.ico").route(web::get().to(favicon)))
            .service(web::resource("/robots.txt").route(web::get().to(robots)))
            .service(web::resource("/keybase.txt").route(web::get().to(keybase)))
            // 404s
            .default_service(web::resource("").route(web::get().to(p404)))
    })
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}
