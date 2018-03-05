use std::ops::Deref;
use std::str::FromStr;

use rocket::{ Rocket, custom };
use rocket::config::{Config, Environment, Limits, LoggingLevel, Result};
use lib::database::init_pool;

use views::*;
use lib::settings::Settings;

pub struct DataPath(pub String);

impl Deref for DataPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn rocket(config: Settings) -> Result<Rocket> {
    let limits = Limits::new()
        .limit("forms", config.server.limits.forms);

    let conf = Config::build(Environment::Production)
        .workers(config.server.workers)
        .log_level(LoggingLevel::from_str(&config.server.log).unwrap_or(LoggingLevel::Normal))
        .address(config.server.address)
        .port(config.server.port)
        .secret_key(config.server.secret_key)
        .limits(limits)
        .extra("template_dir", config.server.template_dir)
        .finalize()?;

    Ok(
        custom(conf, true)
        .manage(init_pool(config.database.url))
        .manage(DataPath(config.path.data_path))
        .mount("/", routes![
            get_audiobook_list_filtered,
            get_audiobook_list,
            get_audiobook,
            get_series_list_filtered,
            get_series_list,
            get_series,
            get_author_list_filtered,
            get_author_list,
            get_author,
            get_speaker_list_filtered,
            get_speaker_list,
            get_speaker,
            get_part_list,
            get_part,
            get_series_rss,
        ])
    )
}
