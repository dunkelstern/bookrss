use std::ops::Deref;

use rocket::{ Rocket, ignite };
use rocket::fairing::AdHoc;
use database::DbMiddleware;

use views::*;

pub struct DataPath(pub String);

impl Deref for DataPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn rocket() -> Rocket {
    ignite()
        .attach(DbMiddleware)
        .attach(AdHoc::on_attach(|rocket| {
            let data_path = rocket.config().get_str("data_path").unwrap().to_string();
            Ok(rocket.manage(DataPath(data_path)))            
        }))
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
        ])
}
