//! Example code for using MongoDB with Actix.

mod model;

use actix_web::{get, web, App, HttpResponse, HttpServer};
extern crate redis;
use redis::Commands;

fn do_something() -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    let res: String = con.get("my_key")?;
    Ok(res)
}

#[get("/get_user/{username}")]
async fn get_user(username: web::Path<String>) -> HttpResponse {
    println!("{}", username);
    let red = do_something().unwrap();
    HttpResponse::Ok().json(("lorem", red))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(get_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
