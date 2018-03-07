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
extern crate serde_json;

// Diesel
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
pub mod database;

// RSS
extern crate rss;

// Chrono
extern crate chrono;

// app
#[macro_use]
extern crate lib;
pub mod views;
mod config;

use lib::settings::Settings;

fn main() {
    match Settings::new("bookrss") {
        Ok(settings) => { 
            match rocket(settings) {
                Ok(rocket) => { rocket.launch(); },
                Err(error) => { println!("Server config error: {:?}", error); },
            };
        },
        Err(error) => { println!("Config error: {:?}", error); }
    };
}
