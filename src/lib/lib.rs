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
