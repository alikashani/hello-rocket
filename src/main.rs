#[macro_use] extern crate rocket;

use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

use rocket::serde::json::Json;
use serde::Serialize;

mod util;

#[derive(Serialize)]
struct NumberResponse {
    number: u64,
    is_prime_number: bool,
    execution_time_in_micros: u128
}

#[get("/")]
fn index() -> &'static str {
    "This is my Rust prime number REST API"
}

#[get("/is-prime?<number>")]
fn is_prime(number: u64) -> Json<NumberResponse> {
    let now = SystemTime::now();

    Json(NumberResponse {
        number,
        is_prime_number: util::is_prime(number),
        execution_time_in_micros: now.elapsed().unwrap().as_micros(),
    })
}

#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

#[rocket::main]
async fn main() {
    let mut config = rocket::config::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let _ = rocket::build()
        .configure(config)
        .mount("/", routes![index, health, is_prime])
        .launch()
        .await;
}