#![feature(plugin)]

// Rocket
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
extern crate rocket;
extern crate rocket_contrib;
pub mod server;
use self::server::rocket;

// Serde
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Diesel
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
pub mod database;

// RSS
extern crate rss;

// Chrono
extern crate chrono;

// app
pub mod models;
pub mod views;

fn main() {
    rocket().launch();
}
