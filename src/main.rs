use std::sync::Mutex;
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get, post, web, App, HttpResponse, HttpServer, Responder, Error,
};
use actix_web::middleware::Logger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// This struct represents state
struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index_mutate(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize logging

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(counter.clone()) // <- register the created data
        .service(
            web::scope("mutate")
                .route("", web::get().to(index_mutate))
        )
        .app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web"),
        }))
        .service(
            web::scope("/state")
                .route("", web::get().to(index))
        )
        .service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                // .route("", web::get().to(index))
                .route("/manual-hello", web::get().to(manual_hello)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}