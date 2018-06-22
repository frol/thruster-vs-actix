extern crate actix_web;
use actix_web::{http, server, App, HttpRequest, HttpResponse};

fn plaintext(req: HttpRequest) -> HttpResponse {
    HttpResponse::build_from(&req)
        .header(http::header::SERVER, "Actix")
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body("Hello, World!")
}

fn main() {
    server::new(
        || App::new()
            .resource("/plaintext", |r| r.f(plaintext)))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
