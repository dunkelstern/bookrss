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

// cmdline parser
#[macro_use]
extern crate clap;

// app
extern crate lib;
mod import;
mod ffmpeg;

use lib::settings::Settings;
use import::import;

fn main() {
    let matches = clap_app!(bookrss =>
        (version: "1.0")
        (author: "Johannes Schriewer <hallo@dunkelstern.de>")
        (about: "Audiobook RSS builder cli tool")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@subcommand import =>
            (about: "import audio file and read it's metadata into the DB")
            (@arg INPUT: +required "Sets the input file to use")
        )
    ).get_matches();

    let config_file = matches.value_of("CONFIG").unwrap_or("bookrss");

    match Settings::new(config_file) {
        Ok(settings) => {
            if let Some(matches) = matches.subcommand_matches("import") {
                import(settings, matches);
            }
        },
        Err(error) => { println!("Config error: {:?}", error); }
    };
}
