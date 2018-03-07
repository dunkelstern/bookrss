#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

// Serde
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Rocket
extern crate rocket;
extern crate rocket_contrib;

// Diesel
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;

// Chrono
extern crate chrono;

// config
extern crate config;
extern crate shellexpand;

// slugify
extern crate slug;

// app
pub mod models;
pub mod settings;
pub mod database;

#[macro_use]
pub mod macros;
