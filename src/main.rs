use std::{env, io};

use actix_files::{Files, NamedFile};
use actix_web::{
    get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use dotenvy::dotenv;

#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.icon")?)
}

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html")))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_port = env::var("APP_PORT").unwrap_or("8000".to_string());

    log::info!("Starting server at http://localhost:{}", app_port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(favicon)
            .service(index)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind((
        "127.0.0.1",
        app_port.parse::<u16>().expect("Invalid port number"),
    ))?
    .workers(2)
    .run()
    .await
}
