#![feature(plugin)]

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

// Chrono
extern crate chrono;

// app
pub mod models;
