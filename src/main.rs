use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Calc {
    num1: i32,
    num2: i32,
}

#[get("/add")]
async fn add(calc: web::Query<Calc>) -> String {
    let number = calc.num1 + calc.num2;
    number.to_string()
}

#[get("/sub")]
async fn sub(calc: web::Query<Calc>) -> String {
    let number = calc.num1 - calc.num2;
    number.to_string()
}

#[get("/mult")]
async fn mult(calc: web::Query<Calc>) -> String {
    let number = calc.num1 * calc.num2;
    number.to_string()
}

#[get("/div")]
async fn div(calc: web::Query<Calc>) -> String {
    if calc.num2 == 0 {
        return format!("CANNOT DIVIDE BY 0!!!!!");
    }
    let number = calc.num1 / calc.num2;
    number.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(sub)
            .service(mult)
            .service(div)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
