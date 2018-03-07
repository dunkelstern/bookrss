use std::str::FromStr;

use rocket::{ Rocket, custom };
use rocket::config;
use rocket::config::{Environment, Limits, LoggingLevel, Result};

use lib::database::init_pool;

use views::*;
use lib::settings::Settings;
use config::Config;

pub fn rocket(config: Settings) -> Result<Rocket> {
    let limits = Limits::new()
        .limit("forms", config.server.limits.forms);

    let conf = config::Config::build(Environment::Production)
        .workers(config.server.workers)
        .log_level(LoggingLevel::from_str(&config.server.log).unwrap_or(LoggingLevel::Normal))
        .address(config.server.address.clone())
        .port(config.server.port)
        .secret_key(config.server.secret_key.clone())
        .limits(limits)
        .extra("template_dir", config.server.template_dir.clone())
        .finalize()?;

    Ok(
        custom(conf, true)
        .manage(init_pool(config.database.url.clone()))
        .manage(Config(config))
        .mount("/", routes![
            get_author_list_filtered,
            get_author_list,
            get_author,
            patch_author,
            create_author,
            delete_author,
        ])
        .mount("/", routes![
            get_narrator_list_filtered,
            get_narrator_list,
            get_narrator,
            patch_narrator,
            create_narrator,
            delete_narrator,
        ])
        .mount("/", routes![
            get_series_list_filtered,
            get_series_list,
            get_series,
            patch_series,
            create_series,
            delete_series,
        ])
        .mount("/", routes![
            get_audiobook_list_filtered,
            get_audiobook_list,
            get_audiobook,
            patch_audiobook,
            create_audiobook,
            delete_audiobook,
        ])
        .mount("/", routes![
            get_part_list,
            get_part,
            get_cover,
        ])
        .mount("/", routes![
            get_series_rss,
            get_audiobook_rss,
        ])
    )
}
