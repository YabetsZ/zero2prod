use std::net::TcpListener;

use actix_web;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:0")?;
    run(listener)?.await
}
