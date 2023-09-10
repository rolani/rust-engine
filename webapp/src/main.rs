// web microservice for multiplying integers

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a calculator microservice!")
}

#[get("/mult/{n1}/{n2}")]
async fn mult(info: web::Path<(i32, i32)>) -> impl Responder {
    let mult_ = webapp::mult(info.0, info.1);
    HttpResponse::Ok().body(format!("{} * {} = {}", info.0, info.1, mult_))
}

#[get("/add/{n1}/{n2}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let add_ = webapp::add(info.0, info.1);
    HttpResponse::Ok().body(format!("{} + {} = {}", info.0, info.1, add_))
}

#[get("/sub/{n1}/{n2}")]
async fn sub(info: web::Path<(i32, i32)>) -> impl Responder {
    let sub_ = webapp::sub(info.0, info.1);
    HttpResponse::Ok().body(format!("{} - {} = {}", info.0, info.1, sub_))
}

#[get("/div/{n1}/{n2}")]
async fn div(info: web::Path<(i32, i32)>) -> impl Responder {
    let div_ = webapp::div(info.0, info.1);
    HttpResponse::Ok().body(format!("{} / {} = {}", info.0, info.1, div_))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(mult)
            .service(add)
            .service(sub)
            .service(div)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
