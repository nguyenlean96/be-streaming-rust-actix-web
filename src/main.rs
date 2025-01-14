use std::{env, io};

use actix_files::{Files, NamedFile};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware::{self, Logger},
    web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use dotenvy::dotenv;

#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.icon")?)
}

#[get("/")]
async fn index(_req: HttpRequest, _session: Session) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../static/index.html")))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);

            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let key =
        actix_web::cookie::Key::from(env::var("APP_KEY").expect("APP_KEY must be set").as_bytes());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_port = env::var("APP_PORT").unwrap_or("8000".to_string());

    log::info!("Starting server at http://localhost:{}", app_port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
            .service(favicon)
            .service(index)
            .service(Files::new("/static", "static").show_files_listing())
            // Redirect to index.html for all other routes
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");

                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "static/index.html"))
                        .finish()
                })),
            )
            .default_service(web::to(default_handler))
    })
    .bind((
        "127.0.0.1",
        app_port.parse::<u16>().expect("Invalid port number"),
    ))?
    .workers(2)
    .run()
    .await
}
